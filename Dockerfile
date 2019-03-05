ARG BUILD_ARCH=x86_64
FROM getsentry/rust-musl-cross:$BUILD_ARCH-musl AS builder

ARG BUILD_ARCH
ENV BUILD_TARGET=$BUILD_ARCH-unknown-linux-musl
WORKDIR /app/

# Build only dependencies to speed up subsequent builds
ADD Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release --target=$BUILD_TARGET --locked

# Add all sources and rebuild
ADD src src/

RUN touch src/main.rs && cargo build --target=$BUILD_TARGET --release

# Copy the compiled binary to a target-independent location so it can be picked up later
RUN cp target/$BUILD_TARGET/release/ftbl_in /usr/local/bin/ftbl_in


FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /app/
ADD ftbl.toml /app/ftbl.toml
COPY --from=builder /usr/local/bin/ftbl_in /usr/local/bin
ENTRYPOINT ftbl_in
