cd `dirname $0`

mkdir -p ./output
chmod 777 ./output
docker compose run --rm schemaspy
