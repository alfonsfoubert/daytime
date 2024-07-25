build:
	cargo build --release

install: build
	mv ./target/release/daytime ~/.local/bin/

clean:
	rm ~/.local/bin/daytime
