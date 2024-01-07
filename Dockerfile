# Stage 1: Build Tailwind
ARG NODE_MAJOR=20
FROM node:${NODE_MAJOR} AS tailwind-build
WORKDIR /app
COPY package.json .
RUN npm install
COPY src src
COPY tailwind.config.js .
COPY input.css .
RUN npx tailwindcss -i ./input.css -o ./main.css

# Stage 2: Build Rust Application
FROM rust:1.71.0-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev curl
COPY . .
COPY --from=tailwind-build /app/main.css /app/style/main.css
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
RUN cargo leptos build --release -vv

# Stage 3: Create the Final Image
FROM debian:bullseye-slim AS final
ARG APP_NAME
WORKDIR /app
RUN apt-get update && apt-get install -y openssl
COPY --from=build /app/target target

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV RUST_ENV=production
ARG UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN chown -R appuser:appuser /app && chmod -R 755 /app
USER appuser
EXPOSE 3000
CMD ["/app/target/release/leptos-cv"]
