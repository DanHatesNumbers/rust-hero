FROM clux/muslrust
WORKDIR /build
ADD src/ /build/src
ADD Cargo.toml /build/
RUN cargo build

FROM alpine:latest
WORKDIR /app
RUN addgroup -S app 
RUN adduser -S app -G app
COPY --from=0 /build/target/x86_64-unknown-linux-musl/debug/rust-hero /app/rust-hero
RUN chown app:app /app/rust-hero
USER app
EXPOSE 8000
ENV ROCKET_ADDRESS 0.0.0.0
CMD ["rust-hero"]
