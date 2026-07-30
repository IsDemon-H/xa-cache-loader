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

#### 2. 创建信任签名帐户

在 Azure Portal 顶部搜索栏输入 `信任签名` → 点击搜索结果中的 **"信任签名帐户"**：

| 中文界面看到的字段 | 填写 |
|-------------------|------|
| 订阅 | 选择你的订阅（默认那个就行） |
| 资源组 | 点击"新建"，命名 `xa-cache-loader` |
| 帐户名称 | `xa-cache-loader` |
| 定价层 | **Community**（免费层） |
| 区域 | `East US` |

点击"审阅并创建" → "创建"。

#### 3. 创建证书配置文件

部署完成后点"转到资源"，进入刚创建的信任签名帐户 → 左侧菜单 **"证书配置文件"** → 顶部 **"+ 添加"**：

| 中文界面看到的字段 | 填写 |
|-------------------|------|
| 配置文件名称 | `release` |
| 证书类型 | **Public-Trust**（公开信任） |
| 使用者 | 留空 |

点击"审阅并创建"。

#### 4. 配置 OIDC（让 GitHub Actions 免密码访问 Azure）

##### 4a. 注册应用

在 Azure Portal 顶部搜索 `Microsoft Entra ID`（中文界面搜索 "应用注册" 也行）→ 左侧 **"应用注册"** → 顶部 **"+ 新注册"**：

| 中文界面看到的字段 | 填写 |
|-------------------|------|
| 名称 | `xa-cache-loader-ci` |
| 支持的帐户类型 | 仅此组织目录中的帐户 |

点击"注册"。

##### 4b. 创建联合凭据

进入刚注册的应用 → 左侧 **"证书和密码"** → 顶部标签 **"联合凭据"** → **"+ 添加凭据"**：

| 中文界面看到的字段 | 填写 |
|-------------------|------|
| 联合凭据方案 | `GitHub Actions`（在下拉列表里选） |
| 组织 | `IsDemon-H` |
| 仓库 | `xa-cache-loader` |
| 实体类型 | `分支` |
| GitHub 分支名称 | `main` |
| 名称 | `release-signing` |

点击"添加"。

##### 4c. 记下三个 ID

回到应用 **"概览"** 页面，记下以下值（马上要用）：

- **应用程序(客户端) ID** — 一串 UUID，形如 `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
- **目录(租户) ID** — 同样是一串 UUID

再去 Azure Portal 顶部搜索 `订阅` → 打开 **"订阅"** 页面 → 记下你的 **订阅 ID**。

> 📝 这 3 个 ID 记在记事本里，等下配 GitHub 要用。

#### 5. 授权信任签名访问

回到信任签名帐户页面 → 左侧 **"访问控制(IAM)"** → 顶部 **"+ 添加"** → **"添加角色分配"**：

| 中文界面看到的字段 | 填写 |
|-------------------|------|
| 角色 | 搜索 `Trusted Signing Certificate Profile Signer`，搜到后选中 |
| 成员 | 点击 "+ 选择成员"，搜索 `xa-cache-loader-ci`，选中 |

点击"审阅并分配"。

---

### 🔐 设置 GitHub Secrets 和 Variables

打开你的仓库 `https://github.com/IsDemon-H/xa-cache-loader` → **Settings** → **Secrets and variables** → **Actions**。

#### Secrets（3 个）

点击 **Secrets** 标签 → **New repository secret**：

| Secret 名 | 值 | 说明 |
|-----------|-----|------|
| `TRUSTED_SIGNING_ENDPOINT` | 见下方端点表 | 信任签名服务的端点 URL |
| `TRUSTED_SIGNING_ACCOUNT` | `xa-cache-loader` | 步骤2 创建的帐户名称 |
| `CERTIFICATE_PROFILE_NAME` | `release` | 步骤3 创建的证书配置文件名称 |

#### Variables（3 个）

切换到 **Variables** 标签 → **New repository variable**：

| 变量名 | 值 |
|--------|-----|
| `AZURE_CLIENT_ID` | 步骤4c 记下的 应用程序(客户端) ID |
| `AZURE_TENANT_ID` | 步骤4c 记下的 目录(租户) ID |
| `AZURE_SUBSCRIPTION_ID` | 步骤4c 记下的 订阅 ID |

**端点对照表**（根据步骤2 创建时选的区域）：

| 你选的区域 | `TRUSTED_SIGNING_ENDPOINT` 填这个 |
|-----------|-----------------------------------|
| East US | `https://eus.codesigning.azure.net` |
| West US | `https://wus.codesigning.azure.net` |
| West Europe | `https://weu.codesigning.azure.net` |
| 其他 | 在 Azure 信任签名帐户的"概述"页面可查到

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
