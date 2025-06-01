FROM rust as builder

WORKDIR /app
COPY . .

# Build the application.
RUN cargo build --release


FROM debian:bookworm-slim as runner

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the application binary and entrypoint script from builder.
COPY --from=builder /app/target/release/chess /usr/local/bin/chess

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_port=8000

EXPOSE 8000
CMD ["/usr/local/bin/chess"]
