provider "vault" {
}

resource "vault_jwt_auth_backend" "jwt_auth" {
    description         = "Enable JWT auth"
    path                = "jwt"
    default_role        = "jwt_vishpat"
    jwks_url            = "http://localhost:8080/auth/jwks"
}

resource "vault_jwt_auth_backend_role" "jwt_vishpat" {
  backend        = vault_jwt_auth_backend.jwt_auth.path
  role_name      = "jwt_vishpat"
  token_policies = ["vishpat_keys_policy"]


  bound_subject   = "vishpat"
  user_claim      = "sub"
  role_type       = "jwt"
}

resource "vault_policy" "vishpat_keys_policy" {
  name   = "vishpat_keys_policy"
  policy = <<EOT

path "kv/vishpat/*" {
    capabilities = [ "create", "read", "update", "delete", "list" ]
}

                EOT
}

