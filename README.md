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

## Installation

### From Windows Store

1. Visit the Microsoft Store
2. Search for "Smart Terminal"
3. Choose your subscription tier
4. Install the application

### From Release

1. Download the latest release from GitHub Releases
2. Extract the ZIP file
3. Run `smart-terminal.exe`

### Building from Source

Prerequisites:
- Rust 1.70 or higher
- Cargo
- Windows SDK (for Windows builds)
- Xcode (for macOS builds)

```bash
# Clone the repository
git clone https://github.com/yourusername/smart-terminal
cd smart-terminal

# Create .env file with required environment variables:
cat > .env << EOL
STORE_PRODUCT_ID=your-windows-store-product-id
API_BASE_URL=https://smart-terminal-api-prod.azurewebsites.net
EOL

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Configuration

The application requires two environment variables:

```env
# The Windows Store product ID for your application
STORE_PRODUCT_ID=your-windows-store-product-id

# The Azure Functions backend URL
API_BASE_URL=https://smart-terminal-api-prod.azurewebsites.net
```

These can be set either in a `.env` file in the project root or as system environment variables.

## Usage

### Basic Commands

- `i` - Enter input mode
- `Esc` - Exit input mode
- `/help` - Show help message
- `/clear` - Clear screen
- `/upgrade` - Open upgrade dialog
- `q` - Quit application

### Navigation (Normal Mode)

- `j` - Scroll down
- `k` - Scroll up
- Arrow keys - Navigate history

### Text Editing (Input Mode)

- `Ctrl+W` - Delete word
- `Ctrl+U` - Clear line
- Arrow keys - Move cursor
- `Up/Down` - Navigate command history

## Subscription Tiers

### Free Tier
- 50 AI queries per month
- Basic command assistance

### Basic Tier ($4.99/month)
- 500 AI queries per month
- Enhanced command suggestions
- Priority support

### Pro Tier ($9.99/month)
- 2,000 AI queries per month
- Advanced AI features
- Premium support
- Custom configurations

### Enterprise Tier ($49.99/month)
- 10,000 AI queries per month
- Dedicated support
- Custom integrations
- Team management

## Development

### Project Structure

```
smart-terminal/
├── src/
│   └── main.rs         # Main application code
├── Cargo.toml          # Project dependencies
├── .env                # Environment configuration
└── README.md          # This file
```

### Building for Windows Store

1. Register as a Microsoft Partner Center developer
2. Set up your app listing and subscription products
3. Get your product ID from the Partner Center
4. Build the MSIX package:
```bash
cargo build --release
# Package creation steps in infrastructure/windows/package.ps1
```

## Infrastructure

The project uses:
- Azure Functions for backend processing
- Azure Key Vault for secret management
- Azure Table Storage for usage tracking
- Windows Store for licensing and payments

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- GitHub Issues for bug reports and feature requests
- Email support for paid tiers
- Documentation at [docs.smartterminal.dev](https://docs.smartterminal.dev)

## Security

Please report security vulnerabilities to security@smartterminal.dev or via GitHub's security advisories.

## Roadmap

- [ ] Linux support
- [ ] Custom themes
- [ ] Plugin system
- [ ] Team collaboration features
- [ ] Advanced AI workflows
- [ ] Cloud synchronization

## FAQ

**Q: How do I upgrade my subscription?**
A: Use the `/upgrade` command or visit the Windows Store page.

**Q: Can I use my own Claude API key?**
A: Currently not supported. All API calls are managed through our secure backend.

**Q: Is there an offline mode?**
A: Not currently, but it's on our roadmap.

**Q: Where do I find my Windows Store Product ID?**
A: After registering your application in the Microsoft Partner Center, you can find the Product ID in your app's listing details.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For available versions, see the [tags on this repository](https://github.com/yourusername/smart-terminal/tags).
