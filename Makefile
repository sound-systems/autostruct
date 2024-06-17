## print the help message.
# Parses this Makefile and prints targets that are preceded by "##" comments.
help:
	@echo "" >&2
	@echo "Available targets: " >&2
	@echo "" >&2
	@awk -F : '\
			BEGIN { in_doc = 0; } \
			/^##/ && in_doc == 0 { \
				in_doc = 1; \
				doc_first_line = $$0; \
				sub(/^## */, "", doc_first_line); \
			} \
			$$0 !~ /^#/ && in_doc == 1 { \
				in_doc = 0; \
				if (NF <= 1) { \
					next; \
				} \
				printf "  %-15s %s\n", $$1, doc_first_line; \
			} \
			' <"$(abspath $(lastword $(MAKEFILE_LIST)))" \
		| sort >&2
	@echo "" >&2

## executes all unit and integration tests in redis cluster mode
test.unit:
	cargo test --lib --no-fail-fast

## executes postgres integration tests
test.postgres:
	cargo test --test=runner --features=postgres_test

## executes integration tests against all supported databases
test.all: test.postgres
