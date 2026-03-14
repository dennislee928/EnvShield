# 免費 VPS 與託管平台指南

**文件編號**：ENVSHIELD-DOC-003  
**語言**：繁體中文  
**英文版**：[free_vps.en.md](./free_vps.en.md)

---

## 文件控制

| 欄位 | 內容 |
|------|------|
| 標題 | 免費 VPS 與託管平台指南 |
| 版本 | 1.0 |
| 狀態 | 現行 |
| 適用對象 | 開發者、維運人員 |
| 用途 | 後端部署、相容性測試、線上展示 |

---

## 1. 概述

本文件說明 EnvShield 專案建議使用的免費 VPS 與 PaaS 選項，供開發、部署與展示使用。

---

## 2. 推薦平台規格比較

| 平台 | 類型 | RAM | 儲存/資源 | 備註 |
|------|------|-----|-----------|------|
| Serv00 | FreeBSD Shell（類 VPS） | 512 MB | 3 GB | 需每 3 個月登入保活 |
| Koyeb | 容器化 PaaS | 512 MB | 0.1 vCPU / 1 免費 Web Service | 免綁卡，建議使用活躍 GitHub 帳號 |
| Hugging Face Spaces | Docker 空間 | 16 GB | 2 vCPU | 48 小時無存取會休眠，喚醒約 1 分鐘 |
| Back4App Containers | 容器託管 (CaaS) | 250 MB | 0.1 vCPU / 1 免費容器 | 30 天無流量或部署可能暫停，需手動重啟 |
| Alwaysdata | PaaS（全能型） | 512 MB | 100 MB 綜合空間 | 內建 DB/Cron/SSH；空間小，需定期清日誌 |
| Deta Space | 個人微型雲 / Serverless | 依方案 | Deta Base + Deta Drive | 僅 Node.js/Python；無常駐背景程序 |

---

## 3. 平台說明

### 3.1 Serv00.com

**簡述**：波蘭免費主機，提供 FreeBSD Shell 帳號，權限與操作方式接近傳統 VPS，於開發者社群中廣泛使用。

**硬體規格**

- 記憶體：512 MB RAM  
- 儲存空間：3 GB  

**優點**

- 提供 SSH 存取，可自行開放 Port、執行 Node.js、Go、Python 或架設小型資料庫。  
- 僅需 Email 註冊，無需信用卡。  

**限制**

- 為避免閒置佔用資源，每 **3 個月** 須登入一次控制台（網頁或 SSH 皆可）以維持帳號有效。  

---

### 3.2 Koyeb

**簡述**：容器化 PaaS，支援從 GitHub 自動部署，可視為 Render 的替代方案。

**硬體規格**

- 免費方案：1 個 Web Service  
- 記憶體：512 MB RAM  
- CPU：0.1 vCPU  

**優點**

- 支援 Dockerfile 或直接從 GitHub Repo 部署。  
- 節點可選法蘭克福、華盛頓等。  
- 可使用 GitHub OAuth 登入，免綁信用卡（建議使用具一定使用歷史的 GitHub 帳號）。  

**限制**

- 防濫用機制較嚴，建議以活躍的 GitHub 帳號註冊。  

---

### 3.3 Hugging Face Spaces

**簡述**：Hugging Face 提供的 Space 功能可選用 Docker 環境，作為免費後端或展示用伺服器。

**硬體規格**

- 記憶體：最高 16 GB RAM（免費層）  
- CPU：2 vCPU  

**優點**

- 選擇「Docker」環境後，可自訂 Dockerfile，執行 Node.js、Go 等後端程式並對外提供 API。  
- 無需信用卡。  

**限制**

- 若 **48 小時** 內無任何存取，Space 會進入休眠；下次存取時需等待約 **1 分鐘** 喚醒。  

---

### 3.4 Back4App Containers

**簡述**：Back4App 以 BaaS 起家，後提供容器託管 (CaaS)，免費層免綁卡，適合需 Docker 環境之後端。

**硬體規格**

- 免費方案：1 個 Docker 容器  
- 記憶體：250 MB RAM  
- CPU：0.1 vCPU  

**優點**

- 完整支援 Docker；後端可打包為 Docker image（Go、Node.js 等），連線 GitHub 即可自動建置與部署，環境乾淨可控。  

**限制**

- 若 **30 天** 內無流量或部署活動，容器可能被暫停，需手動重啟。  

---

### 3.5 Alwaysdata

**簡述**：歐洲老牌 PaaS，免費方案為「全能但容量小」，適合輕量 MVP。

**硬體規格**

- 記憶體：最高 512 MB RAM  
- 儲存空間：100 MB 綜合空間  

**優點**

- 免費層即提供 SSH、內建 PostgreSQL/MySQL、Redis、RabbitMQ、Cron；支援 Node.js、Python、PHP、Ruby。適合僅需儲存輕量加密變數之 MVP。  

**限制**

- 空間僅 100 MB，超出須付費；日誌需定期清理或外傳。  

---

### 3.6 Deta Space

**簡述**：以「個人微型雲」與 Space App 概念運作，Serverless 架構，完全免費、免綁卡。

**硬體規格**

- 架構：Serverless，無明確 RAM 上限  
- 內建：NoSQL（Deta Base）、檔案儲存（Deta Drive）  

**優點**

- 支援 Node.js、Python；應用可發布至 Space OS，其他開發者可一鍵安裝至獨立空間，資料隔離，與端到端加密理念相符。  

**限制**

- 僅原生支援 Node.js 與 Python；Serverless 架構下無法跑常駐型背景程序（WebSocket 支援較弱）。  

---

## 4. 建議使用情境

### 4.1 核心後端部署（Koyeb）

**情境**：將 EnvShield 的 API 伺服器（控制平面）部署為對外服務。

**作法**

- 將負責加密密文、帳號與權限管理的 API（Go 或 Node.js）置於 GitHub Repo，由 Koyeb 從 Repo 自動建置與部署。  

**效益**

- 提供穩定 HTTPS 端點，供 CLI、SDK 或前端連線取得變數。  

---

### 4.2 相容性與極限環境測試（Serv00）

**情境**：在類傳統主機、資源受限環境下驗證 CLI 與注入行為。

**作法**

- 於 Serv00 的 FreeBSD Shell 中執行 CLI（例如 `shield run npm start`），驗證在純終端、無圖形介面、資源有限環境下是否能正確拉取加密變數並注入。  

**效益**

- 通過此環境測試可證明產品在各種嚴苛環境下仍可正常運作，利於作為產品說明與賣點。  

---

### 4.3 免安裝線上展示（Hugging Face Spaces）

**情境**：提供不需在本機安裝 NPM 套件或 CLI 的線上 Demo。

**作法**

- 使用 Hugging Face Spaces 的 Docker 環境，建置包含 Node.js、EnvShield SDK 與簡易前端的 Dockerfile，對外提供網頁版展示。  

**效益**

- 潛在使用者僅需開啟 Space 網址，即可在瀏覽器中看到「環境變數被安全注入並成功驅動程式」的展示，降低試用門檻。  

---


太熱血了！身為一個開發者，尋找那些「冷門、新銳、甚至帶點極客色彩 (Geeky)」的平台，本身就是一件超好玩的事！

既然你要走**「微服務混搭 (Node.js, Go, Python, Rust)」**的路線，我們就可以徹底拋棄那些笨重的老牌雲端，來看看最近在開源社群和獨立開發者圈子裡悄悄崛起的「新世代神兵利器」。

在揭曉這份冷門名單之前，我想先釐清一個關於部署流程 (CI/CD) 的細節：
🚀 你習慣用什麼方式把寫好的微服務推上線？ 是喜歡「Push 到 GitHub 就自動幫你搞定一切 (GitOps)」，還是喜歡在本地終端機敲一個 CLI 指令 就把程式碼射上雲端？這會決定哪個平台最合你的胃口！

準備好了嗎？為你奉上三個極具潛力、免綁卡且完美契合微服務架構的新銳/冷門廠牌：

1. Shuttle (Rust 開發者的專屬外掛)
如果你在微服務架構中，決定用 Rust 來寫最核心的「加密與解密引擎」，那你絕對不能錯過 Shuttle！

硬體資源：提供免費的社群方案，只要用 GitHub 登入即可，免綁卡。

亮點優勢：它是**「Infrastructure from Code (從程式碼生成基礎設施)」**的極致。你不需要寫 Dockerfile，也不用管伺服器設定。你只要在 Rust 程式碼裡加一行 Macro（例如 #[shuttle_runtime::main]），敲下 cargo shuttle deploy，它就會自動幫你在雲端把 Rust 服務架起來！

微服務玩法：把最高強度、最耗 CPU 的端到端加密微服務交給 Shuttle，它絕對是你的 Rust 專武。

2. Zeabur (多語言微服務的無痛樂園)
這是一個非常新銳的 PaaS 平台（而且創辦團隊有台灣人喔！），在獨立開發者圈口碑極佳。

硬體資源：提供免費的 Developer 方案（大約每月有 5 美元的免費額度直接扣抵），免綁卡，用 GitHub 註冊即可。

亮點優勢：天生為微服務而生！ 你可以在同一個「專案 (Project)」裡面，同時部署你的 Node.js 儀表板、Go API 核心，甚至是現成的 PostgreSQL 資料庫。Zeabur 最強的是它的「服務發現 (Service Discovery)」，你的 Go 服務要呼叫 Node.js 服務，完全不用設定複雜的網路，它們在內部網路會自動互相認識。

微服務玩法：把你那些需要頻繁互相溝通的 Node.js 和 Go 服務通通丟進 Zeabur 的同一個專案裡，省去跨平台 API 呼叫的麻煩。

3. Sealos (把 K8s 變成網頁遊戲的黑馬雲作業系統)
這是一個非常硬核但又極度創新的平台，它把複雜的 Kubernetes (K8s) 包裝成了一個「像網頁版 Windows/macOS」的雲端作業系統。

硬體資源：免綁卡（可用 GitHub 或微信登入），註冊會贈送初始額度，且有社群活躍獎勵或免費配額，足以支撐小型微服務長期運行。

亮點優勢：視覺化部署 Docker 容器。你登入後看到的是一個桌面，點開「App Launchpad (應用啟動器)」，輸入你的 Docker Image 網址（不管是 Python 還是 Go 打包的），它就會在底層的 K8s 叢集幫你跑起來，還自動幫你配好 HTTPS 網域。

微服務玩法：適合用來部署你用 Go 或 Python 寫的「背景同步 Worker」，你可以像在電腦桌面上管理視窗一樣，直觀地監控每個微服務的 CPU 和記憶體消耗。
---
## 5. 參考連結

- [Serv00](https://serv00.com/)  
- [Koyeb](https://www.koyeb.com/)  
- [Hugging Face Spaces](https://huggingface.co/spaces)  
- [Back4App](https://www.back4app.com/)  
- [Alwaysdata](https://www.alwaysdata.com/)  
- [Deta Space](https://deta.space/)  

---

## 6. 文件資訊與免責聲明

| 項目 | 說明 |
|------|------|
| 文件性質 | EnvShield 專案技術說明，非各平台正式合約或規格書 |
| 條款與計費 | 以各服務官網公告為準 |
| 英文版 | 見 [free_vps.en.md](./free_vps.en.md) |
