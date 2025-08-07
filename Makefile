run: build
	@./target/debug/server

build: 
	@cargo build

benchmark:
	@hyperfine --warmup 3 './target/debug/server'
