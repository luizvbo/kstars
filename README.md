
## Prefect

```shell
prefect gcl create rate-limited-gh-api --limit 5 --slot-decay-per-second 1.0

prefect work-pool create local --type process
prefect worker start --pool local

```
