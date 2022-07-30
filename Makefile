.PHONY: completions clean radium

radium: Cargo.toml
	cargo build --release

debug: Cargo.toml
	cargo build

completions: completions/completions.conf
	cd completions && completely generate completions.conf

clean:
	rm -r target
