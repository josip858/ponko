#!/usr/bin/env bash
# Fails if anything under DIR (validation bundles about to be published)
# names a place on the machine that made it: the home directory, the
# temporary directory, the run's scratch directories, or the checkout.
#
#   scripts/validation-bundle-guard.sh DIR
#
# Used by validation-bundles.sh and validation-bundle-native.sh.
set -euo pipefail

dir="${1:?usage: validation-bundle-guard.sh DIR}"

# Directories of this machine, where known. A trailing slash is dropped so
# "/tmp/x/" and "/tmp/x" are the same thing; "/" alone is never searched for.
places=()
add() {
  local place="${1%/}"
  if [ -n "$place" ] && [ "$place" != "/" ]; then
    places+=("$place")
  fi
}
add "${HOME:-}"
add "${TMPDIR:-}"
add "${RUNNER_TEMP:-}"
add "${GITHUB_WORKSPACE:-}"
add "$(pwd -P)"
# The temporary directory as the system resolves it (macOS: /private/var/...).
if [ -n "${TMPDIR:-}" ] && [ -d "$TMPDIR" ]; then
  add "$(cd "$TMPDIR" && pwd -P)"
fi
# Fixed names: the tool's scratch directories and the usual temporary roots.
places+=("ponko-validation-" "/tmp/" "/var/folders/" "/var/tmp/")

found=0
for place in "${places[@]}"; do
  if hits="$(grep -rIlF -- "$place" "$dir")"; then
    echo "validation-bundle-guard: a published file names '$place':" >&2
    echo "$hits" >&2
    found=1
  fi
done
if [ "$found" != 0 ]; then
  exit 1
fi
