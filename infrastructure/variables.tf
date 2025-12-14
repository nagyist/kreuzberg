variable "project_id" {
  description = "GCP project ID"
  type        = string
  default     = "kreuzberg-481219"
}

variable "region" {
  description = "GCP region for resources"
  type        = string
  default     = "europe-west3" # Frankfurt
}

variable "cluster_name" {
  description = "Name of the GKE cluster"
  type        = string
  default     = "kreuzberg-gke-runners"
}

variable "github_owner" {
  description = "GitHub organization or user name"
  type        = string
  default     = "kreuzberg-dev"
}

variable "github_repo" {
  description = "GitHub repository name"
  type        = string
  default     = "kreuzberg"
}

variable "gh_app_id" {
  description = "GitHub App ID for runner registration (from TF_VAR_gh_app_id)"
  type        = string
}

variable "gh_app_installation_id" {
  description = "GitHub App Installation ID (from TF_VAR_gh_app_installation_id)"
  type        = string
}

variable "gh_app_private_key" {
  description = "GitHub App Private Key (from TF_VAR_gh_app_private_key)"
  type        = string
  sensitive   = true
}
