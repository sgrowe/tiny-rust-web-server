FROM rust:1.41.0 AS builder


WORKDIR /opt/app


RUN rustup target install x86_64-unknown-linux-musl


COPY Cargo.toml Cargo.lock /opt/app/


# TODO: cache built dependencies


COPY src /opt/app/src/


RUN cargo build --release --target=x86_64-unknown-linux-musl


FROM scratch


WORKDIR /opt/app


COPY --from=builder /opt/app/target/x86_64-unknown-linux-musl/release/tokio-stuff /opt/app/tokio-stuff


EXPOSE 3000


ENTRYPOINT [ "/opt/app/tokio-stuff" ]
