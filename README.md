# 🦀 Rust Codespace Setup (Linux)

This repository provides a **ready-to-use Rust development environment** using **GitHub Codespaces** on Linux.

No local Rust installation is required.

---

## 🚀 What’s Included

- Rust (`rustc`)
- Cargo (Rust package manager)
- Rust Analyzer (VS Code extension)
- Linux-based development container

All tools are installed automatically via a **devcontainer**.

---

## 📁 Project Structure
├── .devcontainer/
│ └── devcontainer.json
├── hello_rust/
│ ├── src/
│ │ └── main.rs
│ ├── Cargo.toml
│ └── target/ # ignored by git
├── .gitignore
└── README.md
```json
{
  "name": "Rust Codespace",
  "image": "mcr.microsoft.com/devcontainers/rust:latest",
  "postCreateCommand": "rustc --version && cargo --version",
  "customizations": {
    "vscode": {
      "extensions": ["rust-lang.rust-analyzer"]
    }
  }
}
```