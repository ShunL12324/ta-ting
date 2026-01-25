# 自动更新功能配置指南

## 概述

TaTing 使用 Tauri 2.x 的内置更新插件实现自动更新功能，支持从 GitHub Releases 自动下载和安装更新。

## 功能特性

- ✅ 自动检查更新
- ✅ 后台下载更新包
- ✅ 带进度提示的下载过程
- ✅ 自动安装并重启
- ✅ 托盘菜单"检查更新"选项
- ✅ 安全的签名验证（需要配置密钥）

## 工作流程

1. **检查更新**: 应用启动时或用户点击"检查更新"
2. **下载更新**: 发现新版本时从 GitHub Releases 下载
3. **安装更新**: 下载完成后自动安装
4. **重启应用**: 安装完成后自动重启以应用更新

## 配置步骤

### 1. 生成更新签名密钥

首先需要生成用于签名更新文件的密钥对：

```bash
# 安装 Tauri CLI (如果还没有)
npm install -g @tauri-apps/cli

# 生成密钥对
tauri signer generate -w ~/.tauri/ta-ting.key
```

这会生成两个密钥：
- **私钥**: `~/.tauri/ta-ting.key` (保密，用于签名)
- **公钥**: 命令会输出，需要添加到 `tauri.conf.json`

### 2. 配置公钥

将生成的公钥添加到 `src-tauri/tauri.conf.json`:

```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://github.com/ShunL12324/ta-ting/releases/latest/download/latest.json"
      ],
      "dialog": true,
      "pubkey": "YOUR_PUBLIC_KEY_HERE"  // 替换为实际公钥
    }
  }
}
```

### 3. 设置 GitHub Secrets

在 GitHub 仓库设置中添加私钥作为 Secret：

1. 进入仓库 Settings → Secrets and variables → Actions
2. 添加新 Secret:
   - Name: `TAURI_PRIVATE_KEY`
   - Value: 私钥内容 (来自 `~/.tauri/ta-ting.key`)

### 4. 创建 GitHub Actions 工作流

创建 `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      fail-fast: false
      matrix:
        platform: [windows-latest, macos-latest]

    runs-on: ${{ matrix.platform }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies (Ubuntu only)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

      - name: Install frontend dependencies
        run: npm install

      - name: Download models
        run: |
          # Windows
          if [ "${{ matrix.platform }}" == "windows-latest" ]; then
            powershell -File scripts/download-models.ps1
          # macOS/Linux
          else
            bash scripts/download-models.sh
          fi

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
          TAURI_KEY_PASSWORD: ${{ secrets.TAURI_KEY_PASSWORD }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'TaTing ${{ github.ref_name }}'
          releaseBody: 'See CHANGELOG.md for details'
          releaseDraft: true
          prerelease: false
```

### 5. 发布新版本

```bash
# 1. 更新版本号
# 编辑 src-tauri/Cargo.toml 和 src-tauri/tauri.conf.json
# 将 version 改为新版本号，如 "0.2.0"

# 2. 提交更改
git add -A
git commit -m "chore: bump version to v0.2.0"

# 3. 创建 tag
git tag v0.2.0

# 4. 推送到 GitHub
git push origin main --tags

# 5. GitHub Actions 会自动构建并创建 Release
# 6. 在 GitHub Releases 页面发布 Draft
```

## 更新清单格式

GitHub Actions 会自动生成 `latest.json` 文件，格式如下：

```json
{
  "version": "0.2.0",
  "notes": "See CHANGELOG.md for details",
  "pub_date": "2026-01-25T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUld...",
      "url": "https://github.com/ShunL12324/ta-ting/releases/download/v0.2.0/TaTing_0.2.0_x64_en-US.msi.zip"
    },
    "darwin-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUld...",
      "url": "https://github.com/ShunL12324/ta-ting/releases/download/v0.2.0/TaTing_0.2.0_x64.dmg.tar.gz"
    }
  }
}
```

## 用户体验

### 自动检查

应用启动时会自动检查更新（可配置）。

### 手动检查

1. 点击托盘图标
2. 选择"检查更新"
3. 如有更新，显示下载对话框

### 更新流程

1. **检测到更新**: 弹出提示框
2. **用户确认**: 点击"立即更新"
3. **下载中**: 显示进度条
4. **安装**: 下载完成后自动安装
5. **重启**: 3 秒后自动重启应用

## 安全性

- ✅ 所有更新包都经过签名验证
- ✅ 只能安装来自官方 GitHub Release 的更新
- ✅ HTTPS 加密传输
- ✅ 公钥验证确保完整性

## 开发环境测试

在开发环境测试更新功能：

```bash
# 1. 构建当前版本
npm run tauri build

# 2. 修改版本号为更高版本
# 编辑 tauri.conf.json: "version": "0.2.0"

# 3. 再次构建
npm run tauri build

# 4. 手动创建 latest.json 并上传到测试服务器
# 5. 修改 tauri.conf.json 的 endpoints 指向测试服务器
# 6. 运行旧版本，测试更新
```

## 故障排查

### 更新检查失败

- 检查网络连接
- 验证 GitHub Releases 是否存在
- 确认 `latest.json` 文件存在
- 检查公钥配置是否正确

### 签名验证失败

- 确认公钥和私钥匹配
- 检查 `TAURI_PRIVATE_KEY` Secret 配置
- 验证更新文件未被篡改

### 下载失败

- 检查 GitHub Release 文件是否存在
- 验证文件命名格式正确
- 确认网络稳定

## 禁用自动更新

如果需要禁用自动更新，修改 `tauri.conf.json`:

```json
{
  "plugins": {
    "updater": {
      "active": false
    }
  }
}
```

## 相关链接

- [Tauri Updater 文档](https://v2.tauri.app/plugin/updater/)
- [GitHub Actions 工作流](https://docs.github.com/actions)
- [Tauri Action](https://github.com/tauri-apps/tauri-action)

---

**最后更新**: 2026-01-25
