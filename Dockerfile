ARG BUILD_ARCH=x86_64
FROM ekidd/rust-musl-builder:latest AS builder

ARG BUILD_ARCH
ENV BUILD_TARGET=$BUILD_ARCH-unknown-linux-musl

# Build only dependencies to speed up subsequent builds
ADD Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release --target=$BUILD_TARGET --locked

# Add all sources and rebuild
ADD src src/
ADD ftbl.toml .

RUN sudo touch src/main.rs && cargo build --target=$BUILD_TARGET --release

# Copy the compiled binary to a target-independent location so it can be picked up later
RUN sudo cp target/$BUILD_TARGET/release/ftbl_in /usr/local/bin/ftbl_in \
    && strip /usr/local/bin/ftbl_in


FROM scratch
COPY --from=builder /etc/ssl /etc/ssl
COPY --from=builder /usr/local/bin/ftbl_in /
CMD ["/ftbl_in"]
