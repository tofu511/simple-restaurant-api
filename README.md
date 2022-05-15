# simple-restaurant-api
- This API is a Rust implementation of Paidy's technical assignment.
- This API consists of the REST-API application and the client application.
	- the REST-API application consists of 4 layers of directories.
		- api-adapter
		- api-core
		- api-driver
		- api-usecase
	- the client application consists of one directory.
		- app-client
- For more information about the assignment, please see [here](https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md)

## Requirements
- [see here](https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md#assignment)

## How to Start API & Client App
### Setup
- install rustup
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup component add clippy rustfmt
```
- install docker
	- get docker from [here](https://docs.docker.com/get-docker/)

### Run API
- start up docker container
```bash
docker-compose up
# or
docker-compose up -d
```
- run api
```bash
cargo run --bin api-driver
```
- the api runs at `localhost:3000`. if you finish start up api, you can call api.
	- for detail API interface, see `./openapi/openapi.yml`
```bash
# this is a example post request
curl -XPOST 'http://localhost:3000/tables/1/item' \
	-H 'content-type: application/json' \
	--data-raw '{"name": "Takoyaki", "quantity": 5}' \
	--compressed

# this is a example get request
curl -XGET 'http://localhost:3000/tables/1/items'
```

### Run Client App
- start up docker container
```bash
docker-compose up
# or
docker-compose up -d
```
- run api
```bash
cargo run --bin api-driver
```
- run client app
```bash
cargo run --bin app-client
```

## Development
### Test
```bash
docker-compose up -d
cargo test
```

### Format
```bash
cargo fmt --all
```

### Lint
```bash
cargo clippy --fix
```

## Architecture
### System Architecture
- This simple restaurant api consists of a Rust api server and MySQL for data persistence.

### Application Architecture
- This simple restaurant api uses a layered architecture to keep the code organized and to help developers to understand the code easily and write a test code easily.
- This api has 4 layers.

#### Responsibility of Each Layer
- api-adapter
	- this layer is for connecting other services or databases.
	- this layer is for specific technical implementations, such as RDB, NoSQL, and REST-API.
- api-core
	- this layer holds the business domain and business logic.
- api-driver
	- this layer is for router implementation and server startup. The layer also handles HTTP status code and processes JSON serialization & deserialization.
- api-usecase
	- this layer describes the user's use case.
	- if it is necessary to access multiple repositories, this layer accesses repositories and aggregates data.

### References
- [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる](https://blog-dry.com/entry/2021/12/26/002649)

