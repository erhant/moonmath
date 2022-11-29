
# Run tests (verbose, no caching)
test:
	go test ./... -v --count=1

testset1:
	go test ./pkg/set1/* -v --count=1
testset2:
	go test ./pkg/set2/* -v --count=1

# Shorthand for go mod tidy
tidy:
	go mod tidy 

.PHONY: test