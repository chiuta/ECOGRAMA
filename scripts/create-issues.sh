#!/usr/bin/env bash
# Creates the labels and the starter issues from docs/issues/*.md on GitHub.
# Usage: scripts/create-issues.sh OWNER/REPO   (needs `gh auth login`; run once, after the first push)
set -euo pipefail
repo="${1:?usage: scripts/create-issues.sh OWNER/REPO}"
cd "$(dirname "$0")/.."
for f in docs/issues/*.md; do
  title="$(sed -n 1p "$f")"
  labels="$(sed -n 2p "$f")"
  IFS=',' read -ra ls <<< "$labels"
  for l in "${ls[@]}"; do gh label create "$l" --repo "$repo" --force >/dev/null; done
  gh issue create --repo "$repo" --title "$title" --label "$labels" --body "$(tail -n +4 "$f")"
done
