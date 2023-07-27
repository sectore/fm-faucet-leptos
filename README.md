_!!! still WIP !!!_

# FM Faucet Leptos

## Port of [faucet-rs](https://github.com/justinmoon/faucet-rs/tree/master)


## Install

```bash
just install
```

Alternatively, if the `nix` package manager is installed, run `nix develop` to enter a developer environment containing all the necessary dependencies.

## Run

```bash
just watch
```

## (from leptos-start): Executing a Server on a Remote Machine Without the Toolchain 
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
fm_faucet
site/
```
Set the following enviornment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="fm_faucet"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.
