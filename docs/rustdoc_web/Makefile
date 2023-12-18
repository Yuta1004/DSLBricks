TARGET_TRIPLE := $(shell rustc -Vv | grep host | cut -f2 -d' ')

bin:
	cargo build
	mkdir -p bin
	cp ./target/debug/rustdoc_web bin
	cp bin/rustdoc_web bin/rustdoc_web-$(TARGET_TRIPLE)

.PHONY: bin
