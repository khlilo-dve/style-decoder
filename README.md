# style-decoder

Reverse-engineer any article's writing style into a reusable **System Prompt**.

Feed it a URL → get a structured 10-dimension style analysis you can plug straight into ChatGPT / Claude / any LLM.

## Quick Start

```bash
# 1. Download the binary from GitHub Releases (no Rust needed)
#    → https://github.com/YOUR_USER/style-decoder/releases

# 2. Set up your API key
export API_KEY=sk-xxx
export BASE_URL=https://api.openai.com/v1   # or any OpenAI-compatible endpoint
export MODEL=gpt-4o

# 3. Run it
style-decoder https://example.com/some-great-article
```

## What It Does

1. **Fetches** the article via [Jina Reader](https://r.jina.ai) (clean text extraction, no headless browser needed)
2. **Analyzes** the writing style across 10 dimensions using your LLM
3. **Outputs** a ready-to-use System Prompt in your terminal
4. **Optionally saves** the result as a `.md` file

## 10 Analysis Dimensions

| # | Dimension |
|---|-----------|
| 1 | Overall style positioning |
| 2 | Title strategy |
| 3 | Opening pattern |
| 4 | Paragraph rhythm |
| 5 | Sentence structure |
| 6 | Argumentation techniques |
| 7 | Emotional tone |
| 8 | Ending strategy |
| 9 | Word choice preferences |
| 10 | Reader interaction style |

## Build from Source

```bash
cargo build --release
```

## Configuration

Create a `.env` file or set environment variables:

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `API_KEY` | Yes | — | Your LLM API key |
| `BASE_URL` | No | `https://api.openai.com/v1` | OpenAI-compatible API endpoint |
| `MODEL` | No | `gpt-4o` | Model name |

## License

MIT
