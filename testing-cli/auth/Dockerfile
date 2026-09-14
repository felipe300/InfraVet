FROM ubuntu:24.04

# DF010: apt-get install sin limpiar cache
RUN apt-get update && apt-get install -y nginx

# DF010: apt install sin limpiar cache
RUN apt update && apt install -y curl

# DF010: sintaxis alternativa
RUN apt get install -y wget

# NO debería generar DF010
RUN apt-get update && apt-get install -y git \
    && rm -rf /var/lib/apt/lists/*

# NO debería generar DF010
RUN apt install -y vim \
    && rm -rf /var/lib/apt/lists/*


FROM alpine:3.22

# DF010
RUN apk add curl

# No DF010: usa --no-cache
RUN apk add --no-cache wget

# No DF010: limpia caché estándar
RUN apk add git && rm -rf /var/cache/apk/*

# No DF010: limpia la ruta alternativa
RUN apk add bash && rm -rf /var/check/apk/*
