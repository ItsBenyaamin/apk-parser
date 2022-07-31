# APK Parser

It's an API for parsing apk files and extract manifest information & icon from it. You can read more about this project at this article: [Writing an API with Rust to parse and extract info from Apk](https://benyaamin.com/post/writing-an-api-with-rust-to-parse-and-extract-info-from-apk/)

## 1. install Rust

First, you need to install Rust. For linux users download and run rust bash file

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then just add this line to your shell. For bash:

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

> **Note**: Check [Rust documentation](https://www.rust-lang.org/tools/install) for more information

## 2. Run

```bash
git clone https://github.com/graymind75/apk-parser.git && cd apk-parser
cargo run
```

## 3. Request

Send a multipart request with a file and `apk` field name.

## 4. Result

You can see an example for [F-Droid](https://f-droid.org/):

```json
{
    "package_name": "org.fdroid.fdroid",
    "version_code": "1014050",
    "version_name": "1.14",
    "min_sdk_version": "22",
    "target_sdk_version": "25",
    "compile_sdk_version": "30",
    "compile_sdk_version_code_name": "11",
    "permissions": [
        "android.permission.INTERNET",
        "android.permission.ACCESS_NETWORK_STATE",
        "android.permission.ACCESS_WIFI_STATE",
        "android.permission.CHANGE_WIFI_MULTICAST_STATE",
        "android.permission.CHANGE_NETWORK_STATE",
        "android.permission.CHANGE_WIFI_STATE",
        "android.permission.BLUETOOTH",
        "android.permission.BLUETOOTH_ADMIN",
        "android.permission.RECEIVE_BOOT_COMPLETED",
        "android.permission.READ_EXTERNAL_STORAGE",
        "android.permission.WRITE_EXTERNAL_STORAGE",
        "android.permission.WRITE_SETTINGS",
        "android.permission.NFC",
        "android.permission.USB_PERMISSION",
        "android.permission.WAKE_LOCK",
        "android.permission.FOREGROUND_SERVICE"
    ],
    "icon": "iVBORw0KGgoAAAANSUhEUgAAAMAAAADACAYAAABS3GwHAAAqMUlEQVR4AeyaBVgbWRuF6 ..."
}
```

> **Note**: The icon is base64 encoded.
