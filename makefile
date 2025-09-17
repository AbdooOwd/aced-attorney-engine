all: clean build-release

clean:
	rm -r build/*

build-release:
	mkdir -p build
	cargo build --release
	cp target/release/aced-attorney-engine build/
	cp -r assets build/
	