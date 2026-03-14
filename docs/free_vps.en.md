# Free VPS and Hosting Platforms Guide

This document describes recommended free VPS and PaaS options for the EnvShield project for development, deployment, and demos.

---

## 1. Overview

| Item | Description |
|------|-------------|
| Document version | 1.0 |
| Audience | Developers, operators |
| Purpose | Backend deployment, compatibility testing, online demos |

---

## 2. Platform Comparison

| Platform | Type | RAM | Storage / Compute | Notes |
|----------|------|-----|-------------------|-------|
| Serv00 | FreeBSD Shell (VPS-like) | 512 MB | 3 GB | Must log in every 3 months to keep account active |
| Koyeb | Container PaaS | 512 MB | 0.1 vCPU / 1 free Web Service | No credit card; use an active GitHub account |
| Hugging Face Spaces | Docker space | 16 GB | 2 vCPU | Sleeps after 48 h without traffic; ~1 min wake-up |
| Back4App Containers | Container hosting (CaaS) | 250 MB | 0.1 vCPU / 1 free container | May pause after 30 days without traffic/deploys; restart manually |
| Alwaysdata | All-in-one PaaS | 512 MB | 100 MB total space | Built-in DB/Cron/SSH; small space, rotate logs |
| Deta Space | Personal micro-cloud / Serverless | per plan | Deta Base + Deta Drive | Node.js/Python only; no long-running background processes |

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

- To prevent idle resource use, you must log in at least once every **3 months** (web console or SSH) to keep the account active.  

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
- GitHub OAuth sign-in without a credit card (an account with some history is recommended).  

**Limitations**

- Stricter anti-abuse measures; use an active GitHub account to register.  

---

### 3.3 Hugging Face Spaces

**Summary**: Hugging Face Spaces can be used with a Docker runtime as a free backend or demo server.

**Specifications**

- Memory: up to 16 GB RAM (free tier)  
- CPU: 2 vCPU  

**Advantages**

- Choose the “Docker” environment and provide a Dockerfile to run Node.js, Go, or other backends and expose an API.  
- No credit card required.  

**Limitations**

- If there is **no traffic for 48 hours**, the Space goes to sleep; the next request may take about **1 minute** to wake it.  

---

### 3.4 Back4App Containers

**Summary**: Back4App started as BaaS and later added container hosting (CaaS); free tier requires no card and suits Docker-based backends.

**Specifications**

- Free tier: 1 Docker container  
- Memory: 250 MB RAM  
- CPU: 0.1 vCPU  

**Advantages**

- Full Docker support; backend can be packaged as a Docker image (Go, Node.js, etc.) and deployed from GitHub with automatic build; environment is clean and controllable.  

**Limitations**

- If there is **no traffic or deployment for 30 days**, the container may be paused and must be restarted manually.  

---

### 3.5 Alwaysdata

**Summary**: Long-standing European PaaS; free tier is feature-rich but small, suitable for lightweight MVPs.

**Specifications**

- Memory: up to 512 MB RAM  
- Storage: 100 MB total space  

**Advantages**

- Free tier includes SSH, PostgreSQL/MySQL, Redis, RabbitMQ, and Cron; supports Node.js, Python, PHP, Ruby. Good for MVPs that only need to store lightweight encrypted variables.  

**Limitations**

- Only 100 MB space; overage requires payment; logs must be rotated or sent elsewhere.  

---

### 3.6 Deta Space

**Summary**: “Personal micro-cloud” and Space App model; Serverless, fully free, no card required.

**Specifications**

- Architecture: Serverless; no fixed RAM limit  
- Built-in: NoSQL (Deta Base), file storage (Deta Drive)  

**Advantages**

- Node.js and Python supported; apps can be published to Space OS so others can one-click install into isolated spaces with data isolation, aligned with end-to-end encryption.  

**Limitations**

- Only Node.js and Python supported natively; Serverless model does not support long-running background processes (limited WebSocket support).  

---

## 4. Recommended Use Cases

### 4.1 Core Backend Deployment (Koyeb)

**Use case**: Host the EnvShield API server (control plane) as a public service.

**Approach**

- Put the API (Go or Node.js) that handles encrypted secrets and access control in a GitHub repo and let Koyeb build and deploy from it.  

**Benefits**

- Stable HTTPS endpoint for the CLI, SDK, or frontend to fetch variables.  

---

### 4.2 Compatibility and Harsh-Environment Testing (Serv00)

**Use case**: Validate CLI and injection behavior on a traditional, resource-limited host.

**Approach**

- Run the CLI on Serv00’s FreeBSD shell (e.g. `shield run npm start`) and verify that it correctly fetches encrypted variables and injects them in a terminal-only, low-resource environment.  

**Benefits**

- Passing this environment demonstrates that the product works in demanding setups and can be used as a product differentiator.  

---

### 4.3 Zero-Install Online Demo (Hugging Face Spaces)

**Use case**: Offer an online demo that does not require installing the NPM package or CLI locally.

**Approach**

- Use Hugging Face Spaces’ Docker environment to build an image that includes Node.js, the EnvShield SDK, and a simple frontend, and expose it as a web demo.  

**Benefits**

- Users can open the Space URL in a browser to see “environment variables securely injected and the app running” without any local setup.  

---

## 5. References

- [Serv00](https://serv00.com/)  
- [Koyeb](https://www.koyeb.com/)  
- [Hugging Face Spaces](https://huggingface.co/spaces)  
- [Back4App](https://www.back4app.com/)  
- [Alwaysdata](https://www.alwaysdata.com/)  
- [Deta Space](https://deta.space/)  

---

## 6. Document Information and Disclaimer

| Item | Description |
|------|-------------|
| Document type | EnvShield project technical reference; not an official contract or specification of any platform |
| Terms and pricing | Governed by each provider’s official site |
| 繁體中文版 | See [free_vps.md](./free_vps.md) |
