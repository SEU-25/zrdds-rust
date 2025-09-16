# 持续集成 (CI) 配置

本项目使用 GitHub Actions 进行持续集成，确保代码质量和构建稳定性。

## CI 工作流程

工作流触发条件：

- 推送到 `main` 或 `master` 分支
- 创建 Pull Request

### 构建作业 (build)

1. **代码格式检查** - 使用 `cargo fmt` 检查代码格式
2. **静态分析** - 使用 `cargo clippy` 进行代码质量检查
3. **项目构建** - 使用 `cargo build` 构建整个项目
4. **运行测试** - 使用 `cargo test` 运行所有测试
5. **示例编译** - 确保 `basic_usage` 示例能够正确编译

### 发布检查作业 (publish-check)

1. **包打包** - 使用 `cargo package` 验证包结构
2. **包内容验证** - 列出包中包含的所有文件

### 文档作业 (doc)

1. **构建文档** - 使用 `cargo doc` 构建文档

2. **检查文档链接**
   - 使用 `cargo-deadlinks` 检查文档中的链接

3. **部署文档**
   - 部署到 Vercel 平台
   - 使用 vercel.json 配置

## 缓存策略

项目使用多层缓存提升构建速度：

- Cargo registry 缓存
- target 目录缓存
- Cargo 工具缓存

## 本地运行 CI 检查

在提交代码前，建议本地运行以下命令进行检查：

```bash
# 格式检查
cargo fmt --all -- --check

# 静态分析
cargo clippy --all-targets --all-features -- -D warnings

# 构建项目
cargo build --verbose

# 运行测试
cargo test --verbose

# 构建示例
cargo build --example basic_usage --verbose

# 检查包发布准备
cargo package --list

# 构建文档
cargo doc --no-deps --verbose
```

## 注意事项

- 所有作业在 Windows 环境下运行
- Clippy 警告允许继续执行
- 示例代码必须能够成功编译，以确保用户体验
- 文档使用 Vercel 托管，需要在仓库中设置 Secrets
