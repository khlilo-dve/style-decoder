# Style Decoder Prompt and Claude Code Skill Design

## Goal

将风格逆向提取规则从 Rust 源码中抽离为独立 Markdown 资产，Rust 在编译期嵌入该文档；同时交付一个用户级 Claude Code skill，支持本地文档和 URL 两种来源，并兼容显式命令与明确的自然语言请求。

## Architecture

- `prompts/style-analysis.md` 是仓库内唯一提示词源，保存 10 个分析维度、证据要求和最终 system prompt 输出要求。
- Rust 使用 `include_str!("../prompts/style-analysis.md")` 编译期嵌入文档，保持发布后的单文件二进制能力。
- `~/.claude/skills/style-decoder/SKILL.md` 是用户级入口，读取本地文件或通过 WebFetch 获取 URL，按同一套分析协议输出结果。

## Data Flow

### Rust CLI

URL → Jina Reader / direct fetch fallback → embedded prompt → OpenAI-compatible Chat Completions → terminal output → optional Markdown save.

### Claude Code skill

`/style-decoder <source>` 或用户明确要求逆向分析文件/链接 → 判断本地路径或 URL → 读取正文 → 依据 10 维协议分析 → 输出证据化风格报告和可复用 system prompt。

## Prompt Contract

输出必须使用 Markdown，覆盖以下维度（仅在原文体现时分析）：整体风格定位、标题策略、开头模式、段落节奏、句式特征、论证手法、情绪基调、结尾策略、用词偏好、读者互动方式。每个维度提供原文句子或段落片段作为证据，最后输出可直接作为 system prompt 的风格复刻指令。不得评价文章质量；证据不足时明确标注未观察到，不得臆测。

## Skill Contract

- 触发：显式 `/style-decoder <source>`，或用户明确要求分析某个文件/URL 的写作风格。
- 本地来源：使用 Claude Code 文件读取能力；支持纯文本、Markdown、HTML 可读文本，PDF 需先读取可提取文本，失败时说明原因。
- URL 来源：使用 WebFetch；不可访问或正文为空时报告具体错误，不伪造内容。
- 默认使用输入原文语言输出；说明文字默认中文。
- 不重复整篇原文，不把密钥或敏感配置写入结果。

## Verification

- Markdown 提示词与原常量规则等价。
- Rust 源码无长提示词字面量并使用 `include_str!`。
- `cargo fmt --check` 和 `cargo check` 通过。
- 用户级 skill 文件存在，包含触发、两类输入、10 维协议、证据规则和错误处理。
- 不修改 `.env`、CI 配置或 git 远端。
