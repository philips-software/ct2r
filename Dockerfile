FROM rust:latest

LABEL maintainer="Forest Keepers <Jeroen.knoops@philips.com>"

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/ct2r

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/ct2r*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 ct2r

RUN adduser -D -s /bin/sh -u 1000 -G ct2r ct2r

WORKDIR /home/ct2r/bin/

COPY --from=cargo-build /usr/src/ct2r/target/x86_64-unknown-linux-musl/release/ct2r .

RUN chown ct2r:ct2r ct2r

USER ct2r

ENTRYPOINT ["./ct2r"]
