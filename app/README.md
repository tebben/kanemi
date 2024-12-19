# Tauri

Testing creating an app using Tauri with Svelte and [skeleton.dev](https://www.skeleton.dev/)

## Pre-requisites

```sh
cargo install tauri-cli --version "^2.0.0" --locked
```

## Run dev

```sh
cargo tauri dev
```

## Android

Not tested yet, need to install android studio/SDK first. Something to try out for later.

### Build apk for testing

```sh
cargo tauri android init
cargo tauri android build --apk --target aarch64 --target armv7
```
