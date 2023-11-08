.PHONY : build

build:
	wasm-pack build
run:
	cd ./frontend && npm run dev

.DEFAULT_GOAL := build