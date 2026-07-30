# Xa缓存加载工具

基于 Rust + egui 的 Windows GUI 工具，用于加载和解压 Xa 缓存文件到目标目录。

## 前置条件

在推送代码到 GitHub 之前，请确保以下文件存在于仓库中：

| 文件 | 说明 | 必需 |
|------|------|------|
| `icon.jpg` | 应用图标，`build.rs` 会自动生成 `icon.ico` | ✅ |
| `assets/hc.7z` | 内置缓存文件，编译时会 `include_bytes!` 嵌入 | ✅ |
| `Cargo.toml` | Rust 项目配置 | ✅ |

> ⚠️ 如果 `assets/hc.7z` 不存在，编译会失败（`include_bytes!` 在编译期检查）。

---

## 🔐 免费代码签名方案

本项目使用 **SignPath Foundation** 为开源项目提供的免费 EV 代码签名证书。签名后的 `.exe` 不会被 Windows SmartScreen 拦截。

### 第一步：注册 SignPath

1. 打开 https://signpath.org/ （SignPath Foundation 页面）
2. 点击 **"Apply for free code signing"**（申请免费代码签名）
3. 填写项目信息：
   - **项目名**: `xa-cache-loader`
   - **仓库地址**: `https://github.com/IsDemon-H/xa-cache-loader`
   - **描述**: Windows GUI tool for extracting Xa cache files, built with Rust and egui
   - **许可证**: 选择合适的开源许可证（推荐 MIT 或 GPL-3.0）
4. 等待 SignPath 审核（通常 1-3 个工作日）

### 第二步：获取 SignPath 凭证

审核通过后，在 SignPath 控制台获取以下信息：

| 参数 | 说明 |
|------|------|
| `Organization ID` | 组织唯一标识 |
| `Project Slug` | 项目标识（如 `xa-cache-loader`） |
| `Signing Policy Slug` | 签名策略标识（如 `release-signing`） |
| `API Token` | 用于 CI 调用的 API 密钥 |

### 第三步：配置 GitHub Secrets

在 GitHub 仓库页面：**Settings → Secrets and variables → Actions → New repository secret**

添加以下 secrets：

| Secret 名 | 值 |
|-----------|-----|
| `SIGNPATH_API_TOKEN` | SignPath 的 API Token |
| `SIGNPATH_ORG_ID` | SignPath 的 Organization ID |

### 第四步：触发签名构建

推送一个版本标签即可触发自动构建 + 签名：

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions 会自动：
1. 编译 Release 版本
2. 提交到 SignPath 进行代码签名
3. 将签名后的 exe 发布为 GitHub Release

也可以手动触发：在 GitHub 仓库的 **Actions → Build and Sign → Run workflow**。

---

## 本地构建

```bash
# 安装 Rust: https://rustup.rs
# 需要 Windows 系统

cargo build --release

# 输出: target/release/xa-cache-loader.exe
```

---

## 备选免费签名方案

如果 SignPath 审核不通过，可以考虑：

### Microsoft Trusted Signing（免费层）

```yaml
# 在 workflow 中添加 signing job
- name: Sign with Microsoft Trusted Signing
  uses: azure/trusted-signing-action@v1
  with:
    endpoint: ${{ secrets.TRUSTED_SIGNING_ENDPOINT }}
    certificate-profile-name: ${{ secrets.CERTIFICATE_PROFILE_NAME }}
    files: "*.exe"
```

需要在 Azure Portal 创建 Trusted Signing Account（Community 层免费，每月 5000 次签名）。

---

## 技术栈

- **Rust** (edition 2024)
- **egui/eframe** 0.31 — GUI 框架
- **zip** + **sevenz-rust** — 压缩文件解压
- **winresource** — Windows 版本信息资源嵌入
- **SignPath** — 免费代码签名
