FROM ghcr.io/static-web-server/static-web-server:2.42-debian

WORKDIR /

RUN mkdir /public/hilfe

COPY ./book /public/hilfe