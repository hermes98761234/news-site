# Stage 1: Build Next.js static output
FROM node:20-alpine AS frontend-build
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
ARG NEXT_PUBLIC_API_URL=/
ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL
RUN npm run build

# Stage 2: Build Rust binaries
FROM rust:1.78-alpine AS backend-build
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY backend/ ./
RUN cargo build --release -p server -p cli

# Stage 3: Minimal runtime image
FROM alpine:3.19
RUN apk add --no-cache ca-certificates sqlite-libs
WORKDIR /app

COPY --from=backend-build /app/target/release/server /usr/local/bin/server
COPY --from=backend-build /app/target/release/news-cli /usr/local/bin/news-cli
COPY --from=frontend-build /app/frontend/out /app/static

RUN mkdir -p /data

ENV DATABASE_URL=/data/news.db
ENV STATIC_DIR=/app/static
ENV SERVER_PORT=3000
ENV RUST_LOG=info

EXPOSE 3000
CMD ["/usr/local/bin/server"]
