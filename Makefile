# The LiveKit WebRTC build needs clang.
SHELL := /bin/bash
export CXX = clang++
export CC = clang

# release bumps this part of the version: patch (default), minor or major
BUMP ?= patch

.PHONY: dev build build-gui install test release

# Rebuild and restart the app whenever a source file changes (needs cargo-watch).
dev:
	cargo watch -w src -w Cargo.toml -x run

build:
	cargo build --release

# the terminal client plus the window client (soworker --gui)
build-gui:
	cargo build --release --features gui

install:
	cargo install --path . --locked --force

test:
	cargo test

# Bump the version in Cargo.toml (BUMP=patch|minor|major), build, commit the bump, tag it and publish a GitHub release.
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
		if [ $$pushed -eq 1 ]; then echo "pushed $$tag but the GitHub release failed: run: gh release create $$tag $$asset --generate-notes"; return; fi; \
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
	git add Cargo.toml Cargo.lock; \
	git commit -q -m "Release $$tag"; committed=1; \
	git tag "$$tag"; tagged=1; \
	git push origin HEAD "$$tag"; pushed=1; \
	gh release create "$$tag" "$$asset" --title "$$tag" --generate-notes
