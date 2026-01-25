#!/usr/bin/env python3
"""
标点恢复模型下载和转换脚本

使用方法:
    python download_punctuation_model.py

将下载并转换模型到: src-tauri/resources/models/punctuation/
"""

import os
from pathlib import Path
from optimum.onnxruntime import ORTModelForTokenClassification
from transformers import AutoTokenizer

# 配置
MODEL_NAME = "oliverguhr/fullstop-punctuation-multilang-sonar-base"
OUTPUT_DIR = Path(__file__).parent / "src-tauri" / "resources" / "models" / "punctuation"

def download_and_convert():
    """下载并转换模型为 ONNX 格式"""
    print(f"正在下载模型: {MODEL_NAME}")
    print(f"输出目录: {OUTPUT_DIR}")

    # 创建输出目录
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    try:
        # 下载并转换模型
        print("\n步骤 1: 下载并转换为 ONNX...")
        model = ORTModelForTokenClassification.from_pretrained(
            MODEL_NAME,
            export=True
        )

        # 下载 tokenizer
        print("\n步骤 2: 下载 tokenizer...")
        tokenizer = AutoTokenizer.from_pretrained(MODEL_NAME)

        # 保存模型
        print(f"\n步骤 3: 保存到 {OUTPUT_DIR}...")
        model.save_pretrained(OUTPUT_DIR)
        tokenizer.save_pretrained(OUTPUT_DIR)

        # 检查文件
        print("\n生成的文件:")
        for file in OUTPUT_DIR.iterdir():
            print(f"  - {file.name} ({file.stat().st_size / 1024 / 1024:.2f} MB)")

        # 测试模型
        print("\n步骤 4: 测试模型...")
        test_text = "今天天气真好我们去公园玩吧"
        inputs = tokenizer(test_text, return_tensors="pt")
        outputs = model(**inputs)

        print("✅ 模型转换成功!")
        print(f"\n模型已保存到: {OUTPUT_DIR}")
        print("\n你现在可以在 Rust 中使用这个模型了!")

    except Exception as e:
        print(f"\n❌ 错误: {e}")
        print("\n请确保安装了以下依赖:")
        print("  pip install optimum[onnxruntime] transformers")
        return False

    return True

def create_config():
    """创建配置文件"""
    config = {
        "model_name": MODEL_NAME,
        "labels": ["O", "COMMA", "PERIOD", "QUESTION", "EXCLAMATION", "SEMICOLON", "COLON"],
        "max_length": 512,
    }

    import json
    config_path = OUTPUT_DIR / "config.json"

    # 如果已经存在 config.json，合并配置
    if config_path.exists():
        with open(config_path, "r", encoding="utf-8") as f:
            existing_config = json.load(f)
        existing_config.update(config)
        config = existing_config

    with open(config_path, "w", encoding="utf-8") as f:
        json.dump(config, f, indent=2, ensure_ascii=False)

    print(f"✅ 配置文件已创建: {config_path}")

if __name__ == "__main__":
    print("=" * 60)
    print("标点恢复模型下载和转换工具")
    print("=" * 60)

    if download_and_convert():
        create_config()

        print("\n" + "=" * 60)
        print("下一步:")
        print("=" * 60)
        print("1. 在 Rust 中使用:")
        print("   ```rust")
        print('   let processor = PunctuationProcessor::new();')
        print('   processor.initialize("resources/models/punctuation").await?;')
        print('   let result = processor.process("输入文本").await?;')
        print("   ```")
        print("\n2. 运行测试:")
        print("   cargo test --package ta-ting --lib punctuation::tests")
