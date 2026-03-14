#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  ./scripts/deploy_koyeb.sh [--config koyeb.yaml] [options]

Options:
  --config <path>              Deployment config file. Defaults to ./koyeb.yaml when present.
  --app <name>                 Koyeb app name. Required unless provided by config.
  --service <name>             Koyeb service name. Defaults to the app name.
  --git <repo>                 Git repo in Koyeb CLI format, e.g. github.com/org/repo. Required unless provided by config.
  --token <token>              Koyeb personal access token. Defaults to $KOYEB_TOKEN if set.
  --git-branch <branch>        Git branch to deploy. Default: main.
  --public-url <url>           Value for ENVSHIELD_PUBLIC_URL. Default: https://envshield.invalid
  --region <code>              Deployment region. Repeatable.
  --instance-type <type>       Instance type. Default: nano.
  --wait-timeout <duration>    CLI wait timeout. Default: 10m.
  --organization <id>          Optional Koyeb organization ID.
  --help                       Show this help text.
EOF
}

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    exit 1
  fi
}

trim() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "$value"
}

strip_quotes() {
  local value="$1"
  if [[ "$value" == \"*\" && "$value" == *\" ]]; then
    value="${value:1:${#value}-2}"
  elif [[ "$value" == \'*\' && "$value" == *\' ]]; then
    value="${value:1:${#value}-2}"
  fi
  printf '%s' "$value"
}

CONFIG_PATH=""
APP_NAME=""
SERVICE_NAME=""
GIT_REPO=""
TOKEN="${KOYEB_TOKEN:-}"
GIT_BRANCH="main"
PUBLIC_URL="https://envshield.invalid"
INSTANCE_TYPE="nano"
WAIT_TIMEOUT="10m"
ORGANIZATION=""
REGIONS=()
BUILDER="docker"
DOCKERFILE="Dockerfile"
PORTS="8080:http"
ROUTES="/:8080"
CHECKS="8080:http:/healthz"
MIN_SCALE="1"
MAX_SCALE="1"

load_config() {
  local path="$1"
  local raw_line=""
  local line=""
  local key=""
  local value=""

  while IFS= read -r raw_line || [[ -n "$raw_line" ]]; do
    line="$(trim "${raw_line%$'\r'}")"
    if [[ -z "$line" || "$line" == \#* ]]; then
      continue
    fi

    if [[ "$line" =~ ^([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*:[[:space:]]*(.*)$ ]]; then
      key="${BASH_REMATCH[1]}"
      value="$(strip_quotes "$(trim "${BASH_REMATCH[2]}")")"

      case "$key" in
        app)
          if [[ -z "$APP_NAME" ]]; then
            APP_NAME="$value"
          fi
          ;;
        service)
          if [[ -z "$SERVICE_NAME" ]]; then
            SERVICE_NAME="$value"
          fi
          ;;
        repository|git)
          if [[ -z "$GIT_REPO" ]]; then
            GIT_REPO="$value"
          fi
          ;;
        branch|git_branch)
          if [[ "$GIT_BRANCH" == "main" ]]; then
            GIT_BRANCH="$value"
          fi
          ;;
        public_url)
          if [[ "$PUBLIC_URL" == "https://envshield.invalid" ]]; then
            PUBLIC_URL="$value"
          fi
          ;;
        instance_type)
          if [[ "$INSTANCE_TYPE" == "nano" ]]; then
            INSTANCE_TYPE="$value"
          fi
          ;;
        wait_timeout)
          if [[ "$WAIT_TIMEOUT" == "10m" ]]; then
            WAIT_TIMEOUT="$value"
          fi
          ;;
        organization)
          if [[ -z "$ORGANIZATION" ]]; then
            ORGANIZATION="$value"
          fi
          ;;
        regions)
          if [[ ${#REGIONS[@]} -eq 0 && -n "$value" ]]; then
            IFS=',' read -r -a parsed_regions <<< "$value"
            for region in "${parsed_regions[@]}"; do
              region="$(trim "$region")"
              if [[ -n "$region" ]]; then
                REGIONS+=("$region")
              fi
            done
          fi
          ;;
        builder)
          if [[ "$BUILDER" == "docker" ]]; then
            BUILDER="$value"
          fi
          ;;
        dockerfile)
          if [[ "$DOCKERFILE" == "Dockerfile" ]]; then
            DOCKERFILE="$value"
          fi
          ;;
        ports)
          if [[ "$PORTS" == "8080:http" ]]; then
            PORTS="$value"
          fi
          ;;
        routes)
          if [[ "$ROUTES" == "/:8080" ]]; then
            ROUTES="$value"
          fi
          ;;
        checks)
          if [[ "$CHECKS" == "8080:http:/healthz" ]]; then
            CHECKS="$value"
          fi
          ;;
        min_scale)
          if [[ "$MIN_SCALE" == "1" ]]; then
            MIN_SCALE="$value"
          fi
          ;;
        max_scale)
          if [[ "$MAX_SCALE" == "1" ]]; then
            MAX_SCALE="$value"
          fi
          ;;
      esac
    fi
  done < "$path"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --config)
      CONFIG_PATH="${2:-}"
      shift 2
      ;;
    --app)
      APP_NAME="${2:-}"
      shift 2
      ;;
    --service)
      SERVICE_NAME="${2:-}"
      shift 2
      ;;
    --git)
      GIT_REPO="${2:-}"
      shift 2
      ;;
    --token)
      TOKEN="${2:-}"
      shift 2
      ;;
    --git-branch)
      GIT_BRANCH="${2:-}"
      shift 2
      ;;
    --public-url)
      PUBLIC_URL="${2:-}"
      shift 2
      ;;
    --region)
      REGIONS+=("${2:-}")
      shift 2
      ;;
    --instance-type)
      INSTANCE_TYPE="${2:-}"
      shift 2
      ;;
    --wait-timeout)
      WAIT_TIMEOUT="${2:-}"
      shift 2
      ;;
    --organization)
      ORGANIZATION="${2:-}"
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

if [[ -z "$CONFIG_PATH" && -f "koyeb.yaml" ]]; then
  CONFIG_PATH="koyeb.yaml"
fi

if [[ -n "$CONFIG_PATH" ]]; then
  if [[ ! -f "$CONFIG_PATH" ]]; then
    echo "error: config file not found: $CONFIG_PATH" >&2
    exit 1
  fi
  load_config "$CONFIG_PATH"
fi

if [[ -z "$APP_NAME" ]]; then
  echo "error: --app is required" >&2
  usage >&2
  exit 1
fi

if [[ -z "$GIT_REPO" ]]; then
  echo "error: --git is required" >&2
  usage >&2
  exit 1
fi

if [[ -z "$SERVICE_NAME" ]]; then
  SERVICE_NAME="$APP_NAME"
fi

require_command koyeb

KOYEB_GLOBAL_ARGS=()
if [[ -n "$TOKEN" ]]; then
  KOYEB_GLOBAL_ARGS+=(--token "$TOKEN")
fi
if [[ -n "$ORGANIZATION" ]]; then
  KOYEB_GLOBAL_ARGS+=(--organization "$ORGANIZATION")
fi

koyeb_cmd() {
  koyeb "${KOYEB_GLOBAL_ARGS[@]}" "$@"
}

COMMON_ARGS=(
  --git "$GIT_REPO"
  --git-branch "$GIT_BRANCH"
  --git-builder "$BUILDER"
  --git-docker-dockerfile "$DOCKERFILE"
  --type web
  --instance-type "$INSTANCE_TYPE"
  --ports "$PORTS"
  --routes "$ROUTES"
  --checks "$CHECKS"
  --env "ENVSHIELD_PUBLIC_URL=$PUBLIC_URL"
  --min-scale "$MIN_SCALE"
  --max-scale "$MAX_SCALE"
  --wait
  --wait-timeout "$WAIT_TIMEOUT"
)

for region in "${REGIONS[@]}"; do
  COMMON_ARGS+=(--regions "$region")
done

app_exists=false
service_exists=false

if koyeb_cmd apps get "$APP_NAME" >/dev/null 2>&1; then
  app_exists=true
fi

if ! $app_exists; then
  echo "Creating app '$APP_NAME'..."
  koyeb_cmd apps create "$APP_NAME"
fi

if koyeb_cmd services get "$SERVICE_NAME" --app "$APP_NAME" >/dev/null 2>&1; then
  service_exists=true
fi

if ! $service_exists; then
  echo "Creating service '$SERVICE_NAME' in app '$APP_NAME'..."
  koyeb_cmd services create "$SERVICE_NAME" \
    --app "$APP_NAME" \
    "${COMMON_ARGS[@]}"
else
  echo "Updating service '$SERVICE_NAME' in app '$APP_NAME'..."
  koyeb_cmd services update "$APP_NAME/$SERVICE_NAME" \
    "${COMMON_ARGS[@]}"
fi

echo
echo "Deployment request completed."
echo "App:      $APP_NAME"
echo "Service:  $SERVICE_NAME"
echo "Git repo: $GIT_REPO"
echo "Branch:   $GIT_BRANCH"
echo "Public:   $PUBLIC_URL"
if [[ -n "$CONFIG_PATH" ]]; then
  echo "Config:   $CONFIG_PATH"
fi
if [[ -n "$TOKEN" ]]; then
  echo "Token:    provided"
fi

if [[ "$PUBLIC_URL" == "https://envshield.invalid" ]]; then
  echo
  echo "warning: ENVSHIELD_PUBLIC_URL is still using the placeholder value."
  echo "warning: rerun this script with --public-url https://<your-koyeb-domain> once the real domain is known."
fi
