default:
  just --list

install:
  rustup override set nightly
  rustup target add wasm32-unknown-unknown
  rustup component add rustfmt
  which cargo-leptos || cargo install --locked cargo-leptos
  which leptosfmt || cargo install --locked leptosfmt
  npm install
  @echo "DONE"

watch:
  cargo leptos watch -v --hot-reload

build-release:
  npx tailwindcss \
    -c ./tailwind.config.js \
    -i ./style/index.css \
    -o ./target/style/output.css \
    --minify
  cargo leptos build --release
