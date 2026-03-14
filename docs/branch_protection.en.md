# GitHub Branch Protection Recommendations

This document captures the recommended GitHub protection settings for the EnvShield `main` branch.

Bottom line:

- Prefer GitHub `rulesets`
- Protect `main`
- Require pull requests
- Require `CI` to pass
- Block force pushes and branch deletion

## Recommended baseline

For the current repository state, the most practical setup is:

1. All changes reach `main` through pull requests
2. `CI` must succeed
3. `deploy-koyeb.yml` is not a pre-merge requirement because it deploys after merge
4. Start with `1` approval and enable `Require review from code owners`

## 1. Prefer rulesets over legacy branch protection

As of `2026-03-14`, GitHub supports both:

- legacy branch protection rules
- repository rulesets

For this repo, I recommend `Rulesets` first because:

- they are easier to reason about
- they scale better to future tag and push policies
- GitHub is continuing to add functionality there

If your current plan or permissions force you to use legacy branch protection, apply the same policy there.

## 2. Recommended target

- Target: `refs/heads/main`
- Enforcement: `Active`
- Bypass: leave empty for now

Inference:
for an early-stage product repo like EnvShield, it is usually better not to add maintainer bypasses immediately, otherwise the policy becomes easy to bypass under time pressure.

## 3. Recommended enabled rules

### 3.1 Pull request rules

- Require a pull request before merging: on
- Required approvals: `1`
- Require review from code owners: on
- Dismiss stale pull request approvals when new commits are pushed: on
- Require approval of the most recent reviewable push: on
- Require conversation resolution before merging: on

### 3.2 Status check rules

- Require status checks to pass: on
- Require branches to be up to date before merging: on

Recommended required checks right now:

- `JavaScript`
- `Go`
- `Rust`
- `Shell scripts`
- `Docker image`

These names match the current job names in [ci.yml](../.github/workflows/ci.yml).

Important:
if you rename workflow jobs later, update the required checks in branch protection at the same time, or merges may be blocked unexpectedly.

### 3.3 Branch safety rules

- Block force pushes: on
- Block deletions: on
- Require linear history: on

## 4. Rules I do not recommend enabling yet

These are not bad features, but I would not turn them on yet:

- Require deployments to succeed before merging
- Require signed commits
- Merge queue

Why:

- `Deploy to Koyeb` runs after merge, so it should not be a pre-merge gate.
- `Require review from code owners` is the exception here: I do recommend enabling it now because the repo already has [CODEOWNERS](../.github/CODEOWNERS), and it helps protect governance and sensitive project files.
- Signed commits and merge queue usually make more sense once contributor count or merge volume is higher.

## 5. Minimum recommended policy

If you want the highest-value version first, use this set:

1. `main` must go through pull requests
2. at least `1` approval
3. enable `Require review from code owners`
4. conversations must be resolved
5. required checks:
   `JavaScript`, `Go`, `Rust`, `Shell scripts`, `Docker image`
6. branches must be up to date with base
7. force pushes blocked
8. deletions blocked
9. linear history enabled

## 6. GitHub UI path

Recommended path:

1. Repo `Settings`
2. `Rules`
3. `Rulesets`
4. `New ruleset`
5. Choose `Branch`
6. Target `refs/heads/main`
7. Enable the rules from this document

If you are using legacy branch protection instead, the path is usually:

1. Repo `Settings`
2. `Branches`
3. `Add branch protection rule`
4. Branch name pattern: `main`

## 7. Practical notes

- required status checks are only reliably selectable after that check has succeeded in this repository within the last `7` days
- if you just added a workflow or renamed a job, run CI once on a PR or on `main` before configuring required checks
- the repo already has [CODEOWNERS](../.github/CODEOWNERS), so enabling `code owner review` now is reasonable
- once the active maintainer group grows past 4 people, moving from `1` approval to `2` is a reasonable next step

## 8. Final recommendation for EnvShield today

For this repository, I recommend:

- use `rulesets`
- protect only `main`
- do not configure bypasses yet
- enable `Require review from code owners`
- require the current 5 `CI` jobs
- do not make `Deploy to Koyeb` a merge gate

## 9. Official references

- GitHub rulesets: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository
- GitHub branch protection: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule
- GitHub protected branches overview: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches
- GitHub available rules for rulesets: https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets
- GitHub required status checks troubleshooting: https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/troubleshooting-required-status-checks
