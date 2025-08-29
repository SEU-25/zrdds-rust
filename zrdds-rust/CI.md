# 持续集成 (CI) 配置

本项目使用 GitHub Actions 进行持续集成，确保代码质量和构建稳定性。

## CI 工作流程

### 构建作业 (build)

在每次推送到 `main` 或 `master` 分支，以及每个 Pull Request 时自动运行：

1. **代码格式检查** - 使用 `cargo fmt` 检查代码格式
2. **静态分析** - 使用 `cargo clippy` 进行代码质量检查
3. **项目构建** - 使用 `cargo build` 构建整个项目
4. **运行测试** - 使用 `cargo test` 运行所有测试
5. **示例编译** - 确保 `basic_usage` 示例能够正确编译

### 发布检查作业 (publish-check)

验证包是否准备好发布到 crates.io：

1. **包打包** - 使用 `cargo package` 验证包结构
2. **包内容验证** - 列出包中包含的所有文件

## 本地运行 CI 检查

在提交代码前，可以本地运行以下命令进行检查：

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
```

## 注意事项

- CI 在 Windows 环境下运行，与项目的目标平台保持一致
- 使用 Cargo 缓存来加速构建过程
- 所有警告都被视为错误，确保代码质量
- 示例代码必须能够成功编译，确保用户体验