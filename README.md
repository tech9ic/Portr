\# Portr 
- by NeRDs (Network Risk Defenders)

===

# Portr is a beginner-friendly, terminal-based tool to monitor outbound network connections on Windows. Built in Rust, Portr helps you see which processes are making network connections from your machine, with a focus on clarity, security, and learning.

# 

# ---

# 

# \## 🚦 What Makes Portr Different from netstat?

# 

# | Feature                   | netstat                   | Portr (Your Tool)                     |

# | ------------------------- | ------------------------- | ------------------------------------- |

# | \*\*Target Audience\*\*       | Advanced users, sysadmins | Beginners, learners, everyone         |

# | \*\*Output Style\*\*          | Static, raw text          | Real-time, interactive TUI            |

# | \*\*Process Info\*\*          | PID (sometimes)           | PID + Process Name (always)           |

# | \*\*DNS Resolution\*\*        | Optional, mixed in output | (Planned)                             |

# | \*\*Highlighting/Flagging\*\* | None                      | Blacklist/Whitelist, suspicious ports |

# | \*\*User Customization\*\*    | None                      | User-defined lists (learns)           |

# | \*\*Self-Learning\*\*         | No                        | Yes (adapts to user input)            |

# | \*\*Security Awareness\*\*    | No                        | Yes (flags suspicious activity)       |

# | \*\*Educational Value\*\*     | Low                       | High (teaches as you use)             |

# | \*\*Extensibility\*\*         | Limited                   | High (open-source, Rust)              |

# | \*\*Platform Support\*\*      | Most OSes                 | Windows (this version)                |

# 

# ---

# 

# \## ✨ Features

# 

# \- Real-time, interactive terminal UI (TUI) with live network data

# \- See all outbound TCP/UDP connections, with local/remote IP, port, PID, and process name

# \- Highlight connections:

# &nbsp; - \*\*Red\*\*: Blacklisted IPs/ports (from `blacklist.txt`)

# &nbsp; - \*\*Green\*\*: Whitelisted IPs/ports (from `whitelist.txt`)

# &nbsp; - \*\*Yellow\*\*: Suspicious ports (commonly used by malware/hackers)

# \- Select rows with Up/Down keys

# \- Add remote IP/port to blacklist (`b`) or whitelist (`w`) directly from the TUI

# \- Confirmation messages for actions

# \- Permanent footer/help line: "Press 'q' or Esc to quit"

# \- Reads and updates `blacklist.txt` and `whitelist.txt` in real time

# \- No duplicate entries in lists

# \- Beginner-friendly, clear, and fast

# 

# ---

# 

# \## 📦 Installation

# 

# 1\. \*\*Install Rust\*\* (if you haven't):

# &nbsp;  https://rustup.rs/

# 

# 2\. \*\*Clone the repo and build:\*\*

# 

# &nbsp;  ```sh

# &nbsp;  git clone <your-portr-repo-url>

# &nbsp;  cd portr-windows

# &nbsp;  cargo build --release

# &nbsp;  ```

# 

# 3\. \*\*Run Portr:\*\*

# &nbsp;  ```sh

# &nbsp;  cargo run --release

# &nbsp;  ```

# &nbsp;  Or run the binary from `target/release/portr-windows.exe`.

# 

# ---

# 

# \## 🚀 Usage

# 

# \- Launch Portr in your terminal.

# \- The TUI will show a live-updating table of all outbound connections.

# \- Use \*\*Up/Down\*\* arrows to select a row.

# \- Press \*\*b\*\* to add the selected remote IP/port to the blacklist.

# \- Press \*\*w\*\* to add the selected remote IP/port to the whitelist.

# \- Press \*\*q\*\* or \*\*Esc\*\* to quit.

# \- Confirmation messages appear at the bottom for 2 seconds.

# \- The footer always shows how to quit.

# 

# \### \*\*Keybindings\*\*

# 

# | Key     | Action                                   |

# | ------- | ---------------------------------------- |

# | Up/Down | Move selection                           |

# | b       | Add selected remote IP/port to blacklist |

# | w       | Add selected remote IP/port to whitelist |

# | q / Esc | Quit Portr                               |

# 

# ---

# 

# \## ⚙️ Configuration

# 

# \- \*\*blacklist.txt\*\*: List of IPs, domains, or ports to flag as suspicious (one per line).

# \- \*\*whitelist.txt\*\*: List of IPs, domains, or ports to mark as trusted (one per line).

# \- You can edit these files manually or add entries from the TUI.

# \- Comments (lines starting with `#`) and blank lines are ignored.

# 

# ---

# 

# \## 🛡️ Suspicious Ports

# 

# Portr highlights connections to ports commonly used by malware, bots, or hacking tools (e.g., 6667, 31337, 12345, 23, etc.) in \*\*yellow\*\*. You can customize this list in the source code.

# 

# ---

# 

# \## 📝 Additional Features \& Ideas for future

# 

# \- \[ ] DNS resolution for remote IPs/domains

# \- \[ ] Export logs or snapshots to a file

# \- \[ ] Filtering/searching in the TUI

# \- \[ ] Remove entries from blacklist/whitelist from the TUI

# \- \[ ] Advanced TUI features (scrolling, sorting, column resizing)

# \- \[ ] Cross-platform support (Linux/macOS)

# \- \[ ] More security awareness (threat intelligence integration)

# 

# ---

# 

# \## 🤝 Contributing

# 

# Contributions, bug reports, and feature requests are welcome! Please open an issue or pull request.

# 

# ---

# 

# \## 📄 License

# 

# MIT License. See \[LICENSE](LICENSE) for details.

# 

# ---

# 

# \*\*Portr is a project by NeRDs (Network Risk Defenders).\*\*



