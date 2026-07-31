# Xa缓存加载工具

基于 Rust + egui 的 Windows GUI 工具，用于加载和解压 Xa 缓存文件到目标目录。

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
