# main.tf
terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

provider "azurerm" {
  features {
    key_vault {
      purge_soft_delete_on_destroy = true
    }
  }
}

# Resource group
resource "azurerm_resource_group" "main" {
  name     = "smart-terminal-${var.environment}"
  location = var.location
}

# Storage account for Function
resource "azurerm_storage_account" "main" {
  name                     = "smartterminal${var.environment}"
  resource_group_name      = azurerm_resource_group.main.name
  location                 = azurerm_resource_group.main.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

# Table for usage tracking
resource "azurerm_storage_table" "usage" {
  name                 = "usageQuota"
  storage_account_name = azurerm_storage_account.main.name
}

# App Service Plan
resource "azurerm_service_plan" "main" {
  name                = "smart-terminal-plan-${var.environment}"
  resource_group_name = azurerm_resource_group.main.name
  location           = azurerm_resource_group.main.location
  os_type            = "Windows"
  sku_name           = "Y1" # Consumption plan
}

# Function App
resource "azurerm_windows_function_app" "main" {
  name                       = "smart-terminal-api-${var.environment}"
  resource_group_name        = azurerm_resource_group.main.name
  location                   = azurerm_resource_group.main.location
  service_plan_id            = azurerm_service_plan.main.id
  storage_account_name       = azurerm_storage_account.main.name
  storage_account_access_key = azurerm_storage_account.main.primary_access_key

  site_config {
    application_stack {
      node_version = "18"
    }
    cors {
      allowed_origins = ["https://login.microsoftonline.com", "ms-windows-store://"]
    }
  }

  app_settings = {
    "WEBSITE_RUN_FROM_PACKAGE"    = "1"
    "KEY_VAULT_NAME"              = azurerm_key_vault.main.name
    "ANTHROPIC_API_ENDPOINT"      = "https://api.anthropic.com/v1"
    "MICROSOFT_PROVIDER_AUTHENTICATION_SECRET" = var.microsoft_auth_secret
  }

  identity {
    type = "SystemAssigned"
  }
}

# Key Vault
resource "azurerm_key_vault" "main" {
  name                       = "smart-terminal-kv-${var.environment}"
  location                   = azurerm_resource_group.main.location
  resource_group_name        = azurerm_resource_group.main.name
  tenant_id                  = data.azurerm_client_config.current.tenant_id
  sku_name                   = "standard"
  soft_delete_retention_days = 7
  purge_protection_enabled   = false
}

# Key Vault Access Policy for Function App
resource "azurerm_key_vault_access_policy" "function" {
  key_vault_id = azurerm_key_vault.main.id
  tenant_id    = data.azurerm_client_config.current.tenant_id
  object_id    = azurerm_windows_function_app.main.identity[0].principal_id

  secret_permissions = [
    "Get", "List"
  ]
}

# Store Anthropic API Key in Key Vault
resource "azurerm_key_vault_secret" "anthropic_key" {
  name         = "AnthropicKey"
  value        = var.anthropic_api_key
  key_vault_id = azurerm_key_vault.main.id

  depends_on = [
    azurerm_key_vault_access_policy.function
  ]
}

# variables.tf
variable "environment" {
  description = "Environment name (dev, prod, etc.)"
  type        = string
}

variable "location" {
  description = "Azure region"
  type        = string
  default     = "eastus"
}

variable "anthropic_api_key" {
  description = "Anthropic API Key"
  type        = string
  sensitive   = true
}

variable "microsoft_auth_secret" {
  description = "Microsoft Store Authentication Secret"
  type        = string
  sensitive   = true
}

# outputs.tf
output "function_app_url" {
  value = "https://${azurerm_windows_function_app.main.default_hostname}"
}

output "key_vault_name" {
  value = azurerm_key_vault.main.name
}

# terraform.tfvars.example
environment = "dev"
location = "eastus"
# anthropic_api_key = "your-key-here"
# microsoft_auth_secret = "your-secret-here"
