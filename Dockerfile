# Dockerfile for building libgovarnam C shared library & testing govarnam-rust
FROM golang:1.22-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    git \
    libsqlite3-dev \
    cargo \
    rustc \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Clone and build govarnam shared library
WORKDIR /build
RUN git clone https://github.com/varnamproject/govarnam.git
WORKDIR /build/govarnam
RUN go build -buildmode=c-shared -o libgovarnam.so .

# Final test runner image
FROM rust:1.78-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libsqlite3-0 \
    ca-certificates \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy built shared library and header from builder stage
COPY --from=builder /build/govarnam/libgovarnam.so /usr/local/lib/libgovarnam.so
COPY --from=builder /build/govarnam/libgovarnam.h /usr/local/include/libgovarnam.h

# Update ldconfig cache so linker and runtime dynamically find libgovarnam
RUN ldconfig

WORKDIR /workspace

CMD ["cargo", "test", "--", "--nocapture"]
