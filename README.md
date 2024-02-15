# 🏴‍☠️ Scurvy
Literally the laziest extention for your terminal.
Instead of having to 5 seconds of work when you do
'ifconfig' or 'hostname -I' you can just do 'ip' or
'mac' and get the same result. This is not limited in
scope to just ip & mac, however.

Altneratively, you can qualify the cli with the 'scurvy' name to use this as a traditional cli tool (i.e. `scurvy ip` or `scurvy mac`).

## 🛠 Development

Get started with `scurvy` by running it locally:

```bash
# To run scurvy with specific tools (ip, mac)
cargo run --bin scurvy {ip, mac}

# To run individual tools
cargo run --bin ip
cargo run --bin mac
```

## 🏗 Build

Compile `scurvy` for deployment:

```bash
cargo build --release
```

## 📦 Installation
TODO

## 🔨 Usage
TODO

## 🤝 Contributing
Contributions are welcome! If you have ideas for new features, improvements, or find a bug, please open an issue or submit a pull request.
