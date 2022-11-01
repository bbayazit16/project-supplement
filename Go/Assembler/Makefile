.PHONY: build deps test benchmark clean


GOBIN = ../../bin
VERSION = 0.0.1


flags = "-X 'main.Version=v$(VERSION)'"


build:
	cd ./cmd/cli && GOOS=linux GOARCH=amd64 go build -ldflags=$(flags) -o $(GOBIN)/mnemo-linux-amd64
	cd ./cmd/cli && GOOS=linux GOARCH=arm64 go build -ldflags=$(flags) -o $(GOBIN)/mnemo-linux-arm64

	cd ./cmd/cli && GOOS=darwin GOARCH=amd64 go build -ldflags=$(flags) -o $(GOBIN)/mnemo-mac-amd64
	cd ./cmd/cli && GOOS=darwin GOARCH=arm64 go build -ldflags=$(flags) -o $(GOBIN)/mnemo-mac-arm64

	cd ./cmd/cli && GOOS=windows GOARCH=amd64 go build -ldflags=$(flags) -o $(GOBIN)/mnemo-windows-amd64.exe
	

deps:
	cd ./internal/ethutil && go get github.com/ethereum/go-ethereum
	cd ./internal/ethutil && go get github.com/ethereum/go-ethereum/common
	cd ./internal/ethutil && go get github.com/ethereum/go-ethereum/core/types
	cd ./internal/ethutil && go get github.com/ethereum/go-ethereum/crypto
	cd ./internal/ethutil && go get github.com/ethereum/go-ethereum/ethclient

test:
	cd ./test && go test


benchmark:
	cd ./test && go test -bench=.


# Alias for benchmark
bench:
	go test -bench=. ./test


clean:
	rm bin/*