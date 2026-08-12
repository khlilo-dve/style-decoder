# 项目规则

## 目录结构

- `src/`：Rust CLI 程序实现。
- `prompts/`：程序消费的稳定 Markdown 提示词资产；文件名使用 kebab-case，每个文件只维护一份提示词契约。
- `docs/superpowers/specs/`：已批准的设计规格。
- `docs/superpowers/plans/`：实施计划。
- `.claude/`：项目级 Claude Code 配置和工作树元数据；用户级 skill 不放在这里。

## 开发约束

- 密钥只放在 `.env` 或环境变量中；不得提交密钥，也不得写入日志。
- 修改 Rust 后必须运行 `cargo fmt --check`、`cargo check` 和 `cargo test`。
- 除非用户明确要求，不修改 CI/CD 或部署配置。
