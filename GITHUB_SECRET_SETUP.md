# 配置 GitHub Secret 步骤

## 📋 私钥内容（已为你准备好）

```
dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5Y2dwZFpObEU5d01zbVJRZDl1dXJGL2hBT1BSenRuYmdnaE14dUhNYktUMEFBQkFBQUFBQUFBQUFBQUlBQUFBQVhlQ2ZHbzhPRmxPUlhubjEzL2MwWk1INUVZK2xmdUdsQXJWeFhpbktXa0tuZnZ1QTFqSVpoeHpDd2NBZUNYNmJLcDBUOTVpYWlKazNpb09HeWRkblJYU2RLM05PRGVFaWhmOHVJT2JoVkNIRDFiNnlKSTJORU5ldFNoTytyN1VXc0ZNQjlOSW1PdkE9Cg==
```

## 🔧 配置步骤

### 1. 打开 GitHub Secrets 设置页面

点击这个链接：
👉 https://github.com/ShunL12324/ta-ting/settings/secrets/actions

### 2. 添加新的 Secret

1. 点击右上角的 **"New repository secret"** 按钮

2. 填写信息：
   - **Name**: `TAURI_SIGNING_PRIVATE_KEY`
   - **Value**: 复制上面的私钥内容（整个字符串）

3. 点击 **"Add secret"** 保存

### 3. 验证配置

配置完成后，你应该能在 Secrets 页面看到：
```
TAURI_SIGNING_PRIVATE_KEY
Last updated: just now
```

## ✅ 完成！

配置完成后，你就可以：

1. **发布第一个版本**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. GitHub Actions 会自动：
   - 构建 Windows 和 macOS 版本
   - 使用你的私钥签名更新包
   - 创建 GitHub Release
   - 上传安装包

3. 然后在 GitHub Releases 页面发布 Release

## 📂 备份位置

你的签名密钥已备份到：
- **项目文件夹**: `backup/ta-ting-signing-key.key`
- **说明文档**: `backup/README.md`
- **状态**: ✅ 已添加到 .gitignore（不会被提交）

---

**下一步**: 配置完 GitHub Secret 后，就可以发布第一个版本了！
