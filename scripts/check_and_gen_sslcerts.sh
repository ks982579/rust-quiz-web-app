#!/bin/bash
# NOTE: This script is consumed by compose-local.yaml Docker Compose

# Check if certificates exist and are valid
if [ ! -f "/certs/fullchain.pem" ] || [ ! -f "/certs/privkey.pem"] || \
  ! openssl x509 -noout -in /certs/fullchain.pem -checkend 86400 > /dev/null 2>&1
then
  # Generate new certificates if not valid
  echo "Generating new SSL certificates"
  openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout privkey.pem -out fullchain.pem \
    -subj "/C=IE/ST=Cork/L=Cork/O=IU/CN=quiztestapp.io"
  echo "Certificates generated"
else
  echo "Existing SSL certificates still valid"
fi
