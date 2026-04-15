FROM ghcr.io/static-web-server/static-web-server:2.42-debian

RUN mkdir /public/hilfe

COPY ./book /public/hilfe