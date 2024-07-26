#!/usr/bin/env sh

privatekey=$(openssl genpkey -algorithm Ed25519)
publickey=$(echo "$privatekey" | openssl pkey -pubout)

echo "Public key:\n"
echo $publickey
echo ""
echo "Public key base64:\n"
echo $publickey | base64
echo ""
echo "Private key:\n"
echo $privatekey
echo ""
echo "Private key base64:\n"
echo $privatekey | base64
