MAKEFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
EXT_CSS_PATH := $(MAKEFILE_DIR)/style.css

CARGO_OPTS := \
	--lib \
	--workspace

doc-embedded:
	cargo doc $(CARGO_OPTS) --config "build.rustdocflags = ['--extend-css', '$(EXT_CSS_PATH)']"
	make copy

doc:
	cargo doc $(CARGO_OPTS)
	make copy

copy:
	cp -r ../../../../target/doc ./rustdoc

.PHONY: doc doc-embedded copy
