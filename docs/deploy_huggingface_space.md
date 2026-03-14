# Hugging Face Space 最小部署說明

本說明使用 repo 根目錄的 `Dockerfile`，將 EnvShield 的 Go control plane 與 React console 以單一 Docker Space 方式部署。

## 適用範圍

- 目標：做出可公開分享的 EnvShield Demo
- 適合：產品展示、介面預覽、基本 API smoke test
- 不適合：正式 secrets control plane

原因：

- 目前 `services/control-plane` 仍使用 in-memory store
- Hugging Face Spaces 閒置會休眠
- Docker Space 的本地磁碟不應視為持久化儲存

## 1. 建立 Docker Space

1. 到 Hugging Face 建立一個新的 Space。
2. SDK 選擇 `Docker`。
3. Hardware 先用免費 CPU 即可。
4. Space 建立完成後，先不要直接用這個 repo 的 `README.md` 覆蓋 Space root，因為 Docker Space 需要自己的 YAML frontmatter。

## 2. 使用 Space README 模板

將 [huggingface_space_readme.template.md](./huggingface_space_readme.template.md) 複製到你的 Space repo root，命名為 `README.md`。

最重要的設定是：

- `sdk: docker`
- `app_port: 8080`

這和目前根目錄 `Dockerfile` 內的 `PORT=8080` 一致。

## 3. 將專案推到 Space repo

最簡單做法是把 Hugging Face Space 當成另一個 git remote：

```bash
git remote add hf-space https://huggingface.co/spaces/<hf-username>/<space-name>
git push hf-space main
```

如果你不想讓主 repo 的 `README.md` 被 Space 用途覆蓋，建議用一個臨時部署分支：

```bash
git checkout -b deploy/hf-space
cp docs/huggingface_space_readme.template.md README.md
git add README.md
git commit -m "chore: prepare hugging face space readme"
git push hf-space deploy/hf-space:main
```

## 4. Space Variables 建議

在 Space Settings -> Variables 裡至少設定：

```text
ENVSHIELD_PUBLIC_URL=https://<your-space-app-url>
```

這個值應該指向 Space 的實際 App URL。

推論：
通常會是類似 `https://<owner>-<space>.hf.space` 的網址，但請以 Hugging Face 顯示的實際 App URL 為準。

目前這個 Demo 不需要額外 secrets 才能啟動；如果之後你把 GitHub OAuth 真正接上，再另外加：

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`

## 5. 部署完成後應該看到什麼

- `/`：EnvShield console
- `/healthz`：健康檢查
- `/v1/*`：control plane API

因為目前是單容器模式，所以不需要再額外開第二個前端服務。

## 6. Demo 模式注意事項

- Space 休眠後，第一次開啟可能會慢一些。
- CLI 的 browser-assisted login 可以展示流程，但不應拿這個 Space 當正式團隊環境。
- 目前資料是記憶體內保存，重新部署或休眠後有可能遺失。

## 7. 官方參考

- Docker Spaces: https://huggingface.co/docs/hub/main/en/spaces-sdks-docker
- Spaces overview: https://huggingface.co/docs/hub/main/en/spaces-overview
- Spaces config reference: https://huggingface.co/docs/hub/spaces-config-reference
