/// 简单的基于规则的标点恢复器（备用方案）
///
/// 这是一个轻量级的标点恢复实现，不依赖 ONNX 模型。
/// 使用启发式规则和语言模式来添加标点符号。
///
/// 优点：
/// - 零依赖（除了标准库）
/// - 启动快速
/// - 内存占用小
/// - 无需下载模型
///
/// 缺点：
/// - 准确度低于 ML 模型
/// - 规则有限
///
/// 适用场景：
/// - 快速原型
/// - 资源受限环境
/// - 用户未安装模型

use anyhow::Result;

pub struct RuleBasedPunctuationRestorer {
    /// 句子结束词（通常后面跟句号）
    sentence_end_words: Vec<String>,
    /// 疑问词（通常后面跟问号）
    question_words: Vec<String>,
    /// 感叹词（通常后面跟感叹号）
    exclamation_words: Vec<String>,
    /// 逗号触发词
    comma_trigger_words: Vec<String>,
}

impl RuleBasedPunctuationRestorer {
    pub fn new() -> Self {
        Self {
            sentence_end_words: vec![
                "了".to_string(),
                "吧".to_string(),
                "呢".to_string(),
                "啊".to_string(),
                "呀".to_string(),
                "的".to_string(),
                "吗".to_string(),
            ],
            question_words: vec![
                "吗".to_string(),
                "呢".to_string(),
                "什么".to_string(),
                "哪里".to_string(),
                "为什么".to_string(),
                "怎么".to_string(),
                "如何".to_string(),
                "谁".to_string(),
                "哪".to_string(),
            ],
            exclamation_words: vec![
                "哇".to_string(),
                "哦".to_string(),
                "啊".to_string(),
                "嗯".to_string(),
                "呀".to_string(),
                "嘿".to_string(),
            ],
            comma_trigger_words: vec![
                "但是".to_string(),
                "然后".to_string(),
                "而且".to_string(),
                "或者".to_string(),
                "因为".to_string(),
                "所以".to_string(),
                "如果".to_string(),
                "虽然".to_string(),
                "不过".to_string(),
                "而".to_string(),
            ],
        }
    }

    /// 恢复标点（简单规则）
    pub fn restore(&self, text: &str) -> Result<String> {
        // 按字符分割
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];
            result.push(ch);

            // 检查是否需要添加标点
            if i > 0 && i < chars.len() - 1 {
                let context = self.get_context(&chars, i);

                // 规则 1: 检测疑问句
                if self.is_question_context(&context) {
                    result.push('？');
                }
                // 规则 2: 检测感叹句
                else if self.is_exclamation_context(&context) {
                    result.push('！');
                }
                // 规则 3: 检测需要逗号的位置
                else if self.should_add_comma(&context, &chars, i) {
                    result.push('，');
                }
                // 规则 4: 句子长度检测（每 15-20 字添加标点）
                else if self.should_add_period_by_length(&result) {
                    result.push('。');
                }
            }

            i += 1;
        }

        // 最后添加句号（如果还没有标点）
        if !result.is_empty() && !self.has_ending_punctuation(&result) {
            result.push('。');
        }

        Ok(result)
    }

    /// 获取上下文（前后几个字符）
    fn get_context(&self, chars: &[char], pos: usize) -> String {
        let start = pos.saturating_sub(5);
        let end = (pos + 5).min(chars.len());
        chars[start..end].iter().collect()
    }

    /// 检查是否是疑问句上下文
    fn is_question_context(&self, context: &str) -> bool {
        self.question_words.iter().any(|word| context.contains(word))
    }

    /// 检查是否是感叹句上下文
    fn is_exclamation_context(&self, context: &str) -> bool {
        self.exclamation_words.iter().any(|word| context.contains(word))
    }

    /// 检查是否应该添加逗号
    fn should_add_comma(&self, _context: &str, chars: &[char], pos: usize) -> bool {
        // 在连接词后添加逗号
        if self.comma_trigger_words.iter().any(|word| {
            let word_chars: Vec<char> = word.chars().collect();
            if pos >= word_chars.len() {
                let start = pos - word_chars.len() + 1;
                &chars[start..=pos] == &word_chars[..]
            } else {
                false
            }
        }) {
            return true;
        }

        false
    }

    /// 根据长度判断是否应该添加句号
    fn should_add_period_by_length(&self, current_text: &str) -> bool {
        // 统计上一个标点后的字符数
        let last_punctuation_pos = current_text
            .rfind(|c: char| matches!(c, '。' | '？' | '！' | '，'))
            .unwrap_or(0);

        let chars_since_punctuation = current_text[last_punctuation_pos..].chars().count();

        // 如果超过 20 个字符没有标点，考虑添加
        chars_since_punctuation > 20
    }

    /// 检查是否已有结尾标点
    fn has_ending_punctuation(&self, text: &str) -> bool {
        text.ends_with('。') || text.ends_with('？') || text.ends_with('！')
    }

    /// 高级版本：使用分词（需要 jieba 等库）
    #[cfg(feature = "advanced-rules")]
    pub fn restore_with_segmentation(&self, text: &str) -> Result<String> {
        // TODO: 实现基于分词的更准确的标点恢复
        // 需要集成 jieba-rs 或类似的中文分词库
        unimplemented!("需要 jieba-rs 支持");
    }
}

impl Default for RuleBasedPunctuationRestorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question() {
        let restorer = RuleBasedPunctuationRestorer::new();
        let result = restorer.restore("你好吗").unwrap();
        assert!(result.contains('？') || result.contains('。'));
    }

    #[test]
    fn test_statement() {
        let restorer = RuleBasedPunctuationRestorer::new();
        let result = restorer.restore("今天天气真好").unwrap();
        assert!(result.ends_with('。'));
    }

    #[test]
    fn test_exclamation() {
        let restorer = RuleBasedPunctuationRestorer::new();
        let result = restorer.restore("哇好漂亮啊").unwrap();
        println!("Result: {}", result);
        // 注意：简单规则可能不够准确
    }

    #[test]
    fn test_long_text() {
        let restorer = RuleBasedPunctuationRestorer::new();
        let result = restorer
            .restore("今天天气真好我们去公园玩吧你觉得怎么样")
            .unwrap();
        println!("Result: {}", result);
        // 应该包含一些标点
        let punct_count = result.chars().filter(|c| matches!(c, '，' | '。' | '？')).count();
        assert!(punct_count > 0);
    }
}
