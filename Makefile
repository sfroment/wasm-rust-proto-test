.PHONY: gen
gen: deps
	buf generate

.PHONY: deps
deps:
	buf dep update
