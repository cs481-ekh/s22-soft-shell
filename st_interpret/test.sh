if ! [ -x "$(command -v cargo)" ]; then
  echo 'Warning: cargo is not installed.' >&2
  echo 'Installing...' >&2
  curl https://sh.rustup.rs -sSf | sh
fi

cargo install cargo-c || { echo "Cargo-c install failed" >&2; exit 1; }

cargo ctest || { echo "Testing Failed" >&2; exit 1; }