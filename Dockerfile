FROM rustlang/rust:nightly
WORKDIR /build
ADD src/ /build/src
ADD Cargo.toml /build/
RUN cargo build

FROM alpine:latest
WORKDIR /app
RUN addgroup -S app 
RUN adduser -S app -G app
USER app
COPY --from=0 /build/target/debug/rust-hero /app/rust-hero
RUN chown app:app /app/rust-hero
CMD ["/app/rust-hero"]
