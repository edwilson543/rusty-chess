FROM rust as builder

WORKDIR /app
COPY diesel.toml ./
COPY src/data/migrations ./migrations/

# Install the Diesel CLI.
RUN cargo install diesel_cli --no-default-features --features postgres


FROM debian:bookworm-slim as runner

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the diesel binary from builder.
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy the files diesel needs to execute the migrations.
COPY --from=builder /app/diesel.toml /diesel.toml
# Note the `diesel.toml` file points at `src/data/migrations/`
COPY --from=builder /app/migrations /src/data/migrations

# Create entrypoint script.
RUN echo '#!/bin/bash\nset -e\ndiesel setup\ndiesel migration run\necho "Migrations completed successfully!"\n' > /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
