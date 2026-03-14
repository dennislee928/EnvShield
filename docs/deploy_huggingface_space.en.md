# Minimal Hugging Face Space Deployment Guide

This guide uses the repo root `Dockerfile` to deploy the EnvShield Go control plane and React console as a single Docker Space.

## Scope

- Goal: publish a shareable EnvShield demo
- Good for: product demos, UI previews, API smoke tests
- Not good for: a real production secrets control plane

Why:

- `services/control-plane` still uses an in-memory store
- Hugging Face Spaces can sleep when idle
- Local disk inside a Docker Space should not be treated as durable storage

## 1. Create a Docker Space

1. Create a new Space on Hugging Face.
2. Select `Docker` as the SDK.
3. Start with the free CPU hardware.
4. Do not overwrite the Space root `README.md` with this repo’s root `README.md`, because Docker Spaces need YAML frontmatter for configuration.

## 2. Use the Space README template

Copy [huggingface_space_readme.template.md](./huggingface_space_readme.template.md) into the Space repo root as `README.md`.

The two important settings are:

- `sdk: docker`
- `app_port: 8080`

That matches the current root `Dockerfile`, which serves on port `8080`.

## 3. Push the project to the Space repo

The simplest option is to treat the Hugging Face Space as another git remote:

```bash
git remote add hf-space https://huggingface.co/spaces/<hf-username>/<space-name>
git push hf-space main
```

If you do not want the main repo `README.md` to be replaced with the Space-specific one, use a temporary deployment branch:

```bash
git checkout -b deploy/hf-space
cp docs/huggingface_space_readme.template.md README.md
git add README.md
git commit -m "chore: prepare hugging face space readme"
git push hf-space deploy/hf-space:main
```

## 4. Recommended Space Variables

In Space Settings -> Variables, set at least:

```text
ENVSHIELD_PUBLIC_URL=https://<your-space-app-url>
```

This value should point to the actual public app URL of your Space.

Inference:
it is often something like `https://<owner>-<space>.hf.space`, but use the exact URL shown by Hugging Face.

This demo does not require extra secrets to boot today. If you later wire in real GitHub OAuth, add:

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`

## 5. What should be live after deploy

- `/`: EnvShield console
- `/healthz`: health endpoint
- `/v1/*`: control plane API

Because this is a single-container deployment, no separate frontend service is needed.

## 6. Demo-mode caveats

- The first load after sleep may be slow.
- The CLI browser-assisted login flow can be demonstrated, but this Space should not be treated as a real team environment.
- Data is currently stored in memory, so redeploys or sleep cycles can wipe it.

## 7. Official references

- Docker Spaces: https://huggingface.co/docs/hub/main/en/spaces-sdks-docker
- Spaces overview: https://huggingface.co/docs/hub/main/en/spaces-overview
- Spaces config reference: https://huggingface.co/docs/hub/spaces-config-reference
