- flow
  router -> handler -> repository -> cors -> usecase

- module
  db, models, error
  sqlx: migrations -> sql syntax

- db
  connect

- models
  table structure
  req structure
  res structure

- error
  error handling

- router
  uri, handler, axum::route

- handler
  invoked methods, query, repository

- repository
  sqlx query
  pool
  request, response
  (mock)
  -> mod collect -> collect methods

- cors
  create middleware

- usecase
  invoke methods from repository
  test

- curl request
  curl -v -X POST http://127.0.0.1:7878/users/add -d '{"name":"Rustan", "msg":"Rust","age":26}' -H 'content-type:application/json'

- process
  main -> cors -> repository -> routing -> handler -> usecase
