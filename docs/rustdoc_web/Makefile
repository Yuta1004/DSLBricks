TARGET_TRIPLE := $(shell rustc -Vv | grep host | cut -f2 -d' ')
OUT_DIR := ./

bin:
	cargo build
	mkdir -p $(OUT_DIR)/bin
	cp ./target/debug/rustdoc_web $(OUT_DIR)/bin
	cp $(OUT_DIR)/bin/rustdoc_web $(OUT_DIR)/bin/rustdoc_web-$(TARGET_TRIPLE)

.PHONY: bin
