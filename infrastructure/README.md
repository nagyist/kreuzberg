# Kreuzberg GKE Infrastructure

OpenTofu/Terraform infrastructure for GKE-based GitHub Actions runners.

## Configuration

- **Project**: kreuzberg-481219
- **Region**: europe-west3 (Frankfurt, Germany)
- **Purpose**: Self-hosted GitHub Actions runners for Docker builds and Rust compilation

## Prerequisites

```bash
# Install tools
task terraform:install

# Authenticate to GCP
gcloud auth application-default login
gcloud config set project kreuzberg-481219

# Set GitHub App credentials (required for runner registration)
export TF_VAR_gh_app_id="123456"
export TF_VAR_gh_app_installation_id="78910"
export TF_VAR_gh_app_private_key="$(cat path/to/private-key.pem)"
```

## Usage

```bash
cd infrastructure
tofu init
tofu plan
tofu apply
```

## Validation

```bash
task terraform:validate  # Validate configuration
task terraform:lint      # Lint with tflint
task terraform:fmt       # Format files
```
