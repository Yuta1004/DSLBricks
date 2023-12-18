TARGET_TRIPLE := $(shell rustc -Vv | grep host | cut -f2 -d' ')

docs_bin:
	cargo build
	mkdir -p bin
	cp ./target/debug/docs_web bin/docs_web
	cp bin/docs_web bin/docs_web-$(TARGET_TRIPLE)
