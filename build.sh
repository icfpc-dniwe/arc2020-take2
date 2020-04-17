#!/bin/sh

set -e

mkdir -p dist

cargo build
cp target/debug/arc2020-take2 dist/runnable
if hash patchelf 2>/dev/null; then
  # Patch on NixOS
  patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 dist/runnable
fi
(
  echo 'OUTFILE = """'
  cat dist/runnable | gzip | base64
  echo '"""'
  cat runner.py
) > dist/arc2020_runner.py
xclip -i -selection clipboard dist/arc2020_runner.py
