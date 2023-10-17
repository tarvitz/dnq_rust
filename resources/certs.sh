#!/bin/bash
#: see also: https://core.telegram.org/bots/self-signed
#: use `CN="yourdomain.com" ./certs.sh` to generate self-signed
#: certificate for testing
CN="${CN:-"localhost"}"
if [ ! -f server.pem ]; then
    openssl req -newkey rsa:2048 -sha256 -nodes \
        -keyout server.key -x509 -days 365 \
        -out server.pem \
        -subj "/C=RU/ST=Moscow/L=Moscow/O=Just4Fun/CN=${CN}"
fi
