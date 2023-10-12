

openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 \
  -nodes -keyout test/data/cert/key.pem -out test/data/cert/cert.pem -subj "/CN=127.0.0.1"
#   -addext "subjectAltName=DNS:lib.wrenger.net,DNS:*.lib.wrenger.net,IP:127.0.0.1"
