# GitHub Branch Protection 建議

本文件提供 EnvShield repo 的 GitHub `main` 分支保護建議。

結論先講：

- 優先使用 GitHub `rulesets`
- 保護目標鎖定 `main`
- 強制 PR 合併
- 強制 `CI` 成功
- 不允許 force push 與刪除分支

## 建議基準

以目前 repo 狀態來看，最實用的設定是：

1. 所有變更都透過 PR 進 `main`
2. `CI` 必須成功
3. `deploy-koyeb.yml` 不列為 merge 前必過條件，因為它是 merge 後部署
4. 先要求 `1` 個 approval，並直接開啟 `Require review from code owners`

## 1. 優先選 rulesets，不是舊式 branch protection

截至 `2026-03-14`，GitHub 同時支援：

- 傳統 branch protection rule
- repository rulesets

對這個 repo，我建議優先用 `Rulesets`，因為：

- 可視性比較好
- 後續擴充到 tag / push policy 比較自然
- GitHub 官方也持續在 rulesets 上增加能力

如果你目前方案或權限限制讓你只能用舊式 branch protection，也可以套用同一組規則。

## 2. 建議保護對象

- Target: `refs/heads/main`
- Enforcement: `Active`
- Bypass: 先留空

推論：
對 EnvShield 目前這種早期產品 repo，最好不要先開 maintainer bypass，避免把流程保護做成形式化存在、實際上常被跳過。

## 3. 建議打開的規則

### 3.1 Pull request 規則

- Require a pull request before merging: 開
- Required approvals: `1`
- Require review from code owners: 開
- Dismiss stale pull request approvals when new commits are pushed: 開
- Require approval of the most recent reviewable push: 開
- Require conversation resolution before merging: 開

### 3.2 Status checks 規則

- Require status checks to pass: 開
- Require branches to be up to date before merging: 開

目前建議設為 required 的 checks：

- `JavaScript`
- `Go`
- `Rust`
- `Shell scripts`
- `Docker image`

這些名稱對應目前 [ci.yml](../.github/workflows/ci.yml) 裡的 job name。

重要：
如果你之後改了 workflow job 名稱，branch protection 裡的 required checks 也要一起更新，不然 merge 會卡住。

### 3.3 Branch safety 規則

- Block force pushes: 開
- Block deletions: 開
- Require linear history: 開

## 4. 目前不建議先打開的規則

以下規則不是不能用，而是我不建議現在就開：

- Require deployments to succeed before merging
- Require signed commits
- Merge queue

原因：

- `Deploy to Koyeb` 是 merge 後部署，不適合當 merge 前 gate。
- `Require review from code owners` 我反而建議現在就打開，因為 repo 已有 [CODEOWNERS](../.github/CODEOWNERS)，而且可以把關基礎治理與敏感檔案。
- signed commits 與 merge queue 比較適合 contributor 或 release 流量更高之後再導入。

## 5. 最小推薦設定清單

如果你只想先做一版最有價值的保護，建議直接這樣設：

1. `main` 必須走 PR
2. 至少 `1` 個 review approval
3. 開啟 `Require review from code owners`
4. conversation 必須 resolved
5. required checks:
   `JavaScript`, `Go`, `Rust`, `Shell scripts`, `Docker image`
6. branch 必須和 base 保持最新
7. 禁止 force push
8. 禁止刪除
9. 啟用 linear history

## 6. GitHub UI 操作建議

建議路徑：

1. Repo `Settings`
2. `Rules`
3. `Rulesets`
4. `New ruleset`
5. 選 `Branch`
6. Target 設 `refs/heads/main`
7. 依本文件勾選規則

如果你使用舊式 branch protection，則路徑通常是：

1. Repo `Settings`
2. `Branches`
3. `Add branch protection rule`
4. Branch name pattern 填 `main`

## 7. 實務注意事項

- required status checks 只有在該 check 近 `7` 天內曾於本 repo 成功跑過，才可被可靠選為 required
- 如果你剛新增 workflow 或剛改 job name，先在 PR 或 `main` 上跑一次 CI，再去設定 required checks
- repo 現在已有 [CODEOWNERS](../.github/CODEOWNERS)，所以可以直接開啟 `code owner review`
- 如果團隊成員增加到 4 人以上，建議把 approval 數量從 `1` 升到 `2`

## 8. EnvShield 目前的建議最終版

對這個 repo，我建議：

- 用 `rulesets`
- 只保護 `main`
- 不開 bypass
- 直接開啟 `Require review from code owners`
- required checks 維持目前 `CI` 的 5 個 job
- `Deploy to Koyeb` 不列入 merge gate

## 9. 官方參考

- GitHub rulesets: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository
- GitHub branch protection: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule
- GitHub protected branches overview: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches
- GitHub available rules for rulesets: https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets
- GitHub required status checks troubleshooting: https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/troubleshooting-required-status-checks
