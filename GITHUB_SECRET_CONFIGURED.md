# ✅ GitHub Secret 配置完成

## 🎉 配置状态

GitHub Secret 已成功配置：

- **Secret 名称**: `TAURI_SIGNING_PRIVATE_KEY`
- **配置时间**: 2026-01-25 21:33:36
- **状态**: ✅ 已激活
- **用途**: 用于签名应用更新包

## 📋 验证

运行以下命令可以看到已配置的 Secret：

```bash
gh secret list
```

输出：
```
TAURI_SIGNING_PRIVATE_KEY    2026-01-25T13:33:36Z
```

## 🚀 现在可以发布版本了！

### 发布第一个正式版本（v0.1.0）

```bash
# 1. 创建版本 tag
git tag v0.1.0

# 2. 推送 tag 到 GitHub
git push origin v0.1.0

# 3. GitHub Actions 会自动：
#    - 构建 Windows 和 macOS 版本
#    - 下载 Sherpa-ONNX 模型
#    - 使用私钥签名更新包
#    - 创建 GitHub Release (Draft)
#    - 上传安装包和 latest.json

# 4. 等待构建完成（约 10-15 分钟）

# 5. 在 GitHub 网页上发布 Release
#    https://github.com/ShunL12324/ta-ting/releases
```

### 或者发布 Alpha 测试版本

```bash
# 如果还在测试阶段，可以发布 alpha 版本
git tag v0.1.0-alpha
git push origin v0.1.0-alpha

# 这会创建一个预发布版本
```

## 📦 GitHub Actions 工作流

当你推送 tag 后，GitHub Actions 会自动运行：

**工作流文件**: `.github/workflows/release.yml`

**构建平台**:
- ✅ Windows (x64)
- ✅ macOS (Universal - Intel + Apple Silicon)

**输出文件**:
- `TaTing_0.1.0_x64_en-US.msi` (Windows 安装包)
- `TaTing_0.1.0_x64.dmg` (macOS 安装包)
- `latest.json` (更新清单)
- `.sig` 签名文件

## 🔍 监控构建进度

查看构建状态：

```bash
# 查看最近的 workflow 运行
gh run list

# 查看特定 workflow 的日志
gh run view --log
```

或访问：
https://github.com/ShunL12324/ta-ting/actions

## ✨ 自动更新测试

发布 Release 后，用户可以：

1. 安装应用
2. 点击托盘图标 → "检查更新"
3. 如果有新版本，会显示更新提示
4. 点击"立即更新"自动下载安装

## 📝 后续版本发布

每次发布新版本：

1. 更新版本号（3个文件）:
   - `src-tauri/Cargo.toml`
   - `src-tauri/tauri.conf.json`
   - `package.json`

2. 提交并创建 tag:
   ```bash
   git add -A
   git commit -m "chore: bump version to v0.2.0"
   git push
   git tag v0.2.0
   git push origin v0.2.0
   ```

3. GitHub Actions 自动构建

4. 在 GitHub 发布 Release

## 🔐 安全提醒

- ✅ 私钥已安全存储为 GitHub Secret
- ✅ 只有仓库管理员可以查看
- ✅ 每次构建都会验证签名
- ✅ 本地备份在 `backup/` 目录

---

**配置完成时间**: 2026-01-25 21:33
**下一步**: 准备好就可以发布第一个版本了！🚀
