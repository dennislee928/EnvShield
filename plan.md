# EnvShield 三階段產品線與首版 Monorepo 規格

## 摘要
- 將三個構想整合成同一條產品線：Phase 1 做 `EnvShield`（E2EE CLI 注入器），Phase 2 擴成 `SyncFlow`（多雲同步與審核），Phase 3 再推出 `SecretSDK`（程式碼級動態取用）。
- 首版 repo 直接建立 monorepo，但只把可用功能做到 Phase 1：Rust CLI、Go control plane、TypeScript Web Console、NPM wrapper、Go wrapper。
- 第一個商業切入點鎖定個人開發者；資料模型從一開始就支援 workspace、device、member、snapshot，方便後續擴到團隊功能。
- 安全模型固定為「secret value 零知識、metadata 可讀」：secret value 只在 CLI 或瀏覽器端加解密，伺服器僅保存密文與可讀的 workspace / environment / version / secret name / checksum 等中繼資料。

## 核心實作
- CLI 是 v1 唯一可編輯 secret 的入口；Web Console v1 只做 GitHub OAuth、workspace onboarding、device 管理、snapshot 狀態與差異檢視，不在 v1 編輯 secret value。
- CLI 登入採 browser-assisted device flow：`shield login` 開瀏覽器完成 GitHub OAuth，後端發給 CLI device token；auth 架構預留 passkey provider，但 v1 不實作。
- 每台 device 在本地生成 X25519 加密金鑰與 Ed25519 簽章金鑰；每個 snapshot 產生隨機 256-bit data key；每個 secret value 用 XChaCha20-Poly1305 加密；data key 對授權 device 做 envelope encryption；snapshot manifest 由上傳 device 簽章。
- v1 CLI 指令固定為 `shield login`、`shield init`、`shield secret set`、`shield secret list`、`shield push`、`shield pull`、`shield status`、`shield run -- <command...>`；`shield run` 只能把解密後變數注入子程序記憶體，不能產生 `.env`。
- 動態快照提醒在 v1 先做輪詢：`run`、`pull`、`status` 都會檢查最新 snapshot version，若本地落後就提醒同步；API 事件模型與資料表預留 SSE/WebSocket 升級點，但即時推播不在 v1 實作。
- NPM 與 Go wrappers 都是 thin wrapper：只負責安裝或定位對應平台的 Rust binary，並透明轉發同一套 CLI 參數，不各自重寫 secret 邏輯。
- Go control plane 以 Postgres 為主要資料庫；v1 直接將 metadata 與小型 ciphertext 存 Postgres，並保留 blob storage 抽象層給後續大型 snapshot 使用。
- Phase 2 在同一 control plane 上新增 Vercel、Render、GitHub Actions connectors、approval flow、audit log、SSE 通知；Phase 3 復用同一 snapshot 與 key-envelope 模型，提供 Node、Go、Python SDK。

## 公開介面與型別
- CLI 合約：`shield init --workspace <name>` 建立 workspace 並註冊 device public keys；`shield secret set <KEY> --env <env>` 互動式輸入 value；`shield push --env <env>` 上傳 snapshot；`shield pull --env <env>` 下載並解密到本地安全快取；`shield run --env <env> -- <cmd...>` 將解密值注入子程序。
- API 合約：提供 GitHub OAuth 啟動與 callback、device token exchange、workspace 建立與查詢、device 註冊與列出、snapshot 建立、最新 snapshot 讀取、environment status 查詢。
- Snapshot 型別固定包含 `snapshot_id`、`workspace_id`、`environment`、`version`、`created_by_device`、`created_at`、`manifest_signature`、`key_envelopes[]`、`secrets[]`。
- Secret item 型別固定包含 `name`、`ciphertext`、`nonce`、`aad_hash`、`value_checksum`；`name` 在 v1 視為可讀 metadata，不做名稱盲化。
- Monorepo 需要有共享 API contract 層，讓 Go API 成為 source of truth，TypeScript Web 與 wrappers 由同一份 OpenAPI/JSON schema 生成 client 與型別。

## 測試計畫
- Crypto 單元測試必須覆蓋：正確 key 可解密、錯誤 key 解密失敗、manifest 驗章失敗會阻止 `pull` 與 `run`。
- CLI 測試必須覆蓋：`shield run` 能成功注入 child process、工作目錄不出現 `.env`、異常中斷後仍執行敏感記憶體清理。
- API 整合測試必須覆蓋：GitHub OAuth 完成後，CLI 可成功 exchange token、建立 workspace、推送 snapshot、拉取 snapshot。
- Snapshot 測試必須覆蓋：本地版本落後時 `run` 與 `status` 會顯示提醒；版本一致時不影響啟動流程。
- Wrapper 測試必須覆蓋：NPM 與 Go wrappers 對 flags 的透明轉發、版本對齊、下載 checksum 驗證與平台偵測。
- Phase 邊界測試必須確認：v1 雖只開單人 workspace 體驗，但資料模型已可向後相容地擴到多 member、approval、connector 與 SDK fetch 流程。

## 假設與預設
- v1 支援 macOS 與 Linux，Windows 延後。
- v1 先驗證「不落地 `.env` 的本地 DX + 零知識 secrets」；團隊審核流、即時推播、多雲同步都屬於 Phase 2。
- Web Console v1 只承擔登入、管理與觀察，不承擔 secret 編輯，避免首版同時承受完整瀏覽器端加密 UX 複雜度。
- GitHub OAuth 是 v1 唯一登入方式；passkey 僅保留資料模型與前端入口位，不進入首版實作。
