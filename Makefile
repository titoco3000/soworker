# The LiveKit WebRTC build needs clang.
SHELL := /bin/bash
export CXX = clang++
export CC = clang
# Use a local libwebrtc prebuilt instead of downloading it (156 MB, no progress output) during the build, when there is one.
# Source: github.com/livekit/rust-sdks release webrtc-89d790b, webrtc-linux-x64-release.zip,
# sha256 b167adad5291cea0e4d66a0454d9d52d2ad714e6b0ed70f4410317d3ebde70c5.
ifneq ($(wildcard third_party/libwebrtc),)
export LK_CUSTOM_WEBRTC = $(CURDIR)/third_party/libwebrtc
endif

# release bumps this part of the version: patch (default), minor or major
BUMP ?= patch

.PHONY: dev build build-tui build-windows install test release

# Rebuild and restart the app whenever a source file changes (needs cargo-watch).
dev:
	cargo watch -w src -w Cargo.toml -x run

build:
	cargo build --release

# the terminal client only: no window code, no pictures, smaller
build-tui:
	cargo build --release --no-default-features --features tui,calls,portal

# Cross-compile the Windows binary (target/x86_64-pc-windows-msvc/release/soworker.exe) from Linux with cargo-xwin, which
# fetches the MSVC headers and libraries (accepting Microsoft's license). One-time setup:
#   rustup target add x86_64-pc-windows-msvc && rustup component add llvm-tools && cargo install cargo-xwin --locked
# The rustup llvm-tools have no `llvm-lib`; `llvm-ar` does the same job under that name.
XWIN_BIN := $(CURDIR)/target/xwin-bin
LLVM_TOOLS := $(shell rustc --print sysroot)/lib/rustlib/$(shell rustc -vV | sed -n 's/^host: //p')/bin
build-windows:
	@mkdir -p $(XWIN_BIN)
	@ln -sf $(LLVM_TOOLS)/llvm-ar $(XWIN_BIN)/llvm-lib
	@ln -sf $(LLVM_TOOLS)/llvm-ar $(XWIN_BIN)/llvm-ar
	@ln -sf $(LLVM_TOOLS)/rust-lld $(XWIN_BIN)/lld-link
	env -u LK_CUSTOM_WEBRTC -u CC -u CXX PATH="$(XWIN_BIN):$$PATH" cargo xwin build --release --target x86_64-pc-windows-msvc

install:
	cargo install --path . --locked --force

test:
	cargo test

# Bump the version in Cargo.toml (BUMP=patch|minor|major), build, commit the bump, tag it and publish a GitHub release
# with the Linux and Windows builds. If the Windows build fails, it asks whether to release without it.
release:
	@test -z "$$(git status --porcelain)" || { echo "commit your changes first"; exit 1; }
	@set -e; \
	old=$$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1); \
	IFS=. read -r ma mi pa <<< "$$old"; \
	case "$(BUMP)" in major) ma=$$((ma+1)); mi=0; pa=0;; minor) mi=$$((mi+1)); pa=0;; patch) pa=$$((pa+1));; *) echo "BUMP must be patch, minor or major"; exit 1;; esac; \
	new="$$ma.$$mi.$$pa"; tag="v$$new"; \
	if git rev-parse -q --verify "refs/tags/$$tag" >/dev/null; then echo "tag $$tag exists"; exit 1; fi; \
	echo "releasing $$old -> $$new"; \
	committed=0; tagged=0; pushed=0; \
	undo() { \
		[ $$? -eq 0 ] && return; \
		if [ $$pushed -eq 1 ]; then echo "pushed $$tag but the GitHub release failed: run: gh release create $$tag $$asset $$win --generate-notes"; return; fi; \
		echo "release failed: undoing the version bump"; \
		[ $$tagged -eq 1 ] && git tag -d "$$tag" >/dev/null; \
		[ $$committed -eq 1 ] && git reset -q --hard HEAD~1; \
		git checkout -q -- Cargo.toml Cargo.lock; \
	}; \
	trap undo EXIT; \
	sed -i "0,/^version = \"$$old\"/s//version = \"$$new\"/" Cargo.toml; \
	cargo build --release; \
	asset="target/release/soworker-$$tag-linux-x86_64.tar.gz"; \
	tar -czf "$$asset" -C target/release soworker; \
	win=""; \
	winzip="$$PWD/target/release/soworker-$$tag-windows-x86_64.zip"; \
	rm -f "$$winzip"; \
	if $(MAKE) --no-print-directory build-windows && (cd target/x86_64-pc-windows-msvc/release && zip -q "$$winzip" soworker.exe); then \
		win="$$winzip"; \
	else \
		read -r -p "Windows build failed (see build-windows for the cargo-xwin setup). Release without it? [y/N] " ans; \
		case "$$ans" in [yY]*) ;; *) exit 1;; esac; \
	fi; \
	git add Cargo.toml Cargo.lock; \
	git commit -q -m "Release $$tag"; committed=1; \
	git tag "$$tag"; tagged=1; \
	git push origin HEAD "$$tag"; pushed=1; \
	gh release create "$$tag" "$$asset" $$win --title "$$tag" --generate-notes
