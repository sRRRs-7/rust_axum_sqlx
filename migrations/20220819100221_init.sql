-- Add migration script here

CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "msg" VARCHAR(200) NOT NULL,
    "age" SMALLINT NOT NULL,
    "image" BYTEA
);

CREATE TABLE "categories" (
    "id" SERIAL PRIMARY KEY,
    "category" VARCHAR(50) NOT NULL
);

CREATE TABLE "posts" (
    "id" SERIAL PRIMARY KEY,
    "user_id" INT REFERENCES users (id),
    "category_id" INT REFERENCES categories (id),
    "titles" VARCHAR(50) NOT NULL,
    "content" TEXT NOT NULL
);