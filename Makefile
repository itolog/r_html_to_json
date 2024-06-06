.PHONY : build

check-size:
	 wc -c pkg/r_html_to_json_bg.wasm

wasm-opt:
	wasm-opt -Oz -o  ./pkg/r_html_to_json_bg.wasm ./pkg/r_html_to_json_bg.wasm

tw:
	twiggy top -n 20 pkg/r_html_to_json_bg.wasm

build:
	wasm-pack build --release
run:
	cd ./frontend && npm run dev

.DEFAULT_GOAL := build