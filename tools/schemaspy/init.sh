#! /bin/sh

cd `dirname $0`

OJDBC_VERSION=23.26.0.0.0
OJDBC_JAR=ojdbc11-${OJDBC_VERSION}.jar
OJDBC_URL=https://repo1.maven.org/maven2/com/oracle/database/jdbc/ojdbc11/${OJDBC_VERSION}/${OJDBC_JAR}

mkdir -p ./drivers
if [ ! -f "./drivers/${OJDBC_JAR}" ]; then
    echo "Downloading ${OJDBC_JAR}..."
    curl -fsSL -o "./drivers/${OJDBC_JAR}" "${OJDBC_URL}"
fi

mkdir -p ./export
chmod 777 ./export
docker compose run --rm schemaspy -dp ./drivers/${OJDBC_JAR}
