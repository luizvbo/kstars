
## Prefect

```shell
prefect gcl create rate-limited-gh-api --limit 1 --slot-decay-per-second 0.09

export PREFECT_API_URL="http://127.0.0.1:4200/api"

prefect work-pool create local --type process
prefect worker start --pool local

```
