#!/bin/bash

cd "$(dirname "$0")" || exit 1

# Define the file names
KEY_FILE="localhost.key.pem"
CERT_FILE="localhost.crt.pem"
TMP_FILE="localhost.tmp.pem"

# Generate an RSA private key in PKCS#8 format
openssl genpkey -algorithm RSA -out "$TMP_FILE"

# Convert the key to traditional RSA format (PKCS#1)
openssl rsa -in "$TMP_FILE" -out "$KEY_FILE" -traditional

# Remove the intermediate PKCS#8 key file
rm "$TMP_FILE"

# Generate a self-signed certificate good for 20 years
openssl req -new -x509 -nodes -key "$KEY_FILE" -days 7300 -out "$CERT_FILE" -subj "/CN=localhost" -addext "subjectAltName = DNS:localhost"

# Display message
echo "RSA private key saved to $KEY_FILE"
echo "Self-signed certificate saved to $CERT_FILE"
