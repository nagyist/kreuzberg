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

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  default     = "prod"
}
