#!/usr/bin/env bash
set -e

main() {
	need_cmd docker
	need_cmd psql
	need_cmd sqlx

	POSTGRES_DB=${POSTGRES_DB:=newsletter}
	POSTGRES_HOST=${POSTGRES_HOST:=localhost}
	POSTGRES_PASSWORD=${POSTGRES_PASSWORD:=password}
	POSTGRES_PORT=${POSTGRES_PORT:=5432}
	POSTGRES_USER=${POSTGRES_USER:=postgres}
	postgres_start
	postgres_wait
	postgres_migrate
}

postgres_start() {
	if [[ -z $SKIP_DOCKER ]]; then
		docker run -d \
			--name postgres \
			-e POSTGRES_DB="${POSTGRES_DB}" \
			-e POSTGRES_HOST="${POSTGRES_HOST}" \
			-e POSTGRES_PASSWORD="${POSTGRES_PASSWORD}" \
			-e POSTGRES_USER="${POSTGRES_USER}" \
			-p "${POSTGRES_PORT}":5432 \
			postgres -N 1000
	fi
}

postgres_wait() {
	export PGPASSWORD="${POSTGRES_PASSWORD}"
	until psql -h ${POSTGRES_HOST} -U ${POSTGRES_USER} -p ${POSTGRES_PORT} -d "postgres" -c '\q'; do
		say "Postgres is still unavailable - sleeping"
		sleep 1
	done
}

postgres_migrate() {
	export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"
	sqlx database create
	sqlx migrate run
	say "Postgres has been migrated, ready to go!"
}

say() {
    printf 'init_db: %s\n' "$1"
}

err() {
    say "$1" >&2
    exit 1
}

need_cmd() {
    if ! check_cmd "$1"; then
        err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

main
