# Multi-stage build for production-ready Sonic SVM GDP Dashboard
FROM rust:1.75-slim-bullseye as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Remove dummy main.rs and copy actual source code
RUN rm -rf src
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/pyth_gdp_fetcher .

# Copy static files
COPY --from=builder /app/static ./static

# Set ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Default command
CMD ["./pyth_gdp_fetcher", "--port", "3000"]

# Labels for container management
LABEL maintainer="Sonic SVM + Pyth Network Team"
LABEL version="0.1.0"
LABEL description="Enterprise-grade real-time economic data platform"
LABEL org.opencontainers.image.source="https://github.com/yourusername/sonic-svm-gdp-dashboard"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.title="Sonic SVM GDP Dashboard"
LABEL org.opencontainers.image.description="Real-time US economic data via Sonic SVM and Pyth Network"
