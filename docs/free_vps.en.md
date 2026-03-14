# Free VPS and Hosting Platforms Guide

This document describes recommended free VPS and PaaS options for the EnvShield project for development, deployment, and demos.

---

## 1. Overview

| Item | Description |
|------|-------------|
| Document version | 1.1 |
| Audience | Developers, operators |
| Purpose | Backend deployment, compatibility testing, online demos |

---

## 2. Platform Comparison

| Platform | Type | RAM | Storage / Compute | Notes |
|----------|------|-----|-------------------|-------|
| Serv00 | FreeBSD Shell (VPS-like) | 512 MB | 3 GB | Good for SSH and shell testing, not a primary v1 deployment target |
| Koyeb | Container PaaS | 512 MB | 0.1 vCPU / 2 GB SSD / 1 free Web Service | Official FAQ currently mentions credit-card validation; free PostgreSQL is demo-grade |
| Hugging Face Spaces | Docker space | 16 GB | 2 vCPU / 50 GB non-persistent disk | Excellent for a single-container demo; idle Spaces sleep |
| Back4App Containers | CaaS (container) | 256 MB | 0.25 CPU / 100 GB transfer / 1 free container | Strong fit for a single-container preview; may pause when idle |
| Alwaysdata | Full-featured PaaS | 256 MB | 1 GB (free Cloud has extra restrictions) | Includes DB/SSH/Cron, but the free mode has meaningful constraints |
| Deta Space | Serverless / micro-cloud | Not specified | Platform-defined quota | Not a natural fit for the current long-running Go API architecture |

---

## 3. Platform Details

### 3.1 Serv00.com

**Summary**: Poland-based free hosting providing a FreeBSD shell account with VPS-like permissions and workflow; widely used in developer communities.

**Specifications**

- Memory: 512 MB RAM  
- Storage: 3 GB  

**Advantages**

- SSH access; you can open ports and run Node.js, Go, Python, or small databases.  
- Email-only registration; no credit card required.  

**Limitations**

- Best treated as an SSH and shell environment, not as a formally supported EnvShield v1 runtime.  
- EnvShield v1 officially targets macOS and Linux first; FreeBSD should be considered extra compatibility work.  

---

### 3.2 Koyeb

**Summary**: Container-based PaaS with GitHub-based auto-deploy; a practical alternative to Render.

**Specifications**

- Free tier: 1 Web Service  
- Memory: 512 MB RAM  
- CPU: 0.1 vCPU  

**Advantages**

- Deploy via Dockerfile or directly from a GitHub repo.  
- Regions include Frankfurt and Washington.  
- A good fit for the repo root Dockerfile that bundles the Go control plane and React console into one service.  

**Limitations**

- The official pricing FAQ currently says a credit card is required for validation.  
- The free PostgreSQL offer has active-time limits, so it is better for demos than for a durable secrets database.  

---

### 3.3 Hugging Face Spaces

**Summary**: Hugging Face Spaces can be used with a Docker runtime as a free backend or demo server.

**Specifications**

- Memory: up to 16 GB RAM (free tier)  
- CPU: 2 vCPU  

**Advantages**

- Choose the “Docker” environment and provide a Dockerfile to run the Go API and static console together.  
- No credit card required.  

**Limitations**

- Great for demos, but not for low-latency auth polling or durable state.  
- The disk is non-persistent, so local files should never be treated as durable storage.  

---

### 3.4 Back4App Containers

**Summary**: Back4App started as BaaS and later added container hosting (CaaS) with a no–credit-card free tier, suitable for Docker-based backends.

**Specifications**

- Free tier: 1 Docker container  
- Memory: 256 MB RAM  
- CPU: 0.25 CPU  

**Advantages**

- Full Docker support; backends can be packaged as Docker images (Go or Node.js) and deployed from GitHub with automatic build and deploy; clean, controllable environment.  
- No credit card required.  

**Limitations**

- If there is **no traffic or deploy activity for 30 days**, the container may be paused and require manual restart.  
- The memory budget is best spent on a single Go binary plus static assets, not Node SSR plus a database.  

---

### 3.5 Alwaysdata

**Summary**: Long-standing European PaaS with a free tier that is feature-rich but capacity-limited; suitable for lightweight MVPs.

**Specifications**

- Memory: 256 MB RAM  
- Storage: 1 GB (the free Cloud mode has extra restrictions)  

**Advantages**

- Free tier includes SSH, built-in PostgreSQL/MySQL, and Cron.  
- Supports Node.js, Python, PHP, Ruby, and others.  
- Once a Postgres-backed EnvShield store exists, it could become a compact all-in-one MVP host.  

**Limitations**

- The official free Cloud mode carries special restrictions around domains, remote DB connections, and certain usage patterns, so do not assume it behaves like a generic VPS.  

---

### 3.6 Deta Space

**Summary**: “Personal micro-cloud” platform where apps are published as Space Apps; free, no credit card, with strong data isolation.

**Specifications**

- Architecture: Serverless; no explicit RAM limit  
- Storage: built-in NoSQL (Deta Base) and file storage (Deta Drive)  

**Advantages**

- Fully free; no credit card.  
- Supports Node.js and Python.  
- Apps can be published to Space OS so other developers can one-click install into their own isolated space; aligns with end-to-end encryption and data isolation.  

**Limitations**

- Only Node.js and Python are natively supported.  
- The current EnvShield control plane is a long-running Go HTTP service, so Deta is not a first-choice v1 target.  

---

## 4. Recommended Use Cases

### 4.1 Core Backend Deployment (Koyeb)

**Use case**: Host the EnvShield API server (control plane) as a public service.

**Approach**

- Use the repo root `Dockerfile` so Koyeb can build one container with the Go control plane and React console together.  
- Set at least `ENVSHIELD_PUBLIC_URL` so CLI browser-assisted login points back to the correct host.  

**Benefits**

- Stable HTTPS endpoint for the CLI, SDK, or frontend to fetch variables.  

---

### 4.2 Compatibility and Harsh-Environment Testing (Serv00)

**Use case**: Validate CLI and injection behavior on a traditional, resource-limited host.

**Approach**

- Reframe Serv00 as an extra shell environment, not an officially supported deployment target.  
- Use it to validate small Go binaries, health checks, shell scripts, and future FreeBSD compatibility work instead of assuming v1 CLI parity today.  

**Benefits**

- Passing this environment demonstrates that the product works in demanding setups and can be used as a product differentiator.  

---

### 4.3 Zero-Install Online Demo (Hugging Face Spaces)

**Use case**: Offer an online demo that does not require installing the NPM package or CLI locally.

**Approach**

- Use Hugging Face Spaces’ Docker environment and deploy the repo root `Dockerfile` directly so the console and API run in one container.  
- Position it as a product demo, not as the real secrets control plane.  

**Benefits**

- Users can open the Space URL in a browser to see “environment variables securely injected and the app running” without any local setup.  

---

## 5. EnvShield Deployment Recommendations

### 5.1 Bottom line

With the repository in its current state, the most practical free deployment strategy is:

| Component | Recommended platform | Why |
|-----------|----------------------|-----|
| Single-container demo (`API + Console`) | Hugging Face Spaces / Back4App Containers | The repo already has a root `Dockerfile`, so you can deploy one container with minimal changes. |
| Staging control plane | Koyeb | Best Docker and GitHub ergonomics among the listed options and an easy path to a paid upgrade later. |
| CLI distribution | GitHub Releases + npm / Go wrapper | The CLI should be distributed, not “hosted” on a PaaS. |
| Shell and constrained-host validation | Serv00 | Useful for SSH and shell behavior checks, but not a primary v1 hosting target. |

### 5.2 Real constraints from the current codebase

- `services/control-plane` still uses an in-memory store today. On any free host, secrets metadata and snapshots can disappear on restart, redeploy, or sleep. Right now these platforms are best for demos, previews, and QA.
- The repo now has a root `Dockerfile` that packages the Go control plane and the React console into one runtime image. On Koyeb, Back4App, and Hugging Face, prefer that single-container topology over splitting the free tier into multiple services.
- `shield-cli` officially targets macOS and Linux in v1. Serv00 is FreeBSD, so it should be treated as extra compatibility work, not as a guaranteed runtime target.

### 5.3 Platform-by-platform advice for EnvShield

#### Koyeb

- Best fit for a staging control plane with the bundled console.
- Deploy from the repo root so the platform can build the root `Dockerfile`.
- Set at least `ENVSHIELD_PUBLIC_URL=https://<your-domain>`.
- Even after a Postgres adapter exists, treat the free Koyeb PostgreSQL option as demo-grade because of the active-time limit.

#### Hugging Face Spaces

- Best fit for a public demo, not the real backend.
- Use a Docker Space and set `app_port=8080`.
- Because the Space can sleep, the browser approval flow and CLI polling loop may feel sluggish. Position it as a product demo, not as a team workflow backend.

#### Back4App Containers

- Strong fit for a preview URL or lightweight demo.
- Keep the current topology: one Go binary serving static assets.
- Use `/healthz` as the health-check endpoint.

#### Alwaysdata

- Consider it only after the Postgres-backed store exists.
- Its value is co-locating app, DB, SSH, and Cron, but the free plan is more restrictive than a normal VPS.
- For the current codebase, it is not the first platform to reach for.

#### Serv00

- Use it for shell scripts, memory-constrained startup behavior, and future Linux/FreeBSD compatibility checks.
- Do not position it as the primary control-plane host, and do not imply FreeBSD CLI support until that target is actually tested and added to CI.

#### Deta Space

- Not recommended for v1.
- The mismatch is architectural: EnvShield currently centers on a long-running Go API, browser approval flow, and future snapshot polling.

### 5.4 Suggested next steps

1. Deploy the root `Dockerfile` to Hugging Face Spaces for the first public demo.
2. Deploy the same image to Koyeb for a staging control plane.
3. Make a Postgres-backed store the next engineering milestone before treating any free host as a real secrets control plane.

## 6. References

- [Serv00](https://serv00.com/)  
- [Koyeb](https://www.koyeb.com/)  
- [Hugging Face Spaces](https://huggingface.co/spaces)  
- [Back4App Containers](https://www.back4app.com/)  
- [Alwaysdata](https://www.alwaysdata.com/)  
- [Deta Space](https://deta.space/)  

---

## 7. Other Platforms by Use Case

The following platforms are grouped by scenario. Check each provider’s site for current specs and free-tier limits.

### 7.1 Microservices and Core API Hosting

For core business logic and APIs in Go, Python, or Node.js.

| Platform | Summary | Link |
|----------|---------|------|
| Adaptable.io | Full-stack containers; connect GitHub for auto runtime detection; free tier includes PostgreSQL or MongoDB, suitable for keys and config. | [Adaptable.io](https://adaptable.io/) |
| Choreo | WSO2 developer platform with generous free tier; Go / Python / Node.js; visual microservice topology. | [Choreo](https://wso2.com/choreo/) |
| Leapcell | Serverless hosting for Go / Python / Node.js; built-in distributed SQLite storage for lightweight state. | [Leapcell](https://leapcell.io/) |
| Genezio | Node.js / TypeScript; type-safe RPC so frontend or CLI can call backend like local functions. | [Genezio](https://genezio.com/) |

### 7.2 Edge and WebAssembly (Wasm)

For Rust-based E2E crypto or security modules compiled to Wasm.

| Platform | Summary | Link |
|----------|---------|------|
| Fermyon Cloud | Wasm-first Serverless; deploy Rust-compiled Wasm with low cold start and millisecond response. | [Fermyon Cloud](https://www.fermyon.com/fermyon-cloud) |
| Deno Deploy | Global edge; JS/TS and Wasm support; load Rust-compiled Wasm modules to reduce verification latency. | [Deno Deploy](https://deno.com/deploy) |

### 7.3 Background Jobs and Automation

For scheduled jobs, webhook-triggered flows, and variable-sync workers.

| Platform | Summary | Link |
|----------|---------|------|
| Windmill.dev | Open source; turn Python / Go / TypeScript scripts into APIs or scheduled jobs with UI; good for sync workers. | [Windmill](https://windmill.dev/) |
| Pipedream | Event-driven; webhooks trigger Node.js / Python / Go; e.g. listen to deploy events and trigger variable sync. | [Pipedream](https://pipedream.com/) |
| Val Town | Lightweight TypeScript; write functions in the browser and expose as APIs; good for transforms or simple webhooks. | [Val Town](https://www.val.town/) |

### 7.4 Self-Managed Infrastructure (Kubernetes)

For deployments that require full control over orchestration and YAML.

| Platform | Summary | Link |
|----------|---------|------|
| KubeSail | Free Kubernetes namespace (no card); run your own Docker and YAML for Go / Node.js microservices. | [KubeSail](https://kubesail.com/) |

---

*This document is technical guidance for the EnvShield project. Terms and pricing are determined by each provider’s official site.*
