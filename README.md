# 认知盗火者：Style-Decoder 🧠🔥

> **Stop prompting blind. Steal the cognitive structure.**
> 拒绝盲目调参，直接逆向提取顶尖创作者的认知骨架。

[![Rust](https://img.shields.io/badge/Rust-Blazing%20Fast-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## ⚠️ 痛点与真相

你是否厌倦了 AI 写出的文章总是充斥着“众所周知”、“不可否认”、“值得注意的是”这种令人作呕的塑料味？

顶级文章的魅力，从来不在于华丽的辞藻，而在于其底层的**认知架构**、**情绪基调**和**思维节奏**。
`style-decoder` 是一个由纯 Rust 编写的极简 CLI 工具。它能在 3 秒内，强行穿透网页的动态渲染限制，将任何一篇顶级爆款文章，逆向剥离成大模型可以直接理解的 **10 维度 System Prompt（风格复刻指令）**。

这是给 AI 时代超级个体的物理外挂。

## 🚀 核心火力

- **降维抓取**：底层接入 Jina Reader API，直接绕过复杂的前端反爬机制，提取纯净文本。
- **十维逆向解析**：自动分析目标文章的逻辑骨架、情绪基调、排比频率、隐喻习惯等 10 个核心维度。
- **一键转化为武器**：输出的结果不是分析报告，而是**直接可以喂给大模型的 System Prompt**。
- **极客级性能**：Rust 编译，单文件极速运行，零臃肿依赖。

## 📥 零门槛极速上手 (For Non-Coders)

无需懂代码，无需配置 Rust 环境，开箱即用：

1. **下载程序**：前往 [Releases 页面](https://github.com/YOUR_USERNAME/style-decoder/releases)，下载适合你系统（Windows `.exe` / macOS / Linux）的最新版本二进制文件。
2. **配置大模型钥匙**：在下载的程序同一目录下，新建一个名为 `.env` 的文本文件，填入你的 LLM API 凭证（以 OpenRouter 为例）：
   ```env
   API_KEY=sk-or-v1-你的真实API_KEY
   BASE_URL=[https://openrouter.ai/api/v1](https://openrouter.ai/api/v1)
   MODEL=anthropic/claude-3-5-sonnet-20241022
   ```
3. **执行逆向提取**：打开终端（命令行），运行程序并附上你想要逆向的爆款文章链接：
   ```bash
   ./style-decoder [https://mp.weixin.qq.com/s/某个爆款文章链接](https://mp.weixin.qq.com/s/某个爆款文章链接)
   ```

*（系统将自动在终端高亮输出极具深度的 Prompt，并询问你是否保存为 `.md` 文件。）*

---

## 🔮 进阶与闭环 (The Real Engine)

`style-decoder` 仅仅是我个人自动化工作流中的一个**单点探针模块**。

如果你觉得每次提取风格后再去手动复制、粘贴、喂给大模型依然太低效，或者你缺乏高质量的输入素材……

我目前构建了一套完整的 **AI 认知内容生成引擎**。它支持：
- 批量读取多个灵感素材（并发处理）。
- CoT（思维链）双通道渲染：先生成逻辑拓扑大纲，再进行高密度血肉填充。
- 自动化极速排版与剪贴板注入。

**🔗 链接与社群**

我不卖课。如果你对构建反脆弱的自动化系统、摆脱“线性劳动”、或者认知的指数级跃迁感兴趣：

- **阅读我的万字拆解文章**：👉 [《AI 时代架构能力远大于技术能力？大学生该如何破局？》](你的文章链接_发布后替换)
- **获取 50 份顶级写作 Prompt 预制菜**：我已经用这个脚本跑完了万维钢、李笑来等顶级高手的文章。关注我的微信公众号 **[你的公众号名称]**，回复 `Prompt` 直接获取。
- **探讨技术与认知**：在公众号后台留言，找到组织。

---
*Built with logic, rendered with AI.*