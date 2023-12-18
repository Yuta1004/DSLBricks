TARGET_TRIPLE := $(shell rustc -Vv | grep host | cut -f2 -d' ')

docs_bin:
	cd ../../../../ && cargo build --bin docs_web --release
	cp ../../../../target/release/docs_web ../../../../target/release/docs_web-$(TARGET_TRIPLE)
