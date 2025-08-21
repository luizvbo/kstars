FROM python:3.13-slim-bookworm

WORKDIR /app
COPY ./requirements.txt /app
COPY ./prefect-flow.py /app
COPY ./target/release/kstars /app

RUN pip install --no-cache-dir uv
RUN uv pip install --no-cache-dir --system -r requirements.txt

EXPOSE 4200
