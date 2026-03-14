# Koyeb Deployment Runbook

This runbook deploys EnvShield to Koyeb using the repo root `Dockerfile`, packaging the Go control plane and React console as a single web service.

## Scope

- Goal: create a reachable staging control plane
- Good for: CLI integration testing, console previews, demo environments
- Not good for: a real production secrets control plane

Why:

- `services/control-plane` still uses an in-memory store
- Any free-host restart, redeploy, or idle policy can wipe current data

## Prerequisites

1. This repository is already pushed to GitHub.
2. You have a Koyeb account and have completed any required validation.
3. You have installed the Koyeb CLI.
4. You have logged in:

```bash
koyeb login
```

Official docs:

- CLI installation: https://www.koyeb.com/docs/build-and-deploy/cli/installation
- Build from Git: https://www.koyeb.com/docs/build-and-deploy/build-from-git

Additional note:
the official Koyeb docs currently focus on dashboard flows, CLI flags, and other IaC integrations. The repo root [koyeb.yaml](../koyeb.yaml) is EnvShield's own declarative deployment config, consumed by [deploy_koyeb.sh](../scripts/deploy_koyeb.sh) and translated into Koyeb CLI arguments.

## 1. Minimal manual deployment flow

If you want to validate the setup once in the dashboard first, use this sequence:

1. Create a new Web Service in Koyeb.
2. Select your GitHub repo as the source.
3. Choose `Docker` as the builder.
4. Set the Dockerfile path to `Dockerfile`.
5. Expose port `8080:http`.
6. Add a route from `/` to port `8080`.
7. Add a health check `8080:http:/healthz`.
8. Add at least this environment variable:

```text
ENVSHIELD_PUBLIC_URL=https://<your-koyeb-domain>
```

9. After deployment, confirm:

- `/` serves the console
- `/healthz` responds
- `/v1/*` serves the control plane API

## 2. Config-based deploy

The recommended path is to edit the repo root [koyeb.yaml](../koyeb.yaml) first:

```yaml
app: envshield
service: envshield
repository: github.com/<owner>/<repo>
branch: main
public_url: https://<your-koyeb-domain>
regions: fra
```

Then run:

```bash
./scripts/deploy_koyeb.sh --config koyeb.yaml
```

After that, future redeploys are just edits to `koyeb.yaml` plus rerunning the same command.

If you want automatic deployment from GitHub Actions, the repo now also includes:

- [ci.yml](../.github/workflows/ci.yml)
- [deploy-koyeb.yml](../.github/workflows/deploy-koyeb.yml)

Recommended repository settings:

- GitHub Secret: `KOYEB_TOKEN`
- GitHub Variable: `KOYEB_PUBLIC_URL`
- GitHub Variable: `KOYEB_ORGANIZATION` if you deploy into an organization

The flow is now:

1. `ci.yml` handles pure test and build validation on pushes and pull requests.
2. `deploy-koyeb.yml` only deploys after `CI` succeeds on `main`.
3. `deploy-koyeb.yml` still supports manual `workflow_dispatch`.

## 3. Create or update the service with the CLI

The current Koyeb CLI reference supports:

- `--git-builder docker`
- `--git-docker-dockerfile Dockerfile`
- `--ports 8080:http`
- `--routes /:8080`
- `--checks 8080:http:/healthz`

Minimal example:

```bash
koyeb apps create envshield

koyeb services create envshield \
  --app envshield \
  --git github.com/<owner>/<repo> \
  --git-branch main \
  --git-builder docker \
  --git-docker-dockerfile Dockerfile \
  --type web \
  --instance-type nano \
  --ports 8080:http \
  --routes /:8080 \
  --checks 8080:http:/healthz \
  --env ENVSHIELD_PUBLIC_URL=https://<your-koyeb-domain> \
  --wait
```

To update an existing service:

```bash
koyeb services update envshield/envshield \
  --git github.com/<owner>/<repo> \
  --git-branch main \
  --git-builder docker \
  --git-docker-dockerfile Dockerfile \
  --type web \
  --instance-type nano \
  --ports 8080:http \
  --routes /:8080 \
  --checks 8080:http:/healthz \
  --env ENVSHIELD_PUBLIC_URL=https://<your-koyeb-domain> \
  --wait
```

## 4. Use the deploy script directly

The repo now includes a reusable deployment script: [deploy_koyeb.sh](../scripts/deploy_koyeb.sh)

If you do not want to run interactive `koyeb login` first, you can provide a token directly:

```bash
KOYEB_TOKEN=<your-koyeb-pat> ./scripts/deploy_koyeb.sh --config koyeb.yaml
```

Minimal usage:

```bash
./scripts/deploy_koyeb.sh \
  --config koyeb.yaml
```

If you want to override a config value temporarily, pass flags on top:

```bash
./scripts/deploy_koyeb.sh \
  --config koyeb.yaml \
  --public-url https://<your-koyeb-domain>
```

If you do not want to use the config file at all, you can still deploy entirely with flags:

```bash
./scripts/deploy_koyeb.sh \
  --app envshield \
  --git github.com/<owner>/<repo> \
  --public-url https://<your-koyeb-domain>
```

Useful optional flags:

- `--service <name>`: custom service name, defaults to the app name
- `--config <path>`: defaults to the repo root `koyeb.yaml` when present
- `--git-branch <branch>`: defaults to `main`
- `--region <code>`: repeatable, for example `--region fra --region was`
- `--instance-type <type>`: defaults to `nano`
- `--organization <id>`: target a specific Koyeb organization
- `--wait-timeout <duration>`: defaults to `10m`

## 5. Post-deploy checks

After deployment, inspect Koyeb status:

```bash
koyeb apps get envshield
koyeb services get envshield --app envshield
```

Then run a minimal smoke test:

```bash
curl -i https://<your-koyeb-domain>/healthz
```

Then verify the homepage and API:

```bash
curl -i https://<your-koyeb-domain>/
curl -i https://<your-koyeb-domain>/v1/devices
```

## 6. Recommended settings

- Keep a single service for now; do not split frontend and backend
- Start with `instance-type nano`
- Use `/healthz` for health checks
- Use a single region first if this is only staging

## 7. Known limitations

- EnvShield does not yet have a real Postgres-backed store, so this is not suitable for important data
- If `ENVSHIELD_PUBLIC_URL` is not set to the actual public URL, the CLI browser-assisted login flow will point to the wrong location
- Koyeb free-tier policies and validation rules can change; defer to the dashboard and official docs

## 8. Official references

- Koyeb CLI installation: https://www.koyeb.com/docs/build-and-deploy/cli/installation
- Koyeb CLI reference: https://www.koyeb.com/docs/build-and-deploy/cli/reference
- Build from Git: https://www.koyeb.com/docs/build-and-deploy/build-from-git
- Health checks: https://www.koyeb.com/docs/run-and-scale/health-checks
