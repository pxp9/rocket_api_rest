# Rocket API REST

## How to run it ??

### Run a PostgreSQL container.
- Install docker.
```
make db
```

### Run diesel migrations.
- Install diesel-cli
```
cargo install diesel_cli --no-default-features --features=postgres
```

- Migrations
```
make diesel
```

### Run API REST.
```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/people_db cargo run
```

## It has 3 methods

- 1 GET /people/id where id is person id from DB 

- 1 POST /new_person json as a body

- 1 PUT /modify/id?name=&age= where id is person id from DB

## Examples to run requests in the API REST

### GET

```
curl http://127.0.0.1:8000/people/1
```

### POST
```
curl -i -X POST -H 'Content-Type: application/json' -d '{"name": "Juanes", "age": 55}' http://127.0.0.1:8000/new_person
```

### PUT
note simple quotes in url because it has special characters !
```
curl -i -X PUT 'http://127.0.0.1:8000/modify/1?name=Ayrat&age=30'
```

### DELETE

```
curl -i -X DELETE 'http://127.0.0.1:8000/remove/1'
```
