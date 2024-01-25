<div align="center"><img src="https://i.imgur.com/dH14aVV.png" alt="neopeek-preview" border="0" width = 600></div>
<h1 align="center">🚀 neopeek &nbsp<a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></img></a></h1>
<p align="center"><strong>CLI tool for quickly obtaining system information in a fancy way</strong></p>

> [!NOTE]
> This is a fan project made in Rust, created solely for learning purposes.
>
> Currently only Windows is supported as the program relies on `wmic`.

## 📦 Installing

### 🔌 Requirements
- **Rust**: Get it from the [official website](https://www.rust-lang.org/tools/install)

<br>   

Clone the repository:
```sh
git clone https://github.com/ReBlast/neopeek.git
```

Build from project directory:
```sh
cargo build --release
```

Add the executable to your PATH:
```sh
setx /M path "%PATH%; (PATH TO .EXE FOLDER)"
```