# tikee GitOps / IaC contract

The management plane exposes declarative drift control without hiding changes behind migration-style compatibility tools:

- `GET /api/v1/gitops/manifest?format=yaml|json` exports current Job, Workflow, Script, Plugin and AlertRule resources.
- `POST /api/v1/gitops/diff` accepts a desired `TikeeManifest` JSON document and returns create/update/delete/unchanged changes with unified text diffs.
- Resource identity is `kind/namespace/app/name`; checksums are based on canonical JSON.
- Bulk apply is intentionally not implicit here. CI/CD should review diff output, then call typed CRUD APIs for approved changes.

See:

- `deploy/gitops/tikee-manifest.example.yaml`
- `deploy/k8s/crd/tikee-manifest-crd.yaml`
- `deploy/terraform/tikee_gitops_manifest.tf`
