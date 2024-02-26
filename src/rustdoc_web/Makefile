TARGET_TRIPLE := $(shell rustc -Vv | grep host | cut -f2 -d' ')
OUT_DIR := ./

bin:
	cargo build -F embedded
	mkdir -p $(OUT_DIR)/bin
	cp -u ./target/debug/rustdoc_web $(OUT_DIR)/bin
	cp -u $(OUT_DIR)/bin/rustdoc_web $(OUT_DIR)/bin/rustdoc_web-$(TARGET_TRIPLE)

.PHONY: bin
