# 免費 VPS 與託管平台指南

本文件說明 EnvShield 專案建議使用的免費 VPS 與 PaaS 選項，供開發、部署與展示使用。

---

## 1. 概述

| 項目 | 說明 |
|------|------|
| 文件版本 | 1.0 |
| 適用對象 | 開發者、維運人員 |
| 用途 | 後端部署、相容性測試、線上展示 |

---

## 2. 推薦平台規格比較

| 平台 | 類型 | RAM | 儲存/資源 | 備註 |
|------|------|-----|-----------|------|
| Serv00 | FreeBSD Shell（類 VPS） | 512 MB | 3 GB | 需每 3 個月登入保活 |
| Koyeb | 容器化 PaaS | 512 MB | 0.1 vCPU / 1 免費 Web Service | 免綁卡，建議使用活躍 GitHub 帳號 |
| Hugging Face Spaces | Docker 空間 | 16 GB | 2 vCPU | 48 小時無存取會休眠，喚醒約 1 分鐘 |
| Back4App Containers | CaaS（容器即服務） | 250 MB | 0.1 vCPU / 1 免費容器 | 30 天無流量或部署可能暫停，需手動重啟 |
| Alwaysdata | 全能型 PaaS | 512 MB | 100 MB 綜合空間 | 內建 DB/Cron；空間超量須付費，日誌需定期清理 |
| Deta Space | Serverless / 微型雲 | 未明示 | Deta Base + Deta Drive | 僅 Node.js / Python；無常駐程序，WebSocket 支援有限 |

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

**簡述**：Back4App 以 BaaS 起家，後提供容器託管（CaaS）服務，免費層免綁信用卡，適合以 Docker 建置之後端。

**硬體規格**

- 免費方案：1 個 Docker 容器  
- 記憶體：250 MB RAM  
- CPU：0.1 vCPU  

**優點**

- 完整支援 Docker；後端可打包為 Docker image（Go 或 Node.js），連線 GitHub 即可自動建置與部署，環境乾淨可控。  
- 免綁卡。  

**限制**

- 若超過 **30 天** 無流量或部署活動，容器可能被暫停，需手動重啟。  

---

### 3.5 Alwaysdata

**簡述**：歐洲老牌 PaaS，免費方案採「功能完整、容量精簡」路線，適合輕量 MVP。

**硬體規格**

- 記憶體：最高 512 MB RAM  
- 儲存：100 MB 綜合空間  

**優點**

- 免費層即提供 SSH、內建 PostgreSQL/MySQL、Redis、RabbitMQ 及 Cron。  
- 支援 Node.js、Python、PHP、Ruby 等。  
- 對僅儲存輕量加密變數之 MVP，100 MB 通常足夠。  

**限制**

- 空間僅 100 MB，超量須付費；日誌須定期清理或外傳。  

---

### 3.6 Deta Space

**簡述**：以「個人微型雲」為概念，將應用以 Space App 形式發布；完全免費、免綁卡，並強調資料隔離。

**硬體規格**

- 架構：Serverless，無明確 RAM 上限  
- 儲存：內建 NoSQL（Deta Base）與檔案儲存（Deta Drive）  

**優點**

- 完全免費、免綁卡。  
- 支援 Node.js、Python。  
- 應用可發布至 Space OS，其他開發者可一鍵安裝至各自獨立空間，資料隔離，與端到端加密理念相符。  

**限制**

- 僅原生支援 Node.js 與 Python。  
- Serverless 架構下無法跑常駐型背景程序；WebSocket 支援較弱。  

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

## 5. 參考連結

- [Serv00](https://serv00.com/)  
- [Koyeb](https://www.koyeb.com/)  
- [Hugging Face Spaces](https://huggingface.co/spaces)  
- [Back4App Containers](https://www.back4app.com/)  
- [Alwaysdata](https://www.alwaysdata.com/)  
- [Deta Space](https://deta.space/)  

---

*本文件為 EnvShield 專案之技術說明，平台條款與計費以各服務官網為準。*
