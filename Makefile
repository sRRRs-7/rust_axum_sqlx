
postgres:
	docker run --name axum_server -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:14-alpine

stopcon:
	docker stop axum_server

delcon:
	docker rm axum_server

createdb:
	docker exec -it axum_server createdb --username=root --owner=root axum_sqlx

dropdb:
	docker exec -it axum_server dropdb axum_sqlx

createsqlx:
	sqlx migrate add ${name}

runsqlx:
	sqlx migrate run


.PHONY: postgres, delcon, stopcon, createdb, dropdb, createsqlx, runsqlx