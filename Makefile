.PHONY: build-crate test install run deploy schedule
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

run:
	python main.py

deploy:
	prefect deployment build main.py:run_kstars_flow -n kstars -q default -o kstars-deployment.yaml
	prefect deployment apply kstars-deployment.yaml

schedule:
	prefect deployment schedule kstars -i kstars/weekly
