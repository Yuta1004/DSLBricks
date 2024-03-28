MAKEFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

SRCS :=\
	$(wildcard src/components/*.tsx) \
	$(wildcard src/pages/*.tsx) \
	src/style.css \
	package.json \
	next.config.js

run:
	npm run dev

build: $(SRCS)
	npm run build

setup: node_modules bundle_rustdoc

node_modules:
	npm install

bundle_rustdoc:
	cargo doc -p lib_catalog --lib
	mkdir -p public
	cp -r ../../../target/doc ./public/rustdoc

.PHONY: run setup
