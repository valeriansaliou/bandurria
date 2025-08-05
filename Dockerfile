FROM rust:1.86.0-alpine AS build

RUN rustc --version && \
    cargo --version

RUN apk add --no-cache musl-dev

WORKDIR /build

COPY . /build

RUN cargo build --release
RUN mv ./target/release/bandurria .

FROM scratch

WORKDIR /usr/src/bandurria

COPY --from=build /build/bandurria /usr/local/bin/bandurria

COPY ./res/assets/ ./res/assets/
COPY config.cfg /etc/bandurria.cfg

CMD [ "bandurria", "-c", "/etc/bandurria.cfg" ]

EXPOSE 8080
