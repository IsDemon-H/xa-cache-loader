# Xa缓存加载工具

基于 Rust + egui 的 Windows GUI 工具，用于加载和解压 Xa 缓存文件到目标目录。

## 前置条件

将代码推送到 GitHub 之前，确保以下文件存在于仓库中：

| 文件 | 说明 | 必需 |
|------|------|------|
| `icon.jpg` | 应用图标，`build.rs` 自动生成 `icon.ico` | ✅ |
| `assets/hc.7z` | 内置缓存文件，编译时嵌入 | ✅ |

---

## 🔐 免费代码签名：Microsoft Trusted Signing

微软官方提供的免费代码签名服务，签名后的 `.exe` 走微软证书链，SmartScreen 直接信任，**零拦截**。

| 对比 | Microsoft Trusted Signing | SignPath Foundation |
|------|--------------------------|---------------------|
| 费用 | ✅ 免费 (5000次/月) | ✅ 免费 |
| 审核 | ✅ **无需审核** | ❌ 严格审核 |
| 要求 | 无 | 项目需被广泛使用 |
| 证书链 | 微软官方证书 | 第三方 EV 证书 |

---

### 📋 一次性配置（约 10 分钟）

#### 1. 注册 Azure 账号

打开 https://portal.azure.com → 用 GitHub 账号或邮箱注册（免费，无需信用卡）。

#### 2. 创建 Trusted Signing Account

在 Azure Portal 顶部搜索栏输入 `Trusted Signing` → 点击 **"信任签名帐户"**：

| 字段 | 填写 |
|------|------|
| 订阅 | 选择你的订阅 |
| 资源组 | 新建，命名 `xa-cache-loader` |
| 帐户名称 | `xa-cache-loader` |
| 定价层 | **Community**（免费层） |
| 区域 | `East US` 或任意 |

点击"审阅并创建" → "创建"。

#### 3. 创建证书配置文件

进入刚创建的 Trusted Signing Account → **证书配置文件** → **添加**：

| 字段 | 填写 |
|------|------|
| 配置文件名称 | `release` |
| 证书类型 | **Public-Trust**（公开信任） |
| 主体 | 留空 |

创建完成。

#### 4. 配置 OIDC（让 GitHub Actions 免密码访问 Azure）

在 Azure Portal 搜索 `Microsoft Entra ID` → **应用注册** → **新注册**：

| 字段 | 填写 |
|------|------|
| 名称 | `xa-cache-loader-ci` |
| 支持的帐户类型 | 仅此组织目录中的帐户 |

注册完成后，进入该应用 → **证书和密码** → **联合凭据** → **添加凭据**：

| 字段 | 填写 |
|------|------|
| 联合凭据方案 | `GitHub Actions` |
| 组织 | `IsDemon-H` |
| 仓库 | `xa-cache-loader` |
| 实体类型 | `分支` |
| GitHub 分支名称 | `main` |
| 名称 | `release-signing` |

点击"添加"。

回到应用**概览**页面，记下：
- `应用程序(客户端) ID`（一串 UUID）

在 Azure Portal 搜索 `订阅` → 记下你的 `订阅 ID`。

在 Microsoft Entra ID 概览页面，记下 `租户 ID`。

#### 5. 授权 Trusted Signing 访问

进入 Trusted Signing Account → **访问控制(IAM)** → **添加角色分配**：

| 字段 | 填写 |
|------|------|
| 角色 | `Trusted Signing Certificate Profile Signer` |
| 成员 | 选择 `xa-cache-loader-ci`（步骤4注册的应用） |

保存。

---

### 🔐 设置 GitHub Secrets

在仓库 **Settings → Secrets and variables → Actions → New repository secret**，添加 3 个 secrets：

| Secret 名 | 值 |
|-----------|-----|
| `TRUSTED_SIGNING_ENDPOINT` | `https://eus.codesigning.azure.net`（根据你的区域，见下表） |
| `TRUSTED_SIGNING_ACCOUNT` | `xa-cache-loader` |
| `CERTIFICATE_PROFILE_NAME` | `release` |

以及 **Actions → Variables** 中添加 3 个变量：

| 变量名 | 值 |
|--------|-----|
| `AZURE_CLIENT_ID` | 步骤4 记下的应用程序(客户端) ID |
| `AZURE_TENANT_ID` | 步骤4 记下的租户 ID |
| `AZURE_SUBSCRIPTION_ID` | 步骤4 记下的订阅 ID |

**端点区域对照表**：

| 区域 | 端点 URL |
|------|----------|
| East US | `https://eus.codesigning.azure.net` |
| West US | `https://wus.codesigning.azure.net` |
| West Europe | `https://weu.codesigning.azure.net` |

---

### 🚀 触发签名发布

```bash
git tag v1.0.0
git push origin v1.0.0
```

推送标签后，GitHub Actions 自动：
1. 编译 Release 版本
2. 调用 Microsoft Trusted Signing 签名
3. 发布签名后的 exe 到 GitHub Release

也可以在 **Actions → Build and Sign → Run workflow** 手动触发（签名产物会作为 artifact 下载）。

---

## 本地构建

```bash
cargo build --release
# 输出: target/release/xa-cache-loader.exe
```

本地构建的 exe **没有签名**，会触发 SmartScreen。签名只在 GitHub Actions 中进行。

---

## 技术栈

- **Rust** (edition 2024)
- **egui/eframe** 0.31 — GUI 框架
- **zip** + **sevenz-rust** — 压缩文件解压
- **winresource** — Windows 版本资源嵌入
- **Microsoft Trusted Signing** — 免费代码签名
