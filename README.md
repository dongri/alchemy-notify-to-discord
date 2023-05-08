# alchemy-notify-to-discord

## Environment Variables
```
$ mv .example.env .env
$ vim .env
DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_NAME=notify
DATABASE_USER=user
DATABASE_PASSWORD=password
ERROR_WEBHOOK_URL=https://discord.com/api/webhooks/xxxx
```

## Creating Postgres Database
Execute ddl.sql to create tables.

## Starting the Server
```
$ docker-compose up -d
or
$ ./run.sh
```

## Registering Notify on Alchemy
https://dashboard.alchemy.com/notify
Register NFT and Address, and set the WEBHOOK URL to:
```
https://{your domain}/nft
https://{your domain}/address
```
