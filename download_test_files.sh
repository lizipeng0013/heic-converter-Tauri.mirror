#!/bin/bash

# 下载公开 HEIC 测试图片
# 这些图片来自公开来源，无版权问题

mkdir -p test-data/heic-files

# Apple 官方 HEIC 样张（无版权）
echo "下载 Apple HEIC 样张..."
curl -L -o test-data/heic-files/apple_sample_1.heic \
  "https://github.com/nokiatech/heif/raw/master/content/images/autumn_1440x960.heic" 2>/dev/null || echo "下载失败: apple_sample_1"

curl -L -o test-data/heic-files/apple_sample_2.heic \
  "https://github.com/nokiatech/heif/raw/master/content/images/autumn_750x500.heic" 2>/dev/null || echo "下载失败: apple_sample_2"

# 其他公开 HEIC 测试图片
echo "下载其他 HEIC 测试图片..."
curl -L -o test-data/heic-files/test_1.heic \
  "https://github.com/strukturag/libheif/raw/master/examples/example.heic" 2>/dev/null || echo "下载失败: test_1"

echo "下载完成！"
ls -lh test-data/heic-files/
