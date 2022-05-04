if ! [ -x "$(command -v cargo)" ]; then
  echo 'Warning: cargo is not installed.' >&2
  echo 'Installing...' >&2
  curl https://sh.rustup.rs -sSf | sh
fi

cargo clean || { echo "cargo clean failed" >&2; exit 1; }