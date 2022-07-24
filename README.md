# apk-parser
an API for parsing apk files and extract manifest information from it

usage is simple :)

## 1. install rust on your machine 
best document for install is own [Rust documention](https://www.rust-lang.org/tools/install)

for linux users download and run rust bash file
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then just add this line to your shell. for bash:
```
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

## 2. Run

```
cargo run
```

## 3. Request
send a multipart request with a file and `apk` field name.

