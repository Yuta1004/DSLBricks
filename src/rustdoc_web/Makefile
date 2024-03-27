TARGET_TRIPLE := $(shell rustc -Vv | grep host | cut -f2 -d' ')
OUT_DIR := ./

bin:
	# cargo build -F embedded
	if [ -e $(OUT_DIR)/bin/rustdoc_web ]; then \
		diff ./target/debug/rustdoc_web $(OUT_DIR)/bin ; \
		if [ $$? -eq 1 ]; then \
			make copy ; \
		fi \
	else \
		make copy ; \
	fi

copy:
	mkdir -p $(OUT_DIR)/bin
	cp ./target/debug/rustdoc_web $(OUT_DIR)/bin
	cp $(OUT_DIR)/bin/rustdoc_web $(OUT_DIR)/bin/rustdoc_web-$(TARGET_TRIPLE)

.PHONY: bin copy
