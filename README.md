# Xa缓存加载工具

基于 Rust + egui 的 Windows GUI 工具，用于加载和解压 Xa 缓存文件到目标目录。

---

## 🔐 代码签名：现实情况

**免费代码签名方案的真实状态：**

| 方案 | 状态 | 原因 |
|------|------|------|
| Microsoft Trusted Signing | ❌ 不可用 | 仅限美国/加拿大组织，需3年以上历史 |
| SignPath Foundation | ❌ 不可用 | 严格审核，要求软件被广泛使用 |
| 购买证书（淘宝等） | ⚠️ 几百元/年 | Sectigo/Comodo OV 证书，最实际的办法 |
| 自签名 | ✅ 免费 | SmartScreen 仍会警告，但文件有签名 |

**结论：纯免费+SmartScreen不拦截 = 目前不存在适合你情况的方案。**

---

## 🎯 实际建议

### 方案 A：自签名（免费，本仓库已配置）

每个 release 会用自签名证书签名。文件属性里能看到"数字签名"标签，可以验证文件未被篡改。

代价：用户首次运行 SmartScreen 会提示"Windows 已保护你的电脑"，点"更多信息"→"仍要运行"即可。

### 方案 B：购买证书（推荐，如果你要分发）

淘宝搜"代码签名证书"，Sectigo/Comodo OV 证书约 ¥300-800/年。拿到 `.pfx` 文件后，在 GitHub Secrets 中配置即可，SmartScreen 不再拦截。

---

## GitHub Actions 自动构建 + 自签名

推送 `v*` 标签自动编译并签名发布：

```bash
git tag v1.0.0
git push origin v1.0.0
```

也可在 Actions 页面手动触发（`workflow_dispatch`）。

---

## 本地构建

```bash
cargo build --release
# 输出: target/release/xa-cache-loader.exe
```

---

## 技术栈

- **Rust** (edition 2024)
- **egui/eframe** 0.31 — GUI 框架
- **zip** + **sevenz-rust** — 压缩文件解压
- **winresource** — Windows 版本资源嵌入
