output "region" {
  description = "GCP region"
  value       = var.region
}

output "project_id" {
  description = "GCP project ID"
  value       = var.project_id
}

output "cluster_name" {
  description = "GKE cluster name"
  value       = module.github_actions_runner.cluster_name
}

output "cluster_location" {
  description = "GKE cluster location"
  value       = module.github_actions_runner.location
}

output "network_name" {
  description = "VPC network name"
  value       = module.github_actions_runner.network_name
}

output "subnet_name" {
  description = "VPC subnet name"
  value       = module.github_actions_runner.subnet_name
}
