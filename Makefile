setup:
	make -C src/editor/src-js setup

docs:
	make -C docs publish

.PHONY: setup docs
