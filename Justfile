data_dir := "rocksdb://dev/data"
surreal_url := "127.0.0.1:8000"

alias db := surrealdb

[parallel]
dev: surrealdb migrate serve

surrealdb:
    surreal start --user root --pass root {{data_dir}}

migrate:
    sleep 5
    surrealkit sync --watch --host "ws://{{surreal_url}}"

serve:
    sleep 10
    SURREAL_URL="{{surreal_url}}" SURREAL_USER="root" SURREAL_PASS="root" SURREAL_NS="serverspot-dev" dx serve