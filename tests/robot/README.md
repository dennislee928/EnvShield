# Robot Framework API Smoke Tests

This folder contains Robot Framework suites that validate the deployed EnvShield control-plane API as a black-box service.

## What this covers

- `GET /healthz`
- all current `/v1/*` control-plane endpoints
- happy-path API flows for auth, devices, workspaces, snapshots, and environment status
- stable negative cases:
  - malformed JSON returns `400`
  - unknown resources return `404`
  - pending device approval returns `409`

## What this does not cover

- console UI rendering
- Rust CLI behavior
- Koyeb deployment behavior
- schema-level `400` validation for missing fields or empty strings

The current server does not implement full request-schema validation, so this suite only treats malformed JSON as a stable `400` contract.

## Local usage

1. Create a virtual environment and install dependencies:

```bash
python3 -m venv .venv-robot
source .venv-robot/bin/activate
python3 -m pip install -r tests/robot/requirements.txt
```

2. Run the full suite against the deployed API:

```bash
python3 -m robot \
  --outputdir robot-results \
  --variable BASE_URL:https://<your-koyeb-domain> \
  tests/robot/suites
```

For this repository, the currently deployed Koyeb URL is also recorded in `koyeb.yaml`.

3. Open the generated reports:

- `robot-results/log.html`
- `robot-results/report.html`
- `robot-results/output.xml`

## GitHub Actions usage

The repo includes a manual workflow:

- `.github/workflows/robot-api-smoke.yml`

It uses the repository variable `KOYEB_PUBLIC_URL` by default and also supports a manual `base_url` override from the workflow dispatch form.

This workflow is intentionally not part of the required `CI` checks. It is a remote smoke suite and depends on the current state of the deployed Koyeb service.

## Suite layout

- `suites/00_health.robot`
- `suites/10_auth.robot`
- `suites/20_devices.robot`
- `suites/30_workspaces.robot`
- `suites/40_snapshots.robot`
- `suites/50_status.robot`
- `suites/60_negative.robot`

## Data isolation

Each test run generates unique values for:

- `deviceName`
- `actorEmail`
- `workspaceName`
- `environment`

That keeps repeated runs from colliding on the shared remote Koyeb environment.
