# 贡献指南

我们欢迎对 cmark-writer 的贡献！本指南概述了向项目贡献的流程。

## 入门

1. **在 GitHub 上 Fork 存储库**
2. **将您的 Fork 克隆**到本地机器
3. **设置开发环境**:

   ```bash
   git clone https://github.com/YOUR-USERNAME/cmark-writer.git
   cd cmark-writer
   cargo build
   cargo test
   ```

## 开发工作流程

### 进行更改

1. 为您的功能或错误修复创建一个新分支：

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. 遵循编码标准进行更改
3. 在适用的情况下为您的更改添加测试
4. 根据需要更新文档

### 测试

在提交之前，确保所有测试都通过：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test -- test_name

# 运行启用了 gfm 功能的测试
cargo test --features gfm
```

### 文档

添加新功能时，请更新文档：

- 使用文档注释添加内联代码文档
- 如有必要，更新 README.md
- 考虑添加示例

## Pull Request 流程

1. **将您的更改推送**到 GitHub 上的 Fork
2. **从您的分支创建 Pull Request** 到主存储库
3. **在 PR 描述中描述您的更改**，包括：
   - PR 添加或修复了什么
   - 任何破坏性更改
   - 测试方法
4. 如果需要，**处理审核反馈**

## 编码标准

- 遵循 Rust 风格指南
- 使用有意义的变量和函数名
- 为公共 API 添加文档注释
- 让函数专注于单一职责
- 在提交前使用 `cargo fmt` 格式化代码

## 错误报告和功能请求

如果您发现错误或有功能请求，请在 GitHub 上开启问题：

- 对于错误，包括重现步骤、预期行为和实际行为
- 对于功能请求，描述该功能以及为何它有价值

## 许可证

通过向 cmark-writer 贡献，您同意您的贡献将根据项目的 MIT 许可证授权。
