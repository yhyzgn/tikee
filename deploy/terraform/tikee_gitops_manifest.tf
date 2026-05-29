terraform {
  required_version = ">= 1.6.0"
}

variable "tikee_api_base" {
  type = string
}

variable "tikee_api_token" {
  type      = string
  sensitive = true
}

variable "manifest_file" {
  type    = string
  default = "../gitops/tikee-manifest.example.yaml"
}

# Provider implementation is intentionally external to this repository for now;
# this module documents the stable management-plane contract used by CI/CD:
#   GET  /api/v1/gitops/manifest?format=yaml
#   POST /api/v1/gitops/diff
# Apply remains controlled by the normal typed CRUD APIs to avoid unreviewed bulk mutation.
output "gitops_contract" {
  value = {
    export = "${var.tikee_api_base}/api/v1/gitops/manifest?format=yaml"
    diff   = "${var.tikee_api_base}/api/v1/gitops/diff"
  }
}
