# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.80.1
ARG APP_NAME=zenload

FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

COPY --from=xx /usr/bin/ /usr/bin/
COPY src/ src/
COPY Cargo.toml Cargo.lock ./

RUN apk add --no-cache clang lld musl-dev git file
ARG TARGETPLATFORM
RUN xx-apk add --no-cache musl-dev gcc

RUN set -ex && \
    xx-cargo build --locked --release --target-dir ./target && \
    cp ./target/$(xx-cargo --print-target-triple)/release/${APP_NAME} /tmp/${APP_NAME} && \
    xx-verify /tmp/${APP_NAME}

FROM alpine:3.18 AS final
ARG UID=10001
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" appuser
USER appuser
COPY --from=build /tmp/${APP_NAME} /bin/
EXPOSE 1024
CMD ["sh", "-c", "while true; do sleep 3600; done"]
