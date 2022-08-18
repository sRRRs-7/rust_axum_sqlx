
postgres:
	docker run --name rust_server -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:14-alpine

createdb:
	docker exec -it rust_server createdb --username=root --owner=root axum_sqlx

dropdb:
	docker exec -it rust_server dropdb axum_sqlx

create_sqlx:
	sqlx migrate add ${name}

run_sqlx:
	sqlx migrate run


.PHONY: postgres, createdb, dropdb, create_sqlx, run_sqlx