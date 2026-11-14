#!/usr/bin/env bash
# Builds and signs the k-mood Health sync shortcut onto the Desktop. Run it
# yourself, so the Keychain token is only ever read by this machine's processes.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
python3 "$here/kmood_health.py" "$work/unsigned.shortcut"
out="$HOME/Desktop/k-mood Health sync.shortcut"
shortcuts sign --mode anyone --input "$work/unsigned.shortcut" --output "$out"
echo "signed: $out"
