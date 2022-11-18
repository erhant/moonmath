
# Run tests (verbose, no caching)
test:
	go test ./... -v --count=1

# Shorthand for go mod tidy
tidy:
	go mod tidy 

.PHONY: test