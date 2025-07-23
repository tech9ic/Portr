<div align="center">
<pre>
╔════════════════════════════════════════════╗
║ ██████╗  ██████╗ ██████╗ ████████╗██████╗  ║
║ ██╔══██╗██╔═══██╗██╔══██╗╚══██╔══╝██╔══██╗ ║
║ ██████╔╝██║   ██║██████╔╝   ██║   ██████╔╝ ║
║ ██╔═══╝ ██║   ██║██╔══██╗   ██║   ██╔══██╗ ║
║ ██║     ╚██████╔╝██║  ██║   ██║   ██║  ██║ ║
║ ╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝ ║
╚════════════════════════════════════════════╝

   by NeRDs — **Ne**twork **R**isk **D**efender**s**
</pre>
</div>


**Portr** is a beginner-friendly, terminal-based tool designed to monitor **outbound network connections on Windows**. Built in **Rust**, Portr helps you visualize which processes are making network connections from your machine, with a strong emphasis on **clarity**, **security**, and **educational value**.

---

## 🚦 What Makes Portr Different from `netstat`?

| Feature                | `netstat`                 | **Portr**                               |
| ---------------------- | ------------------------- | --------------------------------------- |
| **Target Audience**    | Advanced users, sysadmins | Beginners, learners, everyone           |
| **Output Style**       | Static, raw text          | Real-time, interactive TUI              |
| **Process Info**       | PID (sometimes)           | PID + Process Name                      |
| **DNS Resolution**     | Optional                  | _Planned_                               |
| **Highlighting**       | None                      | Blacklist/Whitelist/Suspicious Ports ⚠️ |
| **User Customization** | None                      | Yes (User-defined, learns over time)    |
| **Self-Learning**      | ❌ No                     | ✅ Yes                                  |
| **Security Awareness** | ❌ No                     | ✅ Yes                                  |
| **Educational Value**  | ❌ Low                    | ✅ High                                 |
| **Extensibility**      | Limited                   | High (Open-source in Rust ❤️)           |
| **Platform Support**   | Most OSes                 | Windows (for now)                       |

---

## ✨ Features

- **Live TUI:** See live-updating outbound TCP/UDP connections.
- **Full Connection Info:** Includes local IP/port, remote IP/port, PID, and process name.
- **Smart Highlighting:**
  - 🔴 **Blacklisted connections**
  - 🟢 **Whitelisted connections**
  - 🟡 **Suspicious ports**
- **Interactive List Management:**
  - Press `b` to blacklist a selected connection.
  - Press `w` to whitelist.
- **Footer Help:** Always shows `"Press 'q' or Esc to quit"`.
- **Clean Lists:** No duplicates in blacklist or whitelist.
- **Beginner-first Design:** Clean, fast, intuitive.

---

## 📦 Installation

### 1. Install Rust

👉 [https://rustup.rs](https://rustup.rs)

### 2. Clone & Build

```sh
git clone <your-portr-repo-url>
cd portr-windows
cargo build --release
```

### 3. Run Portr

```sh
cargo run --release
```

Or run the binary directly from:

```
target/release/portr-windows.exe
```

---

## 🚀 Usage

Launch Portr in a terminal. You'll see live outbound connections in a navigable table.

### Keybindings

| Key         | Action                                   |
| ----------- | ---------------------------------------- |
| `↑ / ↓`     | Move selection                           |
| `b`         | Add selected remote IP/port to blacklist |
| `w`         | Add selected remote IP/port to whitelist |
| `q` / `Esc` | Quit Portr                               |

> ⚡ You’ll see confirmation messages after actions.  
> ℹ️ Help message is always shown in the footer.

---

## ⚙️ Configuration

Portr uses two editable plain text files:

- **`blacklist.txt`** — List IPs, domains, or ports to block
- **`whitelist.txt`** — List IPs, domains, or ports to allow

### Notes:

- One entry per line.
- Lines starting with `#` or empty lines are ignored.
- You can modify these manually or interactively in the TUI.

---

## 🛡️ Suspicious Ports

Portr flags outbound connections to suspicious ports (commonly used by malware or hackers):

Examples:

```
23, 6667, 31337, 12345, 54321, 4444
```

⚠️ These are hardcoded for now — but you can customize them in the source.

---

## 📝 Roadmap

- [ ] DNS resolution
- [ ] Log/export connections to file
- [ ] Filter/sort/search/scroll inside TUI
- [ ] Remove from lists within the TUI
- [ ] Cross-platform support (Linux/macOS)
- [ ] Threat intelligence feed integration

---

## 🤝 Contributing

We welcome contributions, bug reports, and feature suggestions!

Feel free to:

- Fork the repo
- Open issues
- Submit pull requests

---

## 📄 License

MIT License — See [`LICENSE`](LICENSE) file for more details.

---

> **Portr** — Making network visibility **simple**, **secure**, and **educational**.  
> Built with 💛 by **tech9ic @ NeRDs (Network Risk Defenders)**
