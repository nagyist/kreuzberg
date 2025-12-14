# Kreuzberg GKE GitHub Actions Runners - Frankfurt (europe-west3)

provider "google" {
  project = var.project_id
  region  = var.region
}

provider "google-beta" {
  project = var.project_id
  region  = var.region
}

# GitHub Actions self-hosted runners on GKE
module "github_actions_runner" {
  source  = "registry.terraform.io/terraform-google-modules/github-actions-runners/google//modules/gh-runner-gke"
  version = "~> 5.0"

  project_id = var.project_id
  region     = var.region

  # GitHub App authentication (from environment variables)
  gh_app_id              = var.gh_app_id
  gh_app_installation_id = var.gh_app_installation_id
  gh_app_private_key     = var.gh_app_private_key
  gh_config_url          = "https://github.com/${var.github_owner}/${var.github_repo}"

  # GKE cluster configuration
  cluster_suffix = var.cluster_name

  # Node pool configuration (Frankfurt zones)
  zones          = ["europe-west3-a", "europe-west3-b"]
  machine_type   = "n2-standard-4"
  min_node_count = 2
  max_node_count = 16
}
