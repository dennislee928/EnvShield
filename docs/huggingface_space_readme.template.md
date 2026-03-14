---
title: EnvShield
emoji: "🛡️"
colorFrom: teal
colorTo: gray
sdk: docker
app_port: 8080
short_description: Zero-knowledge developer secrets demo
---

# EnvShield

This Space runs the EnvShield demo using the repository root `Dockerfile`.

It serves:

- `/` for the console UI
- `/healthz` for health checks
- `/v1/*` for the control plane API

Set `ENVSHIELD_PUBLIC_URL` in Space Variables to this Space's public app URL.

