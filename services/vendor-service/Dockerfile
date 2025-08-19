# Build stage
FROM golang:1.21-alpine AS builder

WORKDIR /app

# Install git for go modules
RUN apk add --no-cache git

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the application
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o main .

# Runtime stage
FROM alpine:latest

WORKDIR /root/

# Install ca-certificates for HTTPS requests
RUN apk --no-cache add ca-certificates

# Copy binary from builder stage
COPY --from=builder /app/main .

# Create non-root user
RUN adduser -D -s /bin/sh appuser && chown appuser:appuser main

USER appuser

EXPOSE 3002

CMD ["./main"]
