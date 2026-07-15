FROM rust:1.97.0 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

COPY migrations ./migrations

RUN cargo build --release

FROM debian:13.6-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl3 ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/releasenotes /usr/local/bin/releasenotes

ENV ROOT_CA=
ENV TFS_CERT=

ENV CERT_PW=
ENV TFS_TOKEN=
ENV TFS_AMIC=
ENV TFS_AEINS=
ENV DB_USER=
ENV DB_PW=
ENV DB_HOST=
ENV DB_PORT=
ENV DB_NAME=

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=18080

ENV TZ=Europe/Berlin

RUN ln -sf /usr/share/zoneinfo/${TZ} /etc/localtime && echo ${TZ} > /etc/timezone

EXPOSE 18080

CMD [ "releasenotes" ]
