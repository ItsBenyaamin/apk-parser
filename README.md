# APK Parser

It's an API for parsing apk files and extract manifest information from it

Article about this project: [Writing an API with Rust to parse and extract info from Apk](https://benyaamin.com/post/writing-an-api-with-rust-to-parse-and-extract-info-from-apk/)

Usage is simple :)

## 1. install Rust

First, you need to install Rust. For linux users download and run rust bash file

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

then just add this line to your shell. for bash:

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

> Note: Check [Rust documentation](https://www.rust-lang.org/tools/install) for more information

## 2. Run

```bash
git clone https://github.com/graymind75/apk-parser.git && cd apk-parser
cargo run
```

## 3. Request

Send a multipart request with a file and `apk` field name.
