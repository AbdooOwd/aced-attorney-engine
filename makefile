all: clean build-release

clean:
	rm -r build/*

# for now, by default, it works on "Release"
build: build-release
run: run-release

build-release:
	mkdir -p build
	cargo build -r
	cp target/release/aced-attorney-engine build/
	cp -r assets build/


run-release: build-release
	./build/aced-attorney-engine