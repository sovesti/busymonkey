# some clarifications for CD/ folder

## requirements for usage

- Docker Compose v2+
- set postgreSQL password

## usage

1. create .env and .config.toml at CD folder, relating on example files (pasting your postgres password)
2. run following commands:

```sh
docker compose --env-file .env config --quiet
docker compose --env-file .env up -d --build --wait
docker compose ps
docker compose logs --tail=100 app
```

3. http://localhost for visiting (or add ur specific port xd)
4. for shutting down:

```sh
docker compose down
```

5. to remove services with postgres data (**BE CAREFUL!!! REALLY DELETES ALL DATA FROM DB AND VOLUME!!!**):

```sh
docker compose down -v
```

OR

2. build an image locally and deploy it (replace `ghcr.io/somebody/busymonkey:a81c327` with your own image, ghcr is an example):

```sh
./deploy.sh ghcr.io/somebody/busymonkey:a81c327
```

3. and to see some logs (for each container state name as follows (app/postgres/proxy, see compose.yaml)):

```sh
docker compose logs --tail=100 app
```

## FAQ

Ask solvrx
