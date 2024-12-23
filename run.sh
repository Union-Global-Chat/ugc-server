docker run \
    --name mariadb \
    -e MARIADB_USER=dev \
    -e MARIADB_DATABASE=dev \
    -e MARIADB_PASSWORD=dev \
    -e MARIADB_ROOT_PASSWORD=dev \
    -p 3306:3306 \
    -d mariadb:latest
