# Build stage
FROM elixir:1.12-alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache build-base git

# Install hex and rebar
RUN mix local.hex --force && \
    mix local.rebar --force

# Set build ENV
ENV MIX_ENV=prod

# Copy mix files
COPY mix.exs mix.lock ./

# Install mix dependencies
RUN mix deps.get --only prod

# Copy source code
COPY . .

# Compile the release
RUN mix compile
RUN mix release

# Runtime stage
FROM alpine:3.16

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache openssl ncurses-libs libstdc++

# Copy release from builder stage
COPY --from=builder /app/_build/prod/rel/order_service ./

# Create non-root user
RUN adduser -D -s /bin/sh appuser && chown -R appuser:appuser /app

USER appuser

EXPOSE 4003

CMD ["./bin/order_service", "start"]
