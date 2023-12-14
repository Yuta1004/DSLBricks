test: unit-test integration-test

unit-test:
	cargo test

integration-test:
	cd tests/default && cargo test
	cd tests/with_serde && cargo test

setup:
	make -C src/editor setup

.PHONY: unit-test integration-test
