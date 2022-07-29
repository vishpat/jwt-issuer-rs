# JWT Issuer 

The [jwt-issuer-rs](https://github.com/vishpat/jwt-issuer-rs) project implements a JWT (JSON Web Token) Issuer in Rust using [actix](https://actix.rs/), [jsonwebtoken](https://crates.io/crates/jsonwebtoken) and [jsonwebkey](https://crates.io/crates/jsonwebkey).

## Key generation

The first step in using the JWT issuer is to generate a key pair using openssl to sign the JWTs. 

```
openssl ecparam -name prime256v1 -genkey -noout -out key.pem
openssl ec -in key.pem -pubout -out public.pem
```

The next step is to convert the private key into a [JWK](https://datatracker.ietf.org/doc/html/rfc7517)

```
cat key.pem | docker run -i danedmunds/pem-to-jwk:latest > key.json
```

## Usage 

Using your JWK stored in a file called **key.json**, run the JWT Issuer service as follows

```
cargo run
```

The above command starts a HTTP server which exposes two endpoints

- **auth/token**: JWT token issuer
- **auth/jwks**: The public JSON Web Keys

A JWT token for a user can be requested using the following curl command

```
curl -X POST http://localhost:8080/auth/token -d '{"name": "vishpat"}' -H 'Content-Type: application/json'
```
(In the real world this endpoint would have authentication and would be exposed via https. However, for simplicity, this is not the case)

The issued JWT can then be verified against the jwks endpoint as follows

```python
# A simple python program to verify the JWT using the 
# JWKS endpoint. 
# Requires python packages pyjwt,requests,cryptography

import jwt
import requests

token = "TOKEN"
jwk_url = "http://localhost:8080/auth/jwks"

jwk = requests.get(jwk_url).json()
key = jwt.algorithms.ECAlgorithm.from_jwk(jwk["keys"][0])
data = jwt.decode(token, key=key, algorithms=['ES256'])
print(data)
```