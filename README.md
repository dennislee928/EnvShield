# EnvShield

EnvShield is a zero-knowledge developer secrets platform. Phase 1 ships a Rust CLI that encrypts secrets locally and injects them into child processes without creating `.env` files, backed by a Go control plane and a TypeScript web console.

## Monorepo layout

- `crates/shield-cli`: Rust CLI, local state, crypto, command execution.
- `services/control-plane`: Go HTTP API for auth, workspaces, devices, and snapshots.
- `apps/console`: TypeScript React console for onboarding and snapshot visibility.
- `packages/api-client`: Generated TypeScript types and fetch client from the OpenAPI contract.
- `packages/npm-wrapper`: Thin npm wrapper that forwards to the Rust CLI binary.
- `wrappers/go`: Thin Go wrapper that forwards to the Rust CLI binary.
- `contracts/openapi.yaml`: Shared API contract used by the control plane and TypeScript packages.

## Quick start

1. Install Node 20+, Rust stable, and Go 1.22+.
2. Run `npm install`.
3. Run `npm run generate:types`.
4. Run `cargo test`.
5. Run `go test ./...` from `services/control-plane`.
6. Run `npm run test --workspaces`.

## Security model

- Secret values are encrypted on the client with XChaCha20-Poly1305.
- Each snapshot uses a random data key that is wrapped to each authorized device with age/X25519 recipients.
- Snapshot manifests are signed with an Ed25519 device key.
- The control plane stores ciphertext and readable metadata only.

## Current implementation notes

- GitHub OAuth is scaffolded with a development fallback that can auto-approve device flows when GitHub client credentials are not configured.
- The control plane defaults to an in-memory store for local development and tests, while keeping a store abstraction ready for a Postgres-backed implementation.
- macOS and Linux are the initial supported platforms for the Rust CLI wrappers.

## Container deployment

The repository includes a root `Dockerfile` that builds the React console and the Go control plane into a single runtime image.

```bash
docker build -t envshield .
docker run --rm -p 8080:8080 \
  -e ENVSHIELD_PUBLIC_URL=http://localhost:8080 \
  envshield
```

That container serves:

- `/v1/*` for the API
- `/healthz` for health checks
- `/` for the console UI

Deployment guides:

- [Koyeb guide](./docs/deploy_koyeb.md)
- [Koyeb guide (EN)](./docs/deploy_koyeb.en.md)
- [Hugging Face Space guide](./docs/deploy_huggingface_space.md)
- [Hugging Face Space guide (EN)](./docs/deploy_huggingface_space.en.md)
