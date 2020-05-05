#!/bin/bash

docker build -t jonathanbeber/rust-grpc-example:v0.1.0 .

docker network create rust-grpc-example
docker run --name server --rm -d --network=rust-grpc-example jonathanbeber/rust-grpc-example:v0.1.0
docker run --name client --rm -it --network=rust-grpc-example jonathanbeber/rust-grpc-example:v0.1.0 bash

docker kill server
docker network rm rust-grpc-example
