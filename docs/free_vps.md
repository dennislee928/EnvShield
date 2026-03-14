# 免費 VPS 與託管平台指南

本文件說明 EnvShield 專案建議使用的免費 VPS 與 PaaS 選項，供開發、部署與展示使用。

---

## 1. 概述

| 項目 | 說明 |
|------|------|
| 文件版本 | 1.1 |
| 適用對象 | 開發者、維運人員 |
| 用途 | 後端部署、相容性測試、線上展示 |

---

## 2. 推薦平台規格比較

| 平台 | 類型 | RAM | 儲存/資源 | 備註 |
|------|------|-----|-----------|------|
| Serv00 | FreeBSD Shell（類 VPS） | 512 MB | 3 GB | 適合 SSH/殼層測試；不建議作 v1 正式部署目標 |
| Koyeb | 容器化 PaaS | 512 MB | 0.1 vCPU / 2 GB SSD / 1 免費 Web Service | 官方 FAQ 目前提到需信用卡驗證；免費 PostgreSQL 較適合短時展示 |
| Hugging Face Spaces | Docker 空間 | 16 GB | 2 vCPU / 50 GB 非持久磁碟 | 很適合單容器 Demo；閒置會休眠 |
| Back4App Containers | CaaS（容器即服務） | 256 MB | 0.25 CPU / 100 GB 傳輸 / 1 免費容器 | 很適合單容器預覽；長期閒置可能暫停 |
| Alwaysdata | 全能型 PaaS | 256 MB | 1 GB（免費 Cloud 另有限制） | 提供 DB/SSH/Cron，但免費模式限制較多 |
| Deta Space | Serverless / 微型雲 | 未明示 | 依平台配額 | 對目前 Go 常駐 API 架構不理想，僅建議實驗性驗證 |

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

- 目前比較適合殼層、CLI 或 SSH 相容性測試；EnvShield v1 的官方支援目標仍是 macOS 與 Linux，FreeBSD 不建議列為首波正式相容平台。  

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
- 適合直接吃 monorepo 根目錄 Dockerfile，將 Go control plane 與 React console 打包成單一服務。  

**限制**

- 依官方 Pricing FAQ，目前需先做信用卡驗證以防濫用。  
- 官方免費 PostgreSQL 雖可用，但有活躍時間限制，不適合當長時間在線的正式 secrets 資料庫。  

---

### 3.3 Hugging Face Spaces

**簡述**：Hugging Face 提供的 Space 功能可選用 Docker 環境，作為免費後端或展示用伺服器。

**硬體規格**

- 記憶體：最高 16 GB RAM（免費層）  
- CPU：2 vCPU  

**優點**

- 選擇「Docker」環境後，可自訂 Dockerfile，執行 Go API 與靜態 console。  
- 無需信用卡。  

**限制**

- 適合展示，不適合依賴低延遲登入輪詢或持久化 secrets 狀態。  
- 磁碟為非持久性，重建或休眠後不應假設本地檔案仍存在。  

---

### 3.4 Back4App Containers

**簡述**：Back4App 以 BaaS 起家，後提供容器託管（CaaS）服務，免費層免綁信用卡，適合以 Docker 建置之後端。

**硬體規格**

- 免費方案：1 個 Docker 容器  
- 記憶體：256 MB RAM  
- CPU：0.25 CPU  

**優點**

- 完整支援 Docker；後端可打包為 Docker image（Go 或 Node.js），連線 GitHub 即可自動建置與部署，環境乾淨可控。  
- 免綁卡。  

**限制**

- 若超過 **30 天** 無流量或部署活動，容器可能被暫停，需手動重啟。  
- 記憶體較小，較適合單一 Go binary + 靜態資產，不適合再塞獨立 Node SSR 或資料庫。  

---

### 3.5 Alwaysdata

**簡述**：歐洲老牌 PaaS，免費方案採「功能完整、容量精簡」路線，適合輕量 MVP。

**硬體規格**

- 記憶體：256 MB RAM  
- 儲存：1 GB（免費 Cloud 方案另有限制）  

**優點**

- 免費層即提供 SSH、內建 PostgreSQL/MySQL 與 Cron。  
- 支援 Node.js、Python、PHP、Ruby 等。  
- 若未來補上 Postgres adapter，可考慮把輕量控制平面與同機資料庫放在一起，減少外部依賴。  

**限制**

- 官方免費 Cloud 附帶特殊限制，例如自訂網域、遠端資料庫連線與部分使用模式受限；不建議直接假設它等同一般 VPS。  

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
- 對目前以 Go 常駐 HTTP API 為核心的 EnvShield control plane 並不自然；除非另做 Node/Python gateway，否則不建議列為 v1 首選。  

---

## 4. 建議使用情境

### 4.1 核心後端部署（Koyeb）

**情境**：將 EnvShield 的 API 伺服器（控制平面）部署為對外服務。

**作法**

- 直接使用 repo 根目錄 `Dockerfile`，將 Go control plane 與 React console 打成單一容器，交由 Koyeb 從 GitHub 自動建置與部署。  
- 至少設定 `ENVSHIELD_PUBLIC_URL`，讓 CLI 的 browser-assisted login 指回正確網址。  

**效益**

- 提供穩定 HTTPS 端點，供 CLI、SDK 或前端連線取得變數。  

---

### 4.2 相容性與極限環境測試（Serv00）

**情境**：在類傳統主機、資源受限環境下驗證 CLI 與注入行為。

**作法**

- 將 Serv00 改定位為「額外殼層環境」驗證，而非正式支援平台；可先測試 control plane 最小 Go binary、HTTP health check 與 shell 腳本。  
- 若未來要把 Rust CLI 納入 Serv00 測試，需先明確補上 FreeBSD 目標支援與 CI。  

**效益**

- 通過此環境測試可證明產品在各種嚴苛環境下仍可正常運作，利於作為產品說明與賣點。  

---

### 4.3 免安裝線上展示（Hugging Face Spaces）

**情境**：提供不需在本機安裝 NPM 套件或 CLI 的線上 Demo。

**作法**

- 使用 Hugging Face Spaces 的 Docker 環境，直接部署 repo 根目錄 `Dockerfile`，將 console 與 API 以單容器方式提供。  
- 建議將它定位成產品展示站，而不是正式 secrets control plane。  

**效益**

- 潛在使用者僅需開啟 Space 網址，即可在瀏覽器中看到「環境變數被安全注入並成功驅動程式」的展示，降低試用門檻。  

---

## 5. EnvShield 專案部署建議

### 5.1 先講結論

以目前 repo 狀態來看，最實際的免費部署組合是：

| 元件 | 建議平台 | 原因 |
|------|----------|------|
| 單容器 Demo（API + Console） | Hugging Face Spaces / Back4App Containers | repo 已有根目錄 `Dockerfile`，可直接部署單一容器；最少調整即可上線展示。 |
| Staging 控制平面 | Koyeb | 對 Docker 與 GitHub 流程最友善，之後要加自訂網域與正式升級也順。 |
| CLI 發佈 | GitHub Releases + npm / Go wrapper | CLI 不屬於託管服務，應走 release 分發，不建議硬塞到 PaaS。 |
| Shell / 類主機相容性測試 | Serv00 | 適合做 SSH、環境注入、受限主機行為驗證，但不宜當 v1 正式承載平台。 |

### 5.2 依目前程式碼的實際限制

- `services/control-plane` 目前仍採 in-memory store。也就是說，任何平台上的資料在重啟、重新部署或休眠後都有機會消失；現階段比較適合 Demo、preview、QA，而非正式 secrets control plane。
- repo 已新增根目錄 `Dockerfile`，會把 Go control plane 與 React console 打成同一個 runtime image。部署到 Koyeb、Back4App、Hugging Face 時，應優先使用它，而不是拆成兩個免費服務。
- `shield-cli` 的 v1 官方支援目標仍是 macOS 與 Linux。Serv00 是 FreeBSD，因此現階段應視為延伸測試，不是產品承諾範圍。

### 5.3 各平台對 EnvShield 的具體建議

#### Koyeb

- 最適合放 `控制平面 + Console` 的 staging 環境。
- 直接以 repo 根目錄建置 Dockerfile，容器內會由 Go server 同時提供 `/v1/*` 與 `/` 靜態前端。
- 至少設定 `ENVSHIELD_PUBLIC_URL=https://<your-domain>`。
- 等你補上 Postgres store 後，仍建議把正式資料庫放到別處，Koyeb 先扮演 app 層。

#### Hugging Face Spaces

- 最適合對外 Demo，而不是正式後端。
- 使用 Docker Space，將 `app_port` 設為 `8080`，直接吃 repo 根目錄 Dockerfile。
- 因為會休眠，瀏覽器登入批准與 CLI 輪詢流程可能會比較慢；建議把它定位成展示「Console + API 介面存在」而不是正式團隊同步。

#### Back4App Containers

- 很適合做 preview / demo URL，成本低且容器導向清楚。
- 因為 RAM 僅 256 MB，應維持現在的單一 Go binary + 靜態資產模式，不要改成 Node SSR。
- Health check 建議打 `/healthz`。

#### Alwaysdata

- 等你補上 Postgres adapter 後，再考慮拿來做「超輕量 MVP」。
- 優勢在於 DB、SSH、Cron 同場，但免費方案限制比一般 VPS 多，應先確認自訂網域、資料庫存取方式與背景程序政策是否符合你的使用情境。
- 以目前程式碼來說，不是第一個應上線的平台。

#### Serv00

- 先用來驗證 shell 腳本、受限記憶體下的程序啟動，以及未來 Linux/FreeBSD 相容性差異。
- 不建議承載正式 control plane，也不建議在 v1 文件中把它描述為已支援的 CLI 目標環境。

#### Deta Space

- 目前不建議投入 v1。
- 你現在的服務核心是 Go 常駐 API，加上瀏覽器批准流程與未來 snapshot 輪詢；這和 Deta 這類偏 serverless、偏 Node/Python 的模型不夠貼合。

### 5.4 建議的下一步

1. 先把根目錄 `Dockerfile` 部署到 Hugging Face Spaces，做第一個公開 Demo。
2. 再把同一個 Dockerfile 部署到 Koyeb，作為 staging control plane。
3. 下一個工程里程碑應是實作 Postgres store；在那之前，不要把任何免費平台當成正式 secrets 資料庫。

## 6. 參考連結

- [Serv00](https://serv00.com/)  
- [Koyeb](https://www.koyeb.com/)  
- [Hugging Face Spaces](https://huggingface.co/spaces)  
- [Back4App Containers](https://www.back4app.com/)  
- [Alwaysdata](https://www.alwaysdata.com/)  
- [Deta Space](https://deta.space/)  

---

## 7. 依情境分類之其他參考平台

以下平台依使用情境整理，規格與免費額度請以各官網為準。

### 7.1 微服務與核心 API 託管

適用於以 Go、Python 或 Node.js 撰寫之核心業務邏輯與 API。

| 平台 | 簡述 | 連結 |
|------|------|------|
| Adaptable.io | 全端容器；連線 GitHub 自動辨識運行時，免費層內建 PostgreSQL 或 MongoDB，適合儲存金鑰與設定。 | [Adaptable.io](https://adaptable.io/) |
| Choreo | WSO2 出品之開發者平台，免費層大方；支援 Go / Python / Node.js，可視化微服務拓撲。 | [Choreo](https://wso2.com/choreo/) |
| Leapcell | Serverless 託管，支援 Go / Python / Node.js；內建 SQLite 分散式儲存，適合輕量狀態。 | [Leapcell](https://leapcell.io/) |
| Genezio | Node.js / TypeScript 友善；型別安全 RPC，前端或 CLI 可像本地函數般呼叫後端。 | [Genezio](https://genezio.com/) |

### 7.2 邊緣運算與 WebAssembly (Wasm)

適用於 Rust 撰寫之端到端加密或安全模組，編譯為 Wasm 部署。

| 平台 | 簡述 | 連結 |
|------|------|------|
| Fermyon Cloud | 專為 Wasm 設計之 Serverless；Rust 編譯 Wasm 部署，低冷啟動、毫秒級回應。 | [Fermyon Cloud](https://www.fermyon.com/fermyon-cloud) |
| Deno Deploy | 全球邊緣節點；支援 JS/TS 與 Wasm，可載入 Rust 編譯之 Wasm 模組，降低驗證延遲。 | [Deno Deploy](https://deno.com/deploy) |

### 7.3 背景任務與自動化工作流

適用於排程任務、Webhook 觸發、變數同步 Worker 等。

| 平台 | 簡述 | 連結 |
|------|------|------|
| Windmill.dev | 開源；將 Python / Go / TypeScript 腳本化為 API 或排程任務，含 UI，適合變數同步 Worker。 | [Windmill](https://windmill.dev/) |
| Pipedream | 事件驅動；Webhook 觸發 Node.js / Python / Go，可監聽部署事件並觸發變數同步。 | [Pipedream](https://pipedream.com/) |
| Val Town | 輕量 TypeScript；於網頁撰寫 function 即對外為 API，適合格式轉換或簡易 webhook。 | [Val Town](https://www.val.town/) |

### 7.4 自管基礎設施（Kubernetes）

適用於需自行掌控編排與 YAML 之部署。

| 平台 | 簡述 | 連結 |
|------|------|------|
| KubeSail | 免費 Kubernetes 命名空間（免綁卡）；可自建 Docker 與 YAML 部署 Go / Node.js 微服務。 | [KubeSail](https://kubesail.com/) |

---

*本文件為 EnvShield 專案之技術說明，平台條款與計費以各服務官網為準。*
