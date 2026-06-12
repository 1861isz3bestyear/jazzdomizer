# Docker

This repo runs three containers:

- `postgres`: PostgreSQL 16 with schema initialization from `postgres/initdb`
- `backend`: Rust/Axum API on container port `3000`
- `frontend`: nginx static frontend on container port `80`, proxying `/api` to the backend

## Run the stack

```sh
docker compose up --build
```

Open the frontend at <http://localhost:8080>.

Useful health checks:

```sh
curl http://localhost:8080/api/health
curl http://localhost:3000/api/health
```

The frontend calls `/api` on the same origin, so browser requests go through nginx to the backend service inside the Docker network. The backend is also published on host port `3000` for direct API checks.

Postgres is exposed on local port `5432` and uses:

- database: `rust_sql_one_db`
- user: `jazzdomizer`
- password: `jazzdomizer`
- container URL: `postgres://jazzdomizer:jazzdomizer@postgres:5432/rust_sql_one_db`
- host URL: `postgres://jazzdomizer:jazzdomizer@127.0.0.1:5432/rust_sql_one_db`

The database data is kept in the Docker named volume `jazzdomizer_postgres_data`.

## Database initialization

Fresh Docker volumes are initialized from one schema source and the seed data dump:

```text
postgres/initdb/
  001_schema.sql
  002_data_only.sql
```

That creates the `songs`, `harmony`, and `list_data` tables required by the backend and seeds the data from the local `rust_sql_one_db` database.

The backend also ensures this schema when it starts. This covers existing Docker volumes where Postgres prints:

```text
PostgreSQL Database directory appears to contain a database; Skipping initialization
```

If you already have that state and the browser shows API errors like `relation "songs" does not exist`, rebuild the backend image:

```sh
docker compose up --build
```

Postgres only runs `postgres/initdb` files when its data directory is empty. If you need to re-run init files or replace the database contents completely, recreate the volume with:

```sh
docker compose down -v
docker compose up --build
```

## Move a local database into Docker

The current local `.env` points to a Unix-socket Postgres database:

```sh
DATABASE_URL='postgres:///rust_sql_one_db?host=/tmp'
```

Create a portable data-only dump from the local machine:

```sh
pg_dump --data-only --inserts --column-inserts --no-owner --no-acl \
  'postgres:///rust_sql_one_db?host=/tmp' \
  > postgres/initdb/002_data_only.sql
```

Then create the Docker database from that dump:

```sh
docker compose down -v
docker compose up --build
```

Files are executed in sorted filename order for fresh volumes. The Postgres image loads `postgres/initdb/001_schema.sql`, then loads `postgres/initdb/002_data_only.sql`. If your dump includes schema, make sure it is compatible with or replaces the existing schema source.

For an existing Docker database that you do not want to delete, restore directly:

```sh
pg_dump --format=custom --no-owner --no-acl \
  'postgres:///rust_sql_one_db?host=/tmp' \
  -f /tmp/jazzdomizer.dump

docker compose up -d postgres
docker cp /tmp/jazzdomizer.dump jazzdomizer-postgres:/tmp/jazzdomizer.dump
docker compose exec postgres pg_restore \
  -U jazzdomizer \
  -d rust_sql_one_db \
  --clean \
  --if-exists \
  /tmp/jazzdomizer.dump
```

## Contain future schema changes

Keep schema changes as SQL files instead of making manual database edits. A simple convention for this repo is:

```text
postgres/initdb/
  002_seed_data.sql
```

Keep the current schema in `postgres/initdb/001_schema.sql`; Docker loads that into fresh Postgres containers and the backend runs the same SQL at startup. Keep small seed data under `postgres/initdb`. For ongoing production-style migrations, add a migration tool such as `sqlx-cli`, keep migrations under `backend/migrations`, and run:

```sh
DATABASE_URL='postgres://jazzdomizer:jazzdomizer@127.0.0.1:5432/rust_sql_one_db' sqlx migrate run
```

Do not store Postgres data files in git. Store schema migrations and small seed data; use dumps for moving real local data into Docker.
