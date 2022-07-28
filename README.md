# JWT Issuer in Rust

This project implements a JWT Issuer in Rust using [actix](https://actix.rs/), [jsonwebtoken](https://crates.io/crates/jsonwebtoken) and [jsonwebkey](https://crates.io/crates/jsonwebkey).

# Functionality

Store your JWK (JSON Web Key) in **key.json** and run the JWT Issuer service as follows

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

