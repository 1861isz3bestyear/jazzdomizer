FROM postgres:16-alpine

COPY ../../postgres/initdb/001_schema.sql /docker-entrypoint-initdb.d/001_schema.sql
COPY ../../postgres/initdb/002_data_only.sql /docker-entrypoint-initdb.d/002_data_only.sql
