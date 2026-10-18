#!/bin/sh

cd `dirname $0`

while ! docker exec postgres pg_isready -U myuser -d mydb
do
    sleep 1
done

docker compose run --rm flyway-clean
docker compose run --rm flyway-migrate
docker compose run --rm flyway-info
