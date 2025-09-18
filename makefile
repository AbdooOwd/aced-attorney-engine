all: clean build-release

clean:
	rm -r build/*

build-release build:
	mkdir -p build
	cargo build --release
	cp target/release/aced-attorney-engine build/
	cp -r assets build/


run-release run: build-release
	./build/aced-attorney-engine