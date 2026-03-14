#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  ./scripts/deploy_huggingface_space.sh --space <owner/space> [options]

Options:
  --space <owner/space>        Hugging Face Space path. Required unless --remote-url is set.
  --remote-url <url>           Explicit git remote URL for the Space.
  --username <name>            Hugging Face username, used with token-auth URLs.
  --token <token>              Hugging Face token for non-interactive HTTPS pushes.
  --target-branch <branch>     Space branch to overwrite. Default: main.
  --template <path>            README template path. Default: docs/huggingface_space_readme.template.md.
  --help                       Show this help text.
EOF
}

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    exit 1
  fi
}

SPACE_PATH=""
REMOTE_URL=""
HF_USERNAME=""
HF_TOKEN=""
TARGET_BRANCH="main"
TEMPLATE_PATH="docs/huggingface_space_readme.template.md"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --space)
      SPACE_PATH="${2:-}"
      shift 2
      ;;
    --remote-url)
      REMOTE_URL="${2:-}"
      shift 2
      ;;
    --username)
      HF_USERNAME="${2:-}"
      shift 2
      ;;
    --token)
      HF_TOKEN="${2:-}"
      shift 2
      ;;
    --target-branch)
      TARGET_BRANCH="${2:-}"
      shift 2
      ;;
    --template)
      TEMPLATE_PATH="${2:-}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

require_command git

if [[ -z "$REMOTE_URL" && -z "$SPACE_PATH" ]]; then
  echo "error: --space or --remote-url is required" >&2
  usage >&2
  exit 1
fi

if [[ ! -f "$TEMPLATE_PATH" ]]; then
  echo "error: template file not found: $TEMPLATE_PATH" >&2
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "error: working tree is not clean; commit or stash changes before deploying" >&2
  exit 1
fi

if [[ -z "$REMOTE_URL" ]]; then
  if [[ -n "$HF_TOKEN" ]]; then
    if [[ -z "$HF_USERNAME" ]]; then
      HF_USERNAME="${SPACE_PATH%%/*}"
    fi
    REMOTE_URL="https://${HF_USERNAME}:${HF_TOKEN}@huggingface.co/spaces/${SPACE_PATH}"
  else
    REMOTE_URL="https://huggingface.co/spaces/${SPACE_PATH}"
  fi
fi

TEMP_BRANCH="deploy/hf-space-$(date +%Y%m%d%H%M%S)"
TEMP_DIR="$(mktemp -d)"
REMOTE_NAME="hf-space-deploy"

cleanup() {
  set +e
  git worktree remove --force "$TEMP_DIR" >/dev/null 2>&1
  git branch -D "$TEMP_BRANCH" >/dev/null 2>&1
}

trap cleanup EXIT

git worktree add -b "$TEMP_BRANCH" "$TEMP_DIR" HEAD >/dev/null

cp "$TEMPLATE_PATH" "$TEMP_DIR/README.md"

(
  cd "$TEMP_DIR"
  git add README.md
  git commit -m "chore: prepare hugging face space deployment readme" >/dev/null
  git remote add "$REMOTE_NAME" "$REMOTE_URL"
  git push "$REMOTE_NAME" "$TEMP_BRANCH:$TARGET_BRANCH" --force
)

echo
echo "Hugging Face Space deployment push completed."
if [[ -n "$SPACE_PATH" ]]; then
  echo "Space:  $SPACE_PATH"
fi
echo "Branch: $TARGET_BRANCH"
echo
echo "Next steps:"
echo "1. Open the Space settings."
echo "2. Set ENVSHIELD_PUBLIC_URL to the Space app URL."
echo "3. Wait for the Docker build to finish."
