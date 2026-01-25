# GitHub Actions 工作流说明

## 📋 配置的工作流

### 1. Build on Push (`.github/workflows/build.yml`)

**触发条件**: 每次 commit 推送到 `master` 或 `main` 分支

**功能**:
- ✅ 自动编译检查
- ✅ 测试构建是否成功
- ✅ 上传构建产物（保留 7 天）
- ❌ 不创建 Release
- ❌ 不发布版本

**用途**: 持续集成（CI），确保代码始终可以编译成功

**构建平台**:
- Windows (x64)
- macOS (Universal)

**产物**:
- 保存在 GitHub Actions Artifacts
- 可以从 Actions 页面下载测试

---

### 2. Release Build (`.github/workflows/release.yml`)

**触发条件**: 推送 tag（如 `v0.1.0`）

**功能**:
- ✅ 自动编译构建
- ✅ 下载模型文件
- ✅ 签名更新包
- ✅ **直接发布** GitHub Release（非 Draft）
- ✅ 上传安装包和 `latest.json`

**用途**: 正式版本发布

**版本类型**:
- `v0.1.0` → 正式版本 (Release)
- `v0.1.0-alpha` → 预发布版本 (Prerelease)
- `v0.1.0-beta` → 预发布版本 (Prerelease)
- `v0.1.0-rc.1` → 预发布版本 (Prerelease)

---

## 🚀 使用方法

### 日常开发（自动编译测试）

```bash
# 正常提交代码
git add -A
git commit -m "feat: add new feature"
git push

# GitHub Actions 会自动：
# 1. 编译 Windows 和 macOS 版本
# 2. 上传构建产物
# 3. 但不发布 Release
```

查看构建状态：
```bash
gh run list
# 或访问
# https://github.com/ShunL12324/ta-ting/actions
```

---

### 发布正式版本

```bash
# 1. 更新版本号（3个文件）
# - src-tauri/Cargo.toml: version = "0.1.0"
# - src-tauri/tauri.conf.json: "version": "0.1.0"
# - package.json: "version": "0.1.0"

# 2. 提交版本更新
git add -A
git commit -m "chore: bump version to v0.1.0"
git push

# 3. 创建并推送 tag
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions 会自动：
# 1. 编译 Windows 和 macOS 版本
# 2. 下载 Sherpa-ONNX 模型
# 3. 签名更新包
# 4. 创建并发布 GitHub Release（自动发布，非 Draft）
# 5. 上传安装包和 latest.json
```

发布后，用户可以：
- 访问 https://github.com/ShunL12324/ta-ting/releases 下载
- 应用内"检查更新"自动更新

---

### 发布测试版本

```bash
# 发布 Alpha 版本
git tag v0.2.0-alpha
git push origin v0.2.0-alpha

# 发布 Beta 版本
git tag v0.2.0-beta.1
git push origin v0.2.0-beta.1

# 这些会被标记为 Prerelease（预发布版本）
```

---

## 📦 构建产物

### Build on Push（每次 commit）

构建产物保存在 Actions Artifacts：

1. 访问: https://github.com/ShunL12324/ta-ting/actions
2. 点击最近的 workflow run
3. 下载 Artifacts:
   - `windows-build` - Windows 安装包
   - `macos-build` - macOS 安装包

**保留时间**: 7 天

---

### Release Build（tag）

构建产物发布到 GitHub Releases：

访问: https://github.com/ShunL12324/ta-ting/releases

包含文件:
- `TaTing_0.1.0_x64_en-US.msi` (Windows)
- `TaTing_0.1.0_x64_en-US.msi.zip` (Windows 压缩)
- `TaTing_0.1.0_x64_en-US.msi.zip.sig` (签名)
- `TaTing_0.1.0_x64.dmg` (macOS)
- `TaTing_0.1.0_x64.dmg.tar.gz` (macOS 压缩)
- `TaTing_0.1.0_x64.dmg.tar.gz.sig` (签名)
- `latest.json` (更新清单)

**保留时间**: 永久

---

## 🔍 监控构建

### 使用 gh CLI

```bash
# 查看最近的 workflow 运行
gh run list

# 查看特定 run 的详情
gh run view <run-id>

# 查看日志
gh run view <run-id> --log

# 下载 artifacts
gh run download <run-id>
```

### 使用网页

访问: https://github.com/ShunL12324/ta-ting/actions

- ✅ 绿色勾 - 构建成功
- ❌ 红色叉 - 构建失败
- 🟡 黄色圈 - 构建中

---

## ⚙️ 工作流配置

### 构建时间

- **Windows**: 约 8-12 分钟
- **macOS**: 约 10-15 分钟
- **总计**: 约 15-20 分钟

### 并发构建

两个平台并行构建，互不影响。

### 失败处理

如果构建失败：
1. 检查 Actions 页面的错误日志
2. 本地复现问题
3. 修复后重新推送

---

## 🎯 最佳实践

### 版本号规范

使用语义化版本（Semantic Versioning）：

- `v1.0.0` - 主版本.次版本.补丁版本
- `v1.0.0-alpha` - Alpha 测试版
- `v1.0.0-beta.1` - Beta 测试版（带序号）
- `v1.0.0-rc.1` - Release Candidate

### 发布流程

1. **开发阶段**:
   - 正常 commit 和 push
   - 每次 push 自动编译测试
   - 不影响用户

2. **测试阶段**:
   - 发布 alpha/beta 版本
   - 标记为 Prerelease
   - 小范围测试

3. **正式发布**:
   - 发布正式版本
   - 自动发布 Release
   - 用户可以自动更新

---

## 📝 示例时间线

```
Day 1:
  10:00 - commit: "feat: add feature A"
         → Auto build & test ✅

  14:00 - commit: "fix: bug in feature A"
         → Auto build & test ✅

Day 2:
  09:00 - commit: "chore: bump version to v0.1.0"
         → Auto build & test ✅

  09:05 - tag: v0.1.0
         → Auto build & release ✅
         → 用户可以下载 v0.1.0

Day 3:
  11:00 - commit: "feat: add feature B"
         → Auto build & test ✅
         (不发布，继续开发)

Day 7:
  15:00 - tag: v0.2.0
         → Auto build & release ✅
         → 旧版本用户收到更新提示
```

---

**配置完成时间**: 2026-01-25
**工作流文件**:
- `.github/workflows/build.yml` (每次 commit)
- `.github/workflows/release.yml` (tag 发布)
