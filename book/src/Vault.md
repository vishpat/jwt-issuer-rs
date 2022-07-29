# Hashicorp Vault Integration

## Vault Overview

Hashicorp [Vault](https://www.vaultproject.io/) is an enterprise grade secret management system. Vault enables storing of secrets and accessing them using various authentication and authorization mechanisms. 

We will be using Vault to store a username and password in it's [kv secret store](https://www.vaultproject.io/docs/secrets/kv). The JWT Issuer will be used to setup [JWT authentication](https://www.vaultproject.io/docs/auth/jwt) to access the secrets stored in the Vault.

## Vault Server
Start a local dev vault server
```
vault server -dev
```

## Key Value Secret Engine

The above command will spit out the admin token that is required to manage the Vault instance. Use the admin token to enable the kv (version 1) secret engine for Vault as shown below.

```
export VAULT_ADDR='http://127.0.0.1:8200'
export VAULT_TOKEN=<VAULT_ADMIN_TOKEN>
vault secrets enable -version=1 kv
```

## Authentication Setup

The next step is to setup the JWT authentication for Vault. This can be done using terraform as follows. 

```terraform
provider "vault" {
}

resource "vault_jwt_auth_backend" "vishpat" {
    description         = "Enable JWT auth"
    path                = "jwt"
    default_role        = "jwt_vishpat"
    jwks_url            = "http://localhost:8080/auth/jwks"
}

resource "vault_jwt_auth_backend_role" "jwt_vishpat" {
  backend        = vault_jwt_auth_backend.vishpat.path
  role_name      = "jwt_vishpat"
  token_policies = ["vishpat_keys_policy"]

  bound_subject   = "vishpat"
  user_claim      = "sub"
  role_type       = "jwt"
}
```

## Policy setup
Now setup a policy (using terraform) in Vault to allow the user **vishpat** to access the secrets stored under **kv/vishpat/***

```terraform
resource "vault_policy" "vishpat_keys_policy" {
  name   = "vishpat_keys_policy"
  policy = <<EOT

path "kv/vishpat/*" {
    capabilities = [ "create", "read", "update", "delete", "list" ]
}

                EOT
}
```

## Terraform 
Apply the terraform plan as shown below

```bash
terraform plan -out plan.out
terraform apply plan.out
```

## JWT token 
Now get the JWT token for the user **vishpat** as shown below.

```bash
curl -X POST -d '{"name": "vishpat"}'  -H 'Content-Type: application/json'   http://localhost:8080/auth/token
```

## Vault authentication
Use the JWT token to authenticate against Vault inorder to get the Vault token to access the secrets.

```bash
vault write auth/jwt/login jwt=<JWT_TOKEN>

Key                  Value
---                  -----
token                hvs.CAESIDZZ-ol5ich3gnllpS6yhuaHDVnqEXwZUiRNlKqBslpIGh4KHGh2cy5pbURralBsZlhTdTRSdlBnMkRjVDJDQng
token_accessor       LY5IENUHKtxvugEiSAiLalH9
token_duration       768h
token_renewable      true
token_policies       ["default" "vishpat_keys_policy"]
identity_policies    []
policies             ["default" "vishpat_keys_policy"]
token_meta_role      jwt_vishpat
```

## Access secrets

The secrets at **kv/vishpat** can now be accessed using the Vault user token.

```bash
export VAULT_TOKEN=hvs.CAESIDZZ-ol5ich3gnllpS6yhuaHDVnqEXwZUiRNlKqBslpIGh4KHGh2cy5pbURralBsZlhTdTRSdlBnMkRjVDJDQng

➜ vault kv put kv/vishpat/test username=X password=Y
Success! Data written to: kv/vishpat/test

➜ vault kv get kv/vishpat/test
=== Data ===
Key      Value
---      -----
password   Y
username   X
```