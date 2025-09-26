# NetSnip - Modern Network Management for macOS

![NetSnip Banner](https://img.shields.io/badge/NetSnip-v1.0.0-6C5CE7)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey)
![License](https://img.shields.io/badge/license-MIT-blue)
![Build](https://img.shields.io/badge/build-passing-brightgreen)

NetSnip is a powerful, native macOS network management application that provides users with intuitive control over their local network. Built with Tauri, Rust, and React, it offers a beautiful neumorphic interface that makes network management accessible to everyday users while providing advanced capabilities for power users.

## ✨ Features

### Core Features (Free)
- 🔍 **Network Discovery** - Automatically scan and discover all devices on your network
- 🎯 **Device Management** - Cut/restore device connections with one click
- 📊 **Real-time Monitoring** - Track bandwidth usage and network activity
- 🏷️ **Device Labeling** - Custom names and notes for easy identification
- 🛡️ **Safety Features** - Built-in protection against self-disconnection
- 🎨 **Beautiful UI** - Modern neumorphic design with dark mode support

### Premium Features
- ⚡ **Speed Control** - Limit bandwidth for specific devices
- 📈 **Advanced Analytics** - Detailed network usage history and statistics
- ⏰ **Scheduled Rules** - Time-based network restrictions
- 🔄 **Auto-refresh** - Real-time updates without manual scanning
- 📱 **Multiple Profiles** - Manage different network configurations
- 🔐 **Advanced Protection** - Enhanced security features

## 🚀 Quick Start

### Prerequisites
- macOS 11.0 or later
- Node.js 18+ and npm
- Rust 1.70+
- Xcode Command Line Tools

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/netsnip.git
cd netsnip
```

2. **Install dependencies**
```bash
npm install
```

3. **Build the application**
```bash
npm run tauri:build
```

4. **Run in development mode**
```bash
npm run tauri:dev
```

## 🏗️ Architecture

NetSnip is built with a modern, secure architecture:

```
┌─────────────────────────────────┐
│       React Frontend            │
│   (TypeScript + Tailwind CSS)   │
└────────────┬────────────────────┘
             │ IPC
┌────────────┴────────────────────┐
│       Tauri Backend             │
│         (Rust Core)             │
├─────────────────────────────────┤
│  • Network Scanner              │
│  • ARP Controller               │
│  • Bandwidth Manager            │
│  • SQLite Database              │
└────────────┬────────────────────┘
             │
┌────────────┴────────────────────┐
│    Privileged Helper            │
│   (Swift - Network Operations)  │
└─────────────────────────────────┘
```

## 🛠️ Technology Stack

- **Frontend**: React 18, TypeScript, Tailwind CSS, Zustand
- **Backend**: Rust, Tauri 2.0, Tokio, pnet
- **Database**: SQLite with sqlx
- **UI/UX**: Neumorphic design, Framer Motion animations
- **Security**: AES-256 encryption, sandboxed operations

## 📦 Project Structure

```
netsnip/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── commands/   # Tauri commands
│   │   ├── modules/    # Core functionality
│   │   └── utils/      # Helper functions
│   └── Cargo.toml
├── src/                # React frontend
│   ├── components/     # UI components
│   ├── stores/         # State management
│   ├── hooks/          # Custom React hooks
│   └── styles/         # CSS and themes
├── PrivilegedHelper/   # macOS privileged operations
└── package.json
```

## 🔒 Security

NetSnip takes security seriously:

- **Sandboxed Operations**: All network operations run in a secure sandbox
- **Encrypted Storage**: Sensitive data encrypted with AES-256
- **Self-Protection**: Built-in safeguards prevent accidental self-disconnection
- **Gateway Protection**: Warnings when attempting to modify gateway device
- **Local-Only**: No cloud connectivity by default (optional premium feature)

## 🧪 Development

### Running Tests
```bash
# Run Rust tests
cd src-tauri
cargo test

# Run React tests
npm test

# Run E2E tests
npm run test:e2e
```

### Building for Production
```bash
# Build for macOS
npm run tauri:build

# The built app will be in src-tauri/target/release/bundle/
```

### Code Style
- Rust: `cargo fmt` and `cargo clippy`
- TypeScript/React: ESLint and Prettier configured

## 📝 Configuration

NetSnip can be configured through the Settings panel or by editing the configuration file:

```json
{
  "theme": "light",
  "autoRefresh": false,
  "refreshInterval": 5,
  "notifications": true,
  "selfProtection": true,
  "gatewayProtection": true
}
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing framework
- [pnet](https://github.com/libpnet/libpnet) - Network packet manipulation
- [React](https://react.dev/) - UI framework
- [Tailwind CSS](https://tailwindcss.com/) - Styling framework

## 📞 Support

- **Documentation**: [docs.netsnip.app](https://docs.netsnip.app)
- **Issues**: [GitHub Issues](https://github.com/yourusername/netsnip/issues)
- **Email**: support@netsnip.app
- **Discord**: [Join our community](https://discord.gg/netsnip)

## 🚦 Roadmap

- [ ] Windows support
- [ ] Linux support
- [ ] Mobile companion app
- [ ] Cloud sync (optional)
- [ ] VPN detection and handling
- [ ] IPv6 support
- [ ] Network topology visualization

---

**⚠️ Disclaimer**: NetSnip is intended for legitimate network management on networks you own or have permission to manage. Misuse of this tool may violate laws and regulations. Use responsibly.

---

Made with ❤️ by the NetSnip Team