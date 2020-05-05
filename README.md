# Rust gRPC example

This repository contains a Rust [gRPC](https://grpc.io/) service and its CLI as a simple implementation example.

It uses the [tikv/grpc-rs](https://github.com/tikv/grpc-rs) crate combined with the [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf/).

This code is based on an hiring coding challenge. I removed all the references to the company and its resources.

# Docs

* The [ADRs](https://labs.spotify.com/2020/04/14/when-should-i-write-an-architecture-decision-record/) used to define the system are in the `docs` path;
* Run `cargo doc --open` for code documentation. For those using purely docker, unfortunately, this project does not export the docs from the docker image yet but the docker image has `lynx` installed. You can read the docs inside the container with `cargo doc && lynx /usr/src/stock/target/doc/stock/index.html`;

# Run it!

The script `run.sh` will start the environment, returning the user a shell with the client configured. The docker image created here is only to be used as an easy way to test the application. This docker image should not be used in production environments.

**Obs**: `run.sh` needs docker installed and running.

```bash
$ bash run.sh
```

It'll generate an output similar to the following, resulting in a console into a client container:

```bash
...
long build output
...
Successfully built IMAGEID
Successfully tagged jonathanbeber/stock:v0.1.0
...
root@CONTAINERID:/usr/src/stock# client --help
root@CONTAINERID:/usr/src/stock# client --host server list --store VENEZA_IT
root@CONTAINERID:/usr/src/stock# client --host server list --show-unavailable
root@CONTAINERID:/usr/src/stock# client --host server list --show-unavailable --store BERLIN_DE
```

In this container, the command `client` is the CLI application with access to the server.

It's also possible to run the tests inside this container.

```
root@CONTAINERID:/usr/src/stock# cargo test
```

## Check the server logs

On `Cargo.toml` the external crate `slog` is defined with `TRACE` log level even for release. It's not recommended at all for production environments. It's configured this way to create better visibility for the challenge.

To check the server logs, run: `docker logs -f server`. This command must be run out of the client container on the Docker host machine.

# Development

## Requirements

This application is written in Rust. Check the [docs](https://www.rust-lang.org/tools/install) to install it.

## Starting the server

```bash
cargo run --bin server

```
It's possible to define the listener address with `cargo run --bin server -- --addr HOST:PORT`.

E.g.:

```
eg:  cargo run --bin server -- --addr 0.0.0.0:8080
```

## Running tests

```
cargo test
```

## Protos

The proto files are located on the `src/proto/stock.proto`. During the build, `stock.rs` and `stock_grpc.rs` will be generated. These files are not supposed to be committed and are already configured on `.gitignore`.
