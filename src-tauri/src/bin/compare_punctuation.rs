/// 标点恢复功能测试
///
/// 测试基于规则的标点恢复实现
///
/// 使用方法:
///   cargo run --bin compare_punctuation

use anyhow::Result;
use ta_ting_lib::punctuation::RuleBasedPunctuationRestorer;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("========================================");
    println!("  标点恢复功能测试 (基于规则)");
    println!("========================================\n");

    // 测试用例
    let test_cases = vec![
        "今天天气真好我们去公园玩吧",
        "你好吗我很好谢谢",
        "你觉得怎么样我觉得不错",
        "明天要下雨吗我想去爬山",
        "哇这个地方真漂亮",
        "为什么会这样呢我不太明白",
        "如果明天天气好我们就去爬山否则就在家看电影",
        "虽然今天很累但是我还是很开心因为完成了很多工作",
    ];

    println!("📋 基于规则的标点恢复 (轻量级，零依赖)");
    println!("----------------------------------------");

    let rule_restorer = RuleBasedPunctuationRestorer::new();

    for (i, text) in test_cases.iter().enumerate() {
        println!("\n测试 {}:", i + 1);
        println!("输入: {}", text);

        let start = std::time::Instant::now();
        let result = rule_restorer.restore(text)?;
        let elapsed = start.elapsed();

        println!("输出: {}", result);
        println!("耗时: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
    }

    println!("\n🎉 测试完成!");

    // 性能测试
    println!("\n⚡ 性能测试...");
    let long_text = "今天天气真好我们去公园玩吧你觉得怎么样我觉得很不错我们可以带点零食和水一起去野餐";

    let start = std::time::Instant::now();
    let result = rule_restorer.restore(long_text)?;
    let elapsed = start.elapsed();

    println!("输入: {} ({} 字符)", long_text, long_text.chars().count());
    println!("输出: {}", result);
    println!("耗时: {:.2}ms", elapsed.as_secs_f64() * 1000.0);

    println!("\n========================================");
    println!("  总结");
    println!("========================================");

    println!("\n基于规则:");
    println!("  ✅ 优点: 启动快(<1ms)、内存小(<1MB)、零依赖");
    println!("  ⚠️  缺点: 准确度有限(~60-70%)");
    println!("  💡 适用: 快速原型、资源受限环境");

    println!("\n💡 提示:");
    println!("  - ONNX 模型版本待 ort 2.0 正式版发布后启用");
    println!("  - 当前版本足够日常使用，可通过添加规则改进");
    println!("  - 查看 docs/PUNCT_CURRENT_STATUS.md 了解详情");

    Ok(())
}
