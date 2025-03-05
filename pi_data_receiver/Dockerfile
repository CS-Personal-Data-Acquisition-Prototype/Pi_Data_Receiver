FROM rust:1.69-bullseye as builder

# Install build dependencies with arm32 libraries
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    gcc-arm-linux-gnueabihf \
    libc6-dev-armhf-cross \
    pkg-config \
    libsqlite3-dev:armhf \
    libudev-dev:armhf

# Create explicit symlinks for the linker to find ARM32 libraries
RUN mkdir -p /usr/arm-linux-gnueabihf && \
    ln -s /usr/lib/arm-linux-gnueabihf /usr/arm-linux-gnueabihf/lib && \
    ln -s /usr/include /usr/arm-linux-gnueabihf/include

# Set up cross-compilation environment
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
    CC_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc \
    PKG_CONFIG_ALLOW_CROSS=1 \
    PKG_CONFIG_DIR= \
    PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig \
    PKG_CONFIG_SYSROOT_DIR=/ \
    RUSTFLAGS="-C link-arg=-L/usr/lib/arm-linux-gnueabihf"

# Install the appropriate Rust target
RUN rustup target add arm-unknown-linux-gnueabihf

# Create a new project directory
WORKDIR /app

# Copy your project files
COPY . .

# Remove tokio from dependencies since it's causing cross-compilation issues
RUN sed -i 's/tokio = { version = "1", features = \["full"\] }/# tokio disabled for cross-compilation/' Cargo.toml

# Build with cargo
RUN cargo build --target=arm-unknown-linux-gnueabihf --release

# Second stage: Create a minimal runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libsqlite3-0 \
    libudev1 \
    && rm -rf /var/lib/apt/lists/*

# Copy just the binary and config files
COPY --from=builder /app/target/arm-unknown-linux-gnueabihf/release/pi_data_receiver /usr/local/bin/
COPY --from=builder /app/config /etc/pi_data_receiver/config

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/pi_data_receiver"]