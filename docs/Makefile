IMAGE := klakegg/hugo:0.111.3-ext-alpine

DOCKER_OPTS := \
	--rm \
	-it \
	-p 1313:1313 \
	-v $(shell pwd):/src \
	--entrypoint /bin/bash \
	$(IMAGE)
HUGO_OPTS := \

USER := $(shell whoami)
FILEPATH :=

run:
	@$(call exec_on_hugo_container, server $(HUGO_OPTS))

chapter:
	@$(call exec_on_hugo_container, new --kind chapter $(FILEPATH))

article:
	@$(call exec_on_hugo_container, new $(FILEPATH))

define exec_on_hugo_container
	docker run $(DOCKER_OPTS) -c "\
		hugo $1 && \
		addgroup $(USER) && \
		adduser -D -G $(USER) $(USER) && \
		chown -R $(USER):$(USER) /src/content \
	"
endef
