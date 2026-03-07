/// Punctuation model test
///
/// Usage:
///   cargo run --bin compare_punctuation -- ./sherpa-onnx-punct-ct-transformer-zh-en-vocab272727-2024-04-12/model.onnx

use anyhow::Result;
use sherpa_rs::punctuate::{Punctuation, PunctuationConfig};

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = std::env::args().collect();
    let model_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "src-tauri/resources/models/sherpa-punct/sherpa-onnx-punct-ct-transformer-zh-en-vocab272727-2024-04-12/model.onnx".to_string());

    println!("Loading punctuation model: {}", model_path);

    let mut punct = Punctuation::new(PunctuationConfig {
        model: model_path,
        ..Default::default()
    })
    .map_err(|e| anyhow::anyhow!("{}", e))?;

    println!("Model loaded. Running tests...\n");

    let test_cases = vec![
        "今天天气真好我们去公园玩吧",
        "你好吗我很好谢谢",
        "为什么会这样呢我不太明白",
        "如果明天天气好我们就去爬山否则就在家看电影",
        "这是一个测试你好吗How are you我很好thank you are you ok谢谢你",
        "The African blogosphere is rapidly expanding bringing more voices online",
    ];

    for (i, text) in test_cases.iter().enumerate() {
        let start = std::time::Instant::now();
        let result = punct.add_punctuation(text);
        let elapsed = start.elapsed();

        println!("Test {}:", i + 1);
        println!("  Input:  {}", text);
        println!("  Output: {}", result);
        println!("  Time:   {:.2}ms", elapsed.as_secs_f64() * 1000.0);
        println!();
    }

    Ok(())
}
