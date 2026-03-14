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
| Back4App Containers | CaaS (container) | 250 MB | 0.1 vCPU / 1 free container | May pause after 30 days without traffic/deploys; restart manually |
| Alwaysdata | Full-featured PaaS | 512 MB | 100 MB total space | Built-in DB/Cron; overage paid; logs must be rotated |
| Deta Space | Serverless / micro-cloud | Not specified | Deta Base + Deta Drive | Node.js/Python only; no long-running processes; limited WebSocket |

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

**Summary**: Back4App started as BaaS and later added container hosting (CaaS) with a no–credit-card free tier, suitable for Docker-based backends.

**Specifications**

- Free tier: 1 Docker container  
- Memory: 250 MB RAM  
- CPU: 0.1 vCPU  

**Advantages**

- Full Docker support; backends can be packaged as Docker images (Go or Node.js) and deployed from GitHub with automatic build and deploy; clean, controllable environment.  
- No credit card required.  

**Limitations**

- If there is **no traffic or deploy activity for 30 days**, the container may be paused and require manual restart.  

---

### 3.5 Alwaysdata

**Summary**: Long-standing European PaaS with a free tier that is feature-rich but capacity-limited; suitable for lightweight MVPs.

**Specifications**

- Memory: up to 512 MB RAM  
- Storage: 100 MB total space  

**Advantages**

- Free tier includes SSH, built-in PostgreSQL/MySQL, Redis, RabbitMQ, and Cron.  
- Supports Node.js, Python, PHP, Ruby, and others.  
- For an MVP that only stores lightweight encrypted variables, 100 MB is often sufficient.  

**Limitations**

- Only 100 MB space; overage is paid; logs must be rotated or sent elsewhere.  

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
- Serverless model does not support long-running background processes; WebSocket support is limited.  

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
- [Back4App Containers](https://www.back4app.com/)  
- [Alwaysdata](https://www.alwaysdata.com/)  
- [Deta Space](https://deta.space/)  

---

*This document is technical guidance for the EnvShield project. Terms and pricing are determined by each provider’s official site.*
