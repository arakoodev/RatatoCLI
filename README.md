# RatatoCLI
Named after Ratatoskr -  is a squirrel who runs up and down the world tree Yggdrasil to carry messages between the eagles perched atop it and the serpent Níðhöggr who dwells beneath one of the three roots of the tree.


An AI-powered terminal application with Claude integration, built for Windows and macOS. RatatoCLI combines the power of a traditional command-line interface with AI assistance to enhance developer productivity.

## Features

- 🤖 AI-powered command assistance using Claude
- 🎨 Modern terminal user interface
- ⌨️ Vim-style modal editing
- 📜 Command history and navigation
- 🔄 Automatic session management
- 🎯 Multi-tier subscription support
- 🔐 Windows Store integration
- ⚡ Azure Functions backend integration

# Smart Terminal

An AI-powered terminal application with Claude integration and Windows Store licensing.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Project Setup](#project-setup)
- [Local Development](#local-development)
- [Azure Backend Setup](#azure-backend-setup)
- [Building the Windows Package](#building-the-windows-package)
- [Windows Store Publishing](#windows-store-publishing)
- [Architecture](#architecture)

## Prerequisites

### Development Tools
- Rust (latest stable version)
- Node.js 18+
- Azure CLI
- Windows 10/11 SDK
- Visual Studio 2019 or newer with:
  - .NET Desktop Development workload
  - Universal Windows Platform development workload
- Windows Store Developer Account

### Required Software
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Windows SDK (if not already installed)
winget install Microsoft.WindowsSDK

# Install Azure Functions Core Tools
npm install -g azure-functions-core-tools@4

# Install Terraform
choco install terraform

# Install WiX Toolset
choco install wixtoolset
```

## Project Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/smart-terminal
cd smart-terminal
```

2. Create environment files:

`.env` for the terminal:
```env
API_BASE_URL=https://your-azure-function-url
STORE_PRODUCT_ID=your-windows-store-product-id
```

`azure/terraform.tfvars`:
```hcl
environment = "prod"
location = "eastus"
anthropic_api_key = "your-key-here"
microsoft_auth_secret = "your-store-secret"
```

## Local Development

1. Build and run the terminal:
```bash
cargo build
cargo run
```

2. Run the Azure Function locally:
```bash
cd azure
npm install
func start
```

## Azure Backend Setup

1. Initialize Terraform:
```bash
cd azure
terraform init
```

2. Deploy infrastructure:
```bash
terraform plan
terraform apply
```

3. Deploy the Function:
```bash
func azure functionapp publish smart-terminal-api
```

## Building the Windows Package

1. Configure version information in `Cargo.toml`:
```toml
[package]
version = "1.0.0"
```

2. Build release version:
```bash
cargo build --release
```

3. Create application manifest (AppxManifest.xml):
```xml
<?xml version="1.0" encoding="utf-8"?>
<Package
  xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
  xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
  xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities">
  
  <Identity
    Name="YourCompany.SmartTerminal"
    Publisher="CN=YourPublisherID"
    Version="1.0.0.0" />
    
  <!-- ... rest of the manifest ... -->
</Package>
```

4. Prepare assets:
```
assets/
  ├── Square150x150Logo.png
  ├── Square44x44Logo.png
  ├── StoreLogo.png
  └── Wide310x150Logo.png
```

5. Create app package:
```bash
# Create directory structure
mkdir AppPackages
mkdir AppPackages/SmartTerminal
copy target/release/smart-terminal.exe AppPackages/SmartTerminal/
copy AppxManifest.xml AppPackages/SmartTerminal/
xcopy /E assets AppPackages/SmartTerminal/assets/

# Create MSIX package
makeappx.exe pack /d AppPackages/SmartTerminal /p SmartTerminal.msix
```

6. Sign the package:
```bash
# Create certificate (if you don't have one)
New-SelfSignedCertificate -Type Custom -Subject "CN=YourCompany" -KeyUsage DigitalSignature -FriendlyName "Smart Terminal Certificate" -CertStoreLocation "Cert:\CurrentUser\My" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")

# Export certificate
$password = ConvertTo-SecureString -String "YourPassword" -Force -AsPlainText
Export-PfxCertificate -cert "Cert:\CurrentUser\My\<certificate-thumbprint>" -FilePath SmartTerminal.pfx -Password $password

# Sign package
signtool.exe sign /fd SHA256 /a /f SmartTerminal.pfx /p YourPassword SmartTerminal.msix
```

## Windows Store Publishing

1. Register in Partner Center:
   - Go to [Partner Center](https://partner.microsoft.com/dashboard)
   - Complete account setup
   - Pay registration fee (if required)

2. Create app submission:
   - Go to "Apps and Games"
   - Click "Create a new app"
   - Reserve app name
   - Set pricing and availability
   - Configure Store listing
   - Set up age ratings

3. Configure product offerings:
```
Subscription Tiers:
- Free: 50 queries/month
- Basic: 500 queries/month ($4.99/month)
- Pro: 2000 queries/month ($9.99/month)
- Enterprise: 10000 queries/month ($49.99/month)
```

4. Submit for certification:
   - Upload signed .msix package
   - Complete store listing
   - Provide testing notes
   - Submit for review

5. Post-submission:
   - Monitor certification status
   - Address any certification feedback
   - Prepare for launch

## Architecture

### Terminal Application
- Rust-based TUI using Ratatui
- Windows Store licensing integration
- Azure Function API integration
- Local storage for preferences and history

### Azure Backend
- Azure Functions for API handling
- Key Vault for secret management
- Table Storage for usage tracking
- Store authentication validation

### Windows Store Integration
- License validation
- Subscription management
- Usage quota enforcement
- Automatic updates

## Troubleshooting

### Common Issues

1. Package Signing Errors:
```bash
# Check certificate validity
certutil -verify SmartTerminal.pfx

# Verify package
signtool.exe verify /pa SmartTerminal.msix
```

2. Store Submission Errors:
- Ensure all required capabilities are declared in manifest
- Verify assets meet size requirements
- Check that version numbers match across all files

3. Azure Function Issues:
```bash
# Check function logs
func azure functionapp logstream smart-terminal-api
```

## License
This repo is under a AGPL-3.0 license with a CLA.
