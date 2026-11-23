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

.PHONY: dev build build-tui build-windows install test release mc mc-linux mc-windows mc-jar mc-test

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
# XWIN_ARGS: extra cargo arguments (the mod's build passes its features and target dir).
XWIN_BIN := $(CURDIR)/target/xwin-bin
LLVM_TOOLS := $(shell rustc --print sysroot)/lib/rustlib/$(shell rustc -vV | sed -n 's/^host: //p')/bin
XWIN_ARGS ?=
build-windows:
	@mkdir -p $(XWIN_BIN)
	@ln -sf $(LLVM_TOOLS)/llvm-ar $(XWIN_BIN)/llvm-lib
	@ln -sf $(LLVM_TOOLS)/llvm-ar $(XWIN_BIN)/llvm-ar
	@ln -sf $(LLVM_TOOLS)/rust-lld $(XWIN_BIN)/lld-link
	env -u LK_CUSTOM_WEBRTC -u CC -u CXX PATH="$(XWIN_BIN):$$PATH" cargo xwin build --release --target x86_64-pc-windows-msvc $(XWIN_ARGS)

# Also refreshes the Minecraft mod when there is a Minecraft with a mods folder (MINECRAFT, default ~/.minecraft): the
# Linux binary is rebuilt, the Windows one is reused when built before (else the jar goes without it).
MINECRAFT ?= $(HOME)/.minecraft
install:
	cargo install --path . --locked --force
	@if [ -d "$(MINECRAFT)/mods" ]; then \
		echo "$(MINECRAFT)/mods found: updating the soworker mod"; \
		$(MAKE) --no-print-directory mc-linux && \
		$(MAKE) --no-print-directory mc-jar MC_ALLOW_MISSING=1 MC_JAR_ARGS="--install --minecraft $(MINECRAFT)"; \
	else \
		echo "no $(MINECRAFT)/mods: skipping the Minecraft mod"; \
	fi

test:
	cargo test

# The Minecraft mod (mc/) carries its own soworker: the bridge only (`mc` feature, no windows, no terminal client),
# for Linux and Windows, stripped (release profile), in a target dir of its own. The jar is built with javac against the installed game
# (no Gradle) and copied into ~/.minecraft/mods. `make mc MC_ALLOW_MISSING=1` builds a jar without the Windows binary
# when that build fails.
MC_FEATURES := --no-default-features --features mc,calls
MC_LINUX := target/mc/release/soworker
MC_WINDOWS := target/mc/x86_64-pc-windows-msvc/release/soworker.exe
mc-linux:
	cargo build --release $(MC_FEATURES) --target-dir target/mc
	@# the oldest glibc the binary starts on is the newest symbol version it needs
	@echo "$(MC_LINUX) needs glibc $$(objdump -T $(MC_LINUX) | grep -o 'GLIBC_[0-9.]*' | sort -uV | tail -1 | sed 's/GLIBC_//')"

mc-windows:
	$(MAKE) --no-print-directory build-windows XWIN_ARGS="$(MC_FEATURES) --target-dir target/mc"

mc-jar:
	python3 mc/build.py --bin linux=$(MC_LINUX) $(if $(wildcard $(MC_WINDOWS)),--bin windows=$(MC_WINDOWS)) $(if $(MC_ALLOW_MISSING),--allow-missing) $(MC_JAR_ARGS)

mc: mc-linux
	@$(MAKE) --no-print-directory mc-windows || { [ -n "$(MC_ALLOW_MISSING)" ] && rm -f $(MC_WINDOWS); } || { echo "Windows build failed: MC_ALLOW_MISSING=1 builds the jar without it"; exit 1; }
	@$(MAKE) --no-print-directory mc-jar MC_JAR_ARGS=--install

# The tests of the mod's binary (the `mc` feature is not in the default build).
mc-test:
	cargo test $(MC_FEATURES) --target-dir target/mc

# Bump the version in Cargo.toml (BUMP=patch|minor|major), build, commit the bump, tag it and publish a GitHub release
# with the Linux and Windows builds and the Minecraft mod jar (which carries the mod's own Linux and Windows binaries).
# If a Windows build fails, it asks whether to release without Windows: the jar then has no Windows binary and the
# release notes say so. A failing Linux mod build or jar fails the release (same undo).
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
		if [ $$pushed -eq 1 ]; then echo "pushed $$tag but the GitHub release failed: run: gh release create $$tag $$asset $$win $$jar --generate-notes"; return; fi; \
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
	$(MAKE) --no-print-directory mc-linux; \
	rm -f $(MC_WINDOWS); \
	if $(MAKE) --no-print-directory build-windows && (cd target/x86_64-pc-windows-msvc/release && zip -q "$$winzip" soworker.exe) && $(MAKE) --no-print-directory mc-windows; then \
		win="$$winzip"; \
	else \
		read -r -p "Windows build failed (see build-windows for the cargo-xwin setup). Release without it? [y/N] " ans; \
		case "$$ans" in [yY]*) ;; *) exit 1;; esac; \
		win=""; rm -f "$$winzip" $(MC_WINDOWS); \
	fi; \
	if [ -n "$$win" ]; then $(MAKE) --no-print-directory mc-jar; else $(MAKE) --no-print-directory mc-jar MC_ALLOW_MISSING=1; fi; \
	jar="mc/build/soworker-mc-$$new.jar"; \
	test -f "$$jar"; \
	notes=""; \
	if [ -z "$$win" ]; then notes="No Windows build in this release: soworker-mc-$$new.jar carries the Linux binary only, so the Minecraft mod works on Linux only."; fi; \
	git add Cargo.toml Cargo.lock; \
	git commit -q -m "Release $$tag"; committed=1; \
	git tag "$$tag"; tagged=1; \
	git push origin HEAD "$$tag"; pushed=1; \
	gh release create "$$tag" "$$asset" $$win "$$jar" --title "$$tag" --generate-notes $${notes:+--notes "$$notes"}

# Android (docs/android-plan.md): the native library for arm64 phones, then the APK (Gradle, android/). Needs the NDK
# (ANDROID_NDK_HOME, default the newest under ~/Android/Sdk/ndk) and the Android libwebrtc prebuilt in
# third_party/libwebrtc-android-arm64: github.com/livekit/rust-sdks release webrtc-89d790b,
# webrtc-android-arm64-release.zip, sha256 81880a4cda27474497ac5277e841ae110ede73e81d0e767606de4bdace8ac82c.
ANDROID_HOME ?= $(HOME)/Android/Sdk
ANDROID_NDK_HOME ?= $(lastword $(sort $(wildcard $(ANDROID_HOME)/ndk/*)))
ANDROID_API := 26
# debug builds much faster (no fat LTO); the copy in the APK is stripped either way
ANDROID_PROFILE ?= debug
ANDROID_TC := $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64/bin
ANDROID_PKG := dev.soworker.app
ANDROID_LIB = target/aarch64-linux-android/$(ANDROID_PROFILE)/libsoworker_android.so
ANDROID_ENV := env -u LK_CUSTOM_WEBRTC -u CC -u CXX \
	LK_CUSTOM_WEBRTC=$(CURDIR)/third_party/libwebrtc-android-arm64 \
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) ANDROID_HOME=$(ANDROID_HOME) \
	CC_aarch64_linux_android=$(ANDROID_TC)/aarch64-linux-android$(ANDROID_API)-clang \
	CXX_aarch64_linux_android=$(ANDROID_TC)/aarch64-linux-android$(ANDROID_API)-clang++ \
	AR_aarch64_linux_android=$(ANDROID_TC)/llvm-ar \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(ANDROID_TC)/aarch64-linux-android$(ANDROID_API)-clang \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS="-C link-arg=-Wl,-z,max-page-size=16384"

.PHONY: android-lib android android-run android-release
android-lib:
	$(ANDROID_ENV) cargo build -p soworker-android --target aarch64-linux-android $(if $(filter release,$(ANDROID_PROFILE)),--release)
	@mkdir -p android/app/src/main/jniLibs/arm64-v8a android/app/libs
	$(ANDROID_TC)/llvm-strip --strip-debug -o android/app/src/main/jniLibs/arm64-v8a/libsoworker_android.so $(ANDROID_LIB)
	cp third_party/libwebrtc-android-arm64/libwebrtc.jar android/app/libs/

android: android-lib
	cd android && ANDROID_HOME=$(ANDROID_HOME) ./gradlew assembleDebug

# install, start, and follow the app's log (stderr goes to logcat under the tag "soworker")
android-run: android
	adb install -r android/app/build/outputs/apk/debug/app-debug.apk
	adb shell am start -n $(ANDROID_PKG)/.MainActivity
	adb logcat -v color --pid=$$(sleep 1; adb shell pidof -s $(ANDROID_PKG))

# a signed APK next to the desktop builds: needs SOWORKER_KEYSTORE, SOWORKER_KEYSTORE_PASS, SOWORKER_KEY_ALIAS,
# SOWORKER_KEY_PASS (keep the keystore outside the repo and back it up: every update must be signed with it)
android-release:
	$(MAKE) --no-print-directory android-lib ANDROID_PROFILE=release
	cd android && ANDROID_HOME=$(ANDROID_HOME) ./gradlew assembleRelease
	cp android/app/build/outputs/apk/release/app-release.apk target/soworker-$$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)-android-arm64.apk
