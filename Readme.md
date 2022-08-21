- flow
  router
  -> handler
  -> repository
  -> cors
  -> usecase

- process
  main
  -> cors
  -> repository
  -> routing
  -> handler
  -> usecase

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

- function
  transaction
  middleware
  authentication

- main
  cors
  server setup
- router
  req receive
- handler -> response format json
  usecase -> implement & test
  repository -> methods define & collect
  sql -> DB query
  db -> connect

- curl request
  curl -v -X POST http://127.0.0.1:7878/users/add -d '{"name":"Rustan", "msg":"Rust","age":26}' -H 'content-type:application/json'
  curl -v -X POST http://127.0.0.1:7878/categories/add -d '{"category":"cate"}' -H 'content-type:application/json'
  curl -v -X POST http://127.0.0.1:7878/posts/add -d '{"user_id":1, "category_id":1, "titles":"new titles", "content":"new content"}' -H 'content-type:application/json'

  curl -v -X GET http://127.0.0.1:7878/users
  curl -v -X GET http://127.0.0.1:7878/users/detail/1
  curl -v -X GET http://127.0.0.1:7878/categories
  curl -v -X GET http://127.0.0.1:7878/categories/detail/1
  curl -v -X GET http://127.0.0.1:7878/posts
  curl -v -X GET http://127.0.0.1:7878/posts/find/user_id/1
  curl -v -X GET http://127.0.0.1:7878/posts/find/category_id/1
  curl -v -X GET http://127.0.0.1:7878/posts/find/titles\?titles\=rust
  curl -v -X GET http://127.0.0.1:7878/posts/find/content\?content\=rust
  curl -v -X GET http://127.0.0.1:7878/posts/detail/1

  curl -v -X PUT http://127.0.0.1:7878/users/edit/2 -d '{"name":"Rustan", "msg":"Rust","age":26}' -H 'content-type:application/json'
  curl -v -X PUT http://127.0.0.1:7878/categories/edit/2 -d -d '{"category":"cate"}' -H 'content-type:application/json'
  curl -v -X PUT http://127.0.0.1:7878/posts/edit/2 -d '{"user_id":1, "category_id":1, "titles":"new titles", "content":"new content"}' -H 'content-type:application/json'

  curl -v -X DELETE http://127.0.0.1:7878/users/delete/1
  curl -v -X DELETE http://127.0.0.1:7878/categories/delete/1
  curl -v -X DELETE http://127.0.0.1:7878/posts/delete/1

- multipart/form-data
  curl -v -X POST -F img=@/Users/srrrs/Desktop/Image/golang.png -F user_id=1 http://127.0.0.1:7878/users/edit/image
