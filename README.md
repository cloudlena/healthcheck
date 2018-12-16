# Healthcheck

[![Build Status](https://img.shields.io/travis/mastertinner/healthcheck.svg?style=flat-square)](https://travis-ci.org/mastertinner/healthcheck)
[![Docker Build](https://img.shields.io/docker/build/mastertinner/healthcheck.svg?style=flat-square)](https://hub.docker.com/r/mastertinner/healthcheck)

A base image for minimal Docker images. It is an extension of `scratch` that contains a built in HTTP health check and encourages non-privileged execution.
This image only adds about 3.6M to `scratch` and is intended for running HTTP services written in languages that compile to a binary format (e.g. [Go](https://golang.org) or [Rust](https://www.rust-lang.org)).

## Advantages

- Your binary will be executed using the non privileged `app` user
- A HTTP health check is run every 30 seconds

## Usage

### Rust

```Dockerfile
FROM rust:1 AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM mastertinner/healthcheck:latest
WORKDIR /usr/myapp
COPY --from=builder /usr/src/app/target/release/myapp .
EXPOSE 8080
ENTRYPOINT ["./myapp"]
```

You can add the following lines to your `Cargo.toml` file to further minimize the size of the final image:

```toml
[profile.release]
opt-level = "z"
lto = true
```

### Go

```Dockerfile
FROM golang:1 AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN CGO_ENABLED=0 go build -ldflags="-s -w" -a -installsuffix cgo -o bin/clong ./cmd/clong

FROM healthcheck:latest
WORKDIR /usr/myapp
COPY --from=builder /usr/src/myapp/myapp .
EXPOSE 8080
ENTRYPOINT ["./myapp"]
```

### Customize

You can set the following environment variables in your container to customize the health check:

- `PORT`: The port your app is listening on. Defaults to `8080`.
- `HEALTHCHECK_PATH`: The health check path of your app. Defaults to `/`.
