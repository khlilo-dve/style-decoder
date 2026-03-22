use dialoguer::{Input, theme::ColorfulTheme};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process;

// ── OpenAI-compatible request/response types ────────────────────────

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

// ── LLM API call (OpenAI-compatible) ────────────────────────────────

async fn call_llm(
    client: &Client,
    base_url: &str,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_content: &str,
) -> Result<String, String> {
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));

    let body = ChatRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_content.to_string(),
            },
        ],
    };

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let status = resp.status();
    let raw_body = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!("API error (HTTP {}): {}", status, raw_body));
    }

    let chat_resp: ChatResponse = serde_json::from_str(&raw_body).map_err(|e| {
        format!(
            "Failed to parse API response: {}\nRaw: {}",
            e,
            &raw_body[..raw_body.len().min(500)]
        )
    })?;

    chat_resp
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| {
            format!(
                "API returned empty choices\nRaw: {}",
                &raw_body[..raw_body.len().min(500)]
            )
        })
}

// ── Jina Reader + fallback strip-tags ────────────────────────────────

fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    let mut prev_blank = false;
    result
        .lines()
        .filter(|line| {
            let blank = line.trim().is_empty();
            if blank && prev_blank {
                return false;
            }
            prev_blank = blank;
            true
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

async fn fetch_readable_text(client: &Client, url: &str) -> Result<String, String> {
    let jina_url = format!("https://r.jina.ai/{}", url);
    println!("[info] Fetching via Jina Reader ...");

    let jina_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let resp = jina_client
        .get(&jina_url)
        .header("Accept", "text/markdown")
        .header("User-Agent", "style-decoder/0.1")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            let text = r
                .text()
                .await
                .map_err(|e| format!("Failed to read Jina response: {}", e))?;
            if text.trim().is_empty() {
                return Err("Jina Reader returned empty content".to_string());
            }
            println!("[info] Jina Reader OK, {} chars", text.len());
            Ok(text)
        }
        Ok(r) => {
            eprintln!(
                "[warn] Jina Reader returned HTTP {}, falling back to direct fetch",
                r.status()
            );
            fetch_fallback_plain(client, url).await
        }
        Err(e) => {
            eprintln!("[warn] Jina Reader failed: {}, falling back to direct fetch", e);
            fetch_fallback_plain(client, url).await
        }
    }
}

async fn fetch_fallback_plain(client: &Client, url: &str) -> Result<String, String> {
    let resp = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (compatible; style-decoder/0.1)")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let html_body = resp
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let text = strip_html_tags(&html_body);
    if text.is_empty() {
        return Err("Extracted content is empty".to_string());
    }
    println!("[info] Fallback fetch OK (strip-tags), {} chars", text.len());
    Ok(text)
}

// ── Style analysis prompt (10 dimensions) ────────────────────────────

const STYLE_ANALYSIS_PROMPT: &str = r#"你是一个写作风格分析专家。用户会给你一篇完整的文章正文，你需要逆向分析该文章的写作风格，并输出一份可以直接作为 system prompt 使用的「风格指令文档」。

输出要求：
1. 用 Markdown 格式
2. 涵盖以下维度（如果文章中体现了的话）：
   - 整体风格定位（如"理性 + 隐喻"、"口语化 + 犀利"等）
   - 标题策略（标题长度、是否用问句/反问/数字等）
   - 开头模式（故事切入、金句开头、直接观点等）
   - 段落节奏（长短交替、短段密集等）
   - 句式特征（长句/短句偏好、排比、设问等）
   - 论证手法（类比、举例、数据引用、反直觉等）
   - 情绪基调（冷静、激昂、反讽、温暖等）
   - 结尾策略（升华、行动号召、开放式提问等）
   - 用词偏好（口语/书面、中英混用、领域术语等）
   - 读者互动方式（如果有的话）
3. 每个维度给出具体的示例句子或段落片段作为佐证
4. 最后给出一段可直接作为 system prompt 的「风格复刻指令」

注意：不要评价文章质量，只做风格提取和描述。"#;

// ── Env helpers ─────────────────────────────────────────────────────

fn env_var(key: &str) -> Result<String, String> {
    std::env::var(key).map_err(|_| format!("Environment variable `{}` not set. Check your .env file.", key))
}

fn env_var_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

// ── Main ────────────────────────────────────────────────────────────

fn fatal(msg: &str) -> ! {
    eprintln!("[error] {}", msg);
    process::exit(1);
}

#[tokio::main]
async fn main() {
    // Load .env
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("[warn] .env not loaded: {} (using system env vars)", e);
    }

    // Parse URL from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: style-decoder <URL>");
        eprintln!();
        eprintln!("  Reverse-engineer any article's writing style into a reusable System Prompt.");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  style-decoder https://mp.weixin.qq.com/s/xxxxx");
        eprintln!();
        eprintln!("Environment variables (or .env file):");
        eprintln!("  API_KEY   - Your LLM API key (required)");
        eprintln!("  BASE_URL  - OpenAI-compatible API base URL (default: https://api.openai.com/v1)");
        eprintln!("  MODEL     - Model name (default: gpt-4o)");
        process::exit(1);
    }
    let url = &args[1];

    let api_key = env_var("API_KEY").unwrap_or_else(|e| fatal(&e));
    let base_url = env_var_or("BASE_URL", "https://api.openai.com/v1");
    let model = env_var_or("MODEL", "gpt-4o");

    println!();
    println!("  style-decoder v{}", env!("CARGO_PKG_VERSION"));
    println!("  ─────────────────────────────────");
    println!("  MODEL    = {}", model);
    println!("  TARGET   = {}", url);
    println!();

    // 1. Fetch article text via Jina Reader
    let client = Client::new();
    let article_text = fetch_readable_text(&client, url)
        .await
        .unwrap_or_else(|e| fatal(&e));

    // 2. Call LLM for 10-dimension style analysis
    println!("[info] Analyzing writing style via LLM ...");
    let style_analysis = call_llm(
        &client,
        &base_url,
        &api_key,
        &model,
        STYLE_ANALYSIS_PROMPT,
        &article_text,
    )
    .await
    .unwrap_or_else(|e| fatal(&format!("LLM call failed: {}", e)));

    // 3. Print result with visual separator
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              STYLE ANALYSIS RESULT                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("{}", style_analysis);
    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!();

    // 4. Ask user whether to save
    let save_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Save as .md file? Enter filename (without .md) or 'n' to skip")
        .default("n".to_string())
        .interact_text()
        .unwrap_or_else(|e| fatal(&format!("Input failed: {}", e)));

    let save_input = save_input.trim();
    if save_input == "n" || save_input == "N" || save_input.is_empty() {
        println!("[done] Output displayed. Nothing saved.");
    } else {
        let filename = format!("{}.md", save_input);
        fs::write(&filename, &style_analysis)
            .unwrap_or_else(|e| fatal(&format!("Failed to write `{}`: {}", filename, e)));
        println!("[done] Saved → {}", filename);
    }
}
