#! /bin/sh

cd `dirname $0`

cd ../tools/openapi
pnpm export

cd ../..
mkdir -p ./tools/openapi-generator/spec
cp ./tools/openapi/export/openapi.yaml ./tools/openapi-generator/spec/openapi.yaml

docker compose run --rm openapi-generator \
  generate \
  -i //spec/openapi.yaml \
  -g rust \
  -o //export \
  --additional-properties=packageName=web
