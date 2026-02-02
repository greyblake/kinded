all: fmt test-all clippy examples typos

test-all: test test-doc

test:
	cargo test --workspace

test-doc:
	cd kinded && cargo test --doc
	cd kinded_macros && cargo test --doc

fmt:
	cargo fmt

clippy:
	cargo clippy --workspace -- -D warnings

examples:
	#!/usr/bin/env bash
	set -euxo pipefail
	ROOT_DIR=$(pwd)
	for EXAMPLE in $(ls examples); do
		cd "$ROOT_DIR/examples/$EXAMPLE"
		cargo run
	done

watch:
	cargo watch -x 'test --workspace'

watch-sandbox:
	cargo watch -s "cd sandbox && cargo run"

typos:
	which typos >/dev/null || cargo install typos-cli
	typos
