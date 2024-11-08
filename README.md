# GCD - Git Repository Quick Navigation Tool

GCD is a powerful command-line tool written in Rust designed to make navigating your Git repositories fast and easy. By indexing your repositories and allowing fuzzy search, GCD lets you jump into any project folder with minimal keystrokes. Shell integration is available for all major shells, so you can use it seamlessly in your preferred environment.

---

## 🌟 Features

- **Lightning-Fast Navigation:** Instantly navigate to your Git repositories with fuzzy search matching.
- **Shell Integration:** Supports Bash, Zsh, Fish, and PowerShell for an uninterrupted terminal experience.
- **Intuitive Commands:** Just type `gcd` with a repository name pattern to find and move to your project in seconds.

## 🚀 Installation

### Quick install

```bash
grip registry add github.com/tristanpoland/My-Grip-Registry
grip install gcd
```

### Source Install

1. **Clone the repository** and navigate to the project directory:
   ```bash
   git clone https://github.com/tristanpoland/GCD
   cd gcd
   ```

2. **Build the project** using Cargo:
   ```bash
   cargo build --release
   ```

3. **Install GCD** by moving the binary to a directory in your PATH:
   ```bash
   cp target/release/gcd /usr/local/bin
   ```

## 🔧 Shell Integration

To use `gcd` seamlessly from any shell, you can install shell integration for your preferred shell:

### Bash, Zsh, Fish, and PowerShell

```bash
gcd install <shell>
```

Replace `<shell>` with `bash`, `zsh`, `fish`, or `ps` for PowerShell. The integration script will be automatically added to your shell's configuration file.

## 🖥️ Usage

### Index Your Repositories

First, tell `gcd` where to find your Git repositories:
```bash
gcd index /path/to/your/repositories
```

### Navigate to a Repository

To navigate, simply provide a name pattern:
```bash
gcd <repo_name_pattern>
```

If there’s a match, `gcd` will take you directly to that repository!

## 🌈 Examples

1. **Indexing repositories:**
   ```bash
   gcd index ~/projects
   ```
   This command will scan your `~/projects` folder for Git repositories and save them in GCD’s index.

2. **Navigating to a repository:**
   ```bash
   gcd awesome-project
   ```
   GCD will find and change to the directory of `awesome-project` if it exists.

## 🛠️ Configuration

GCD stores its configuration and index in `~/.config/gcd/config.json`. You can manually edit this file if needed, but it’s usually managed automatically.

## 🤝 Contributing

1. **Fork the repository** and create your branch.
2. **Implement your feature or fix**.
3. **Submit a pull request**.

We welcome contributions! Please check the issues section for suggestions or bugs to help fix.

## 📜 License

This project is licensed under the MIT License.

---

✨ Happy Navigating!
