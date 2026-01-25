# GitHub 自动更新配置快速指南

## 🔑 步骤 1: 配置 GitHub Secrets

签名密钥已生成在 `.tauri/ta-ting.key`（此文件不会提交到 Git）

### 添加 Secret 到 GitHub:

1. 打开仓库: https://github.com/ShunL12324/ta-ting/settings/secrets/actions

2. 点击 **New repository secret**

3. 添加私钥:
   - **Name**: `TAURI_SIGNING_PRIVATE_KEY`
   - **Value**: 复制 `.tauri/ta-ting.key` 文件的完整内容

4. （可选）如果设置了密码，添加:
   - **Name**: `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
   - **Value**: 你的密钥密码

## 📦 步骤 2: 发布新版本

```bash
# 1. 更新版本号
# 编辑以下文件:
# - src-tauri/Cargo.toml: version = "0.2.0"
# - src-tauri/tauri.conf.json: "version": "0.2.0"
# - package.json: "version": "0.2.0"

# 2. 提交更改
git add -A
git commit -m "chore: bump version to v0.2.0"
git push

# 3. 创建并推送 tag
git tag v0.2.0
git push origin v0.2.0

# 4. GitHub Actions 会自动:
#    - 构建 Windows 和 macOS 版本
#    - 签名更新包
#    - 创建 Draft Release
#    - 上传安装包和 latest.json

# 5. 在 GitHub Releases 页面发布 Draft
```

## ✅ 步骤 3: 验证

发布 Release 后，检查:

1. Release 包含以下文件:
   - `TaTing_x.x.x_x64_en-US.msi` (Windows)
   - `TaTing_x.x.x_x64.dmg` (macOS)
   - `latest.json` (更新清单)
   - `.sig` 签名文件

2. `latest.json` 格式正确:
   ```json
   {
     "version": "0.2.0",
     "platforms": {
       "windows-x86_64": { ... },
       "darwin-x86_64": { ... }
     }
   }
   ```

## 🔄 步骤 4: 测试自动更新

1. 安装旧版本应用
2. 点击托盘菜单 → "检查更新"
3. 应该能看到新版本提示
4. 点击更新，验证下载和安装流程

## ⚠️ 注意事项

### 保护私钥
- ✅ `.tauri/*.key` 已添加到 `.gitignore`
- ✅ 永远不要提交私钥到 Git
- ✅ 定期备份私钥到安全位置
- ⚠️ 如果丢失私钥，将无法发布更新！

### 版本号规范
- 使用语义化版本: `major.minor.patch`
- Tag 格式: `vX.Y.Z` (如 `v0.2.0`)
- 三个文件的版本号必须一致

### 首次发布
- 第一次发布时，旧版本应用无法自动更新（因为没有更新器）
- 从 v0.2.0 开始，用户可以自动更新到更新版本

## 📚 完整文档

详见: `docs/AUTO_UPDATE_GUIDE.md`

---

**密钥位置**: `.tauri/ta-ting.key` (本地，不提交)
**公钥**: 已配置在 `tauri.conf.json`
**GitHub Actions**: `.github/workflows/release.yml`
