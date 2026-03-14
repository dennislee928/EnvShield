# Koyeb 實際部署 Runbook

本 runbook 以 repo 根目錄的 `Dockerfile` 為基礎，將 EnvShield 的 Go control plane 與 React console 以單一 Web Service 方式部署到 Koyeb。

## 適用範圍

- 目標：建立一個可對外連線的 staging control plane
- 適合：CLI 整合測試、console 預覽、Demo 環境
- 不適合：正式 production secrets control plane

原因：

- 目前 `services/control-plane` 仍是 in-memory store
- 任何免費平台上的重啟、重新部署、閒置策略，都可能讓資料消失

## 前置條件

1. 這個 repo 已推到 GitHub。
2. 你已建立 Koyeb 帳號並完成必要驗證。
3. 你已安裝 Koyeb CLI。
4. 你已完成登入：

```bash
koyeb login
```

官方文件：

- CLI 安裝: https://www.koyeb.com/docs/build-and-deploy/cli/installation
- Git 建置: https://www.koyeb.com/docs/build-and-deploy/build-from-git

補充：
目前 Koyeb 官方文件主要公開的是 Dashboard、CLI 參數與其他 IaC 流程。repo 根目錄的 [koyeb.yaml](../koyeb.yaml) 是 EnvShield 專案自己的 declarative deploy config，會由 [deploy_koyeb.sh](../scripts/deploy_koyeb.sh) 讀取並轉成 Koyeb CLI 參數。

## 1. 最小手動部署流程

如果你想先用 Dashboard 驗證一次，建議照下面做：

1. 在 Koyeb 建立一個新的 Web Service。
2. Source 選 GitHub repo。
3. Builder 選 `Docker`。
4. Dockerfile path 填 `Dockerfile`。
5. Exposed port 設 `8080:http`。
6. Route 設 `/` 指向 `8080`。
7. Health check 設 `8080:http:/healthz`。
8. Environment Variables 至少加上：

```text
ENVSHIELD_PUBLIC_URL=https://<your-koyeb-domain>
```

9. 部署完成後，確認：

- `/` 可開啟 console
- `/healthz` 回應健康檢查
- `/v1/*` 可回應 control plane API

## 2. Config-based deploy

最建議的用法是先改好 repo 根目錄 [koyeb.yaml](../koyeb.yaml)：

```yaml
app: envshield
service: envshield
repository: github.com/<owner>/<repo>
branch: main
public_url: https://<your-koyeb-domain>
regions: fra
```

然後直接執行：

```bash
./scripts/deploy_koyeb.sh --config koyeb.yaml
```

這樣之後只要改 `koyeb.yaml`，就可以重跑同一條部署流程。

如果你要用 GitHub Actions 自動部署，repo 也已經附上 workflow：

- [ci.yml](../.github/workflows/ci.yml)
- [deploy-koyeb.yml](../.github/workflows/deploy-koyeb.yml)

建議至少設定：

- GitHub Secret: `KOYEB_TOKEN`
- GitHub Variable: `KOYEB_PUBLIC_URL`
- GitHub Variable: `KOYEB_ORGANIZATION`（如果你是用 organization）

流程現在是：

1. `ci.yml` 在 push / PR 時跑純測試與建置驗證。
2. `deploy-koyeb.yml` 只會在 `main` 分支的 `CI` 成功後自動部署。
3. `deploy-koyeb.yml` 也支援手動 `workflow_dispatch`。

## 3. 用 CLI 建立或更新服務

Koyeb CLI 官方 reference 目前支援：

- `--git-builder docker`
- `--git-docker-dockerfile Dockerfile`
- `--ports 8080:http`
- `--routes /:8080`
- `--checks 8080:http:/healthz`

最小指令範例：

```bash
koyeb apps create envshield

koyeb services create envshield \
  --app envshield \
  --git github.com/<owner>/<repo> \
  --git-branch main \
  --git-builder docker \
  --git-docker-dockerfile Dockerfile \
  --type web \
  --instance-type nano \
  --ports 8080:http \
  --routes /:8080 \
  --checks 8080:http:/healthz \
  --env ENVSHIELD_PUBLIC_URL=https://<your-koyeb-domain> \
  --wait
```

更新已存在的服務：

```bash
koyeb services update envshield/envshield \
  --git github.com/<owner>/<repo> \
  --git-branch main \
  --git-builder docker \
  --git-docker-dockerfile Dockerfile \
  --type web \
  --instance-type nano \
  --ports 8080:http \
  --routes /:8080 \
  --checks 8080:http:/healthz \
  --env ENVSHIELD_PUBLIC_URL=https://<your-koyeb-domain> \
  --wait
```

## 4. 直接使用 deploy script

repo 已附上可重複使用的部署腳本：[deploy_koyeb.sh](../scripts/deploy_koyeb.sh)

如果你不想先執行互動式 `koyeb login`，也可以直接提供 token：

```bash
KOYEB_TOKEN=<your-koyeb-pat> ./scripts/deploy_koyeb.sh --config koyeb.yaml
```

最小用法：

```bash
./scripts/deploy_koyeb.sh \
  --config koyeb.yaml
```

如果你想臨時覆蓋 config 裡的值，也可以直接加參數：

```bash
./scripts/deploy_koyeb.sh \
  --config koyeb.yaml \
  --public-url https://<your-koyeb-domain>
```

如果你不想使用 config file，也可以完全用 flags：

```bash
./scripts/deploy_koyeb.sh \
  --app envshield \
  --git github.com/<owner>/<repo> \
  --public-url https://<your-koyeb-domain>
```

常用可選參數：

- `--service <name>`: 自訂 service 名稱，預設與 app 名稱相同
- `--config <path>`: 預設會自動讀取 repo 根目錄 `koyeb.yaml`
- `--git-branch <branch>`: 預設 `main`
- `--region <code>`: 可重複使用，例如 `--region fra --region was`
- `--instance-type <type>`: 預設 `nano`
- `--organization <id>`: 指定 Koyeb organization
- `--wait-timeout <duration>`: 預設 `10m`

## 5. 部署後檢查

部署完成後，先看 Koyeb 狀態：

```bash
koyeb apps get envshield
koyeb services get envshield --app envshield
```

然後做最小 smoke test：

```bash
curl -i https://<your-koyeb-domain>/healthz
```

再確認首頁與 API：

```bash
curl -i https://<your-koyeb-domain>/
curl -i https://<your-koyeb-domain>/v1/devices
```

## 6. 建議設定

- 先維持單一 service，不要拆前後端
- `instance-type` 先用 `nano`
- `health check` 用 `/healthz`
- 若只是 staging，先固定單區域即可

## 7. 已知限制

- 目前 EnvShield control plane 尚未接真實 Postgres store，不適合保存重要資料
- `ENVSHIELD_PUBLIC_URL` 如果未設成真實可公開網址，CLI browser-assisted login 會跳回錯誤位置
- Koyeb 免費方案與驗證規則可能變動，請以官方 Dashboard 與官方文件為準

## 8. 官方參考

- Koyeb CLI installation: https://www.koyeb.com/docs/build-and-deploy/cli/installation
- Koyeb CLI reference: https://www.koyeb.com/docs/build-and-deploy/cli/reference
- Build from Git: https://www.koyeb.com/docs/build-and-deploy/build-from-git
- Health checks: https://www.koyeb.com/docs/run-and-scale/health-checks
