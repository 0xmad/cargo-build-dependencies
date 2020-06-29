# cargo-build-dependencies

[![Crates.io](https://img.shields.io/crates/v/cargo-build-dependencies.svg)](https://crates.io/crates/cargo-build-dependencies)

This tool extends [Cargo](https://doc.rust-lang.org/cargo/) to allow you to
build only the dependencies in a given rust project. This is useful for docker
builds where each build step is cached. The time it takes to build dependencies
is often a significant portion of the overall build time. Therefore it is
beneficial in docker builds to build dependencies in a separate step earlier
than the main build. Since the dependency building step will be cached,
dependencies will not need to be rebuilt when the project's own source code
changes.

Based on https://github.com/nacardin/cargo-build-deps


## Install
`cargo install cargo-build-dependencies`

## Usage
`cargo build-dependencies`

## Example

Change Dockerfile from

```
FROM rust:1.43 as rust-builder
RUN mkdir /tmp/PROJECT_NAME
WORKDIR /tmp/PROJECT_NAME
COPY . .
RUN cargo build  --release
```

to

```
FROM rust:1.43 as rust-builder
RUN cargo install cargo-build-dependencies
RUN cd /tmp && USER=root cargo new --bin PROJECT_NAME
WORKDIR /tmp/PROJECT_NAME
COPY Cargo.toml Cargo.lock ./
RUN cargo build-dependencies --release
COPY src /tmp/PROJECT_NAME/src
RUN cargo build  --release
```
