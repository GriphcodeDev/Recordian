# 🎙️ Recordian (Work in progress)

**Recordian** is an easy-to-use audio recording application, designed with simplicity and functionality in mind. Built with Rust and Slint, it provides a straightforward UI for recording, saving, and managing audio clips with a focus on speed and usability.

---

## 📜 Features

- **Record High-Quality Audio**: Capture audio clips with clear sound quality.
- **User-Friendly Interface**: Minimalistic design with easy navigation.
- **Cross-Platform Support**: Runs on Linux, macOS, and Windows.
- **Fast and Lightweight**: Developed in Rust for performance and efficiency.

---

## 📥 Installation

### Prerequisites

Make sure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Slint](https://slint.dev/) (for UI components)

### Steps

1. **Clone the Repository**
    ```bash
    git clone https://github.com/yourusername/recordian.git
    cd recordian
    ```

2. **Build the Project**
    ```bash
    cargo build --release
    ```

3. **Run the Application**
    ```bash
    cargo run --release
    ```

---

## 📂 Usage

Once you start Recordian, you can:
1. **Start Recording**: Click the "Record" button to begin.
2. **Stop Recording**: Click "Stop" to end and save the clip.
3. **Playback**: Manage and play back saved recordings.

---

## 💻 Development

Contributions are welcome! To get started with development:
1. Fork the repo and clone your fork locally.
2. Make sure you have `slint` installed for UI modifications.

To install Slint, add it as a dependency in `Cargo.toml`:
```toml
[dependencies]
slint = "1.8.0"
