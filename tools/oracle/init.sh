#! /bin/sh

cd `dirname $0`

while ! docker exec oracle bash -c "echo \"SELECT 'READY' FROM DUAL;\" | sqlplus -s -L system/oracle@localhost:1521/FREEPDB1" 2>/dev/null \
    | grep -q "READY";
do
    echo "Waiting for Oracle to be ready..."
    sleep 3
done

docker exec -i oracle bash -c "sqlplus -S system/oracle@localhost:1521/FREEPDB1" < ./setup/create_users.sql