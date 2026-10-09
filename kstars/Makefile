.PHONY: build-crate test install run
build-crate:
	cargo build --release 

test:
	cargo test

install:
	if command -v uv > /dev/null; then \
		uv pip install -r requirements.txt; \
	else \
		pip install -r requirements.txt; \
	fi

run: build-crate
	python main.py
