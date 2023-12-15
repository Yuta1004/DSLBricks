IMAGE := dslbricks-editor:build
SH := /bin/ash
RUN_OPTS :=\
	--rm \
	-p 5000:5000 \
	-it \
	-v $(shell pwd)/src:/workdir/src \
	-v $(shell pwd)/next.config.js:/workdir/next.config.js \
	-v $(shell pwd)/tsconfig.json:/workdir/tsconfig.json \
	-v $(shell pwd):/outdir
BUILD_OPTS :=\
	--rm \
	-v $(shell pwd)/src:/workdir/src \
	-v $(shell pwd)/next.config.js:/workdir/next.config.js \
	-v $(shell pwd)/tsconfig.json:/workdir/tsconfig.json \
	-v $(shell pwd):/outdir

run:
	docker run $(RUN_OPTS) $(IMAGE) $(SH) -c "npm run dev"

build:
	docker run $(BUILD_OPTS) $(IMAGE) $(SH) -c "npm run build && rm -rf ../outdir/dist && mv out ../outdir/dist"

shell:
	docker run $(OPTS) $(IMAGE) $(SH)

image:
	docker build -t $(IMAGE) .
