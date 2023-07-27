default:
  just --list

install:
  rustup override set nightly
  rustup target add wasm32-unknown-unknown
  rustup component add rustfmt
  which leptosfmt || cargo install --locked leptosfmt
  npm install
  @echo "DONE"

watch:
  trunk serve

build-release:
  trunk build

clean:
  cargo clean
  trunk clean
