FROM node:18-alpine as web-builder
WORKDIR /src
COPY client/package.json ./
COPY client/package-lock.json ./
RUN npm install
COPY client/* ./
RUN npm run build


FROM rust:alpine3.15 as builder

WORKDIR /build
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY migrations migrations
COPY diesel.toml diesel.toml
COPY src src
RUN cargo install --path .


FROM alpine:alpine3.15
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release /app

COPY --from=web-builder /src/build /app/static

ENV STATIC_FILE_PATH=/app/static
ENV DATABASE_URL=/app/db.sqlite3

CMD ["/app/deck_buddy"]
