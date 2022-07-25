# apk-parser

an API for parsing apk files and extract manifest information from it

Article about this repository: [Writing an API with Rust to parse and extract info from Apk](https://benyaamin.com/post/writing-an-api-with-rust-to-parse-and-extract-info-from-apk/)

usage is simple :)

## 1. install rust on your machine

best document for install is own [Rust documention](https://www.rust-lang.org/tools/install)

for linux users download and run rust bash file

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

then just add this line to your shell. for bash:

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

## 2. Run

```bash
cargo run
```

## 3. Request

send a multipart request with a file and `apk` field name.
