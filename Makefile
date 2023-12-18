docs-web:
	make -C docs/rustdoc_web

test: unit-test integration-test

unit-test:
	cargo test

integration-test:
	cd tests/default && cargo test
	cd tests/with_serde && cargo test

.PHONY: unit-test integration-test
