#!/bin/bash

# HEIC Converter 测试覆盖率报告生成脚本
# 使用方法: ./generate-coverage.sh

set -e

echo "=========================================="
echo "HEIC Converter 测试覆盖率报告生成"
echo "=========================================="
echo ""

# 检查依赖
check_dependencies() {
    echo "[1/4] 检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        echo "❌ 未找到 Rust (cargo)"
        exit 1
    fi
    
    if ! command -v pnpm &> /dev/null; then
        echo "❌ 未找到 pnpm"
        exit 1
    fi
    
    echo "✅ 依赖检查通过"
    echo ""
}

# Rust 测试覆盖率
run_rust_coverage() {
    echo "[2/4] 生成 Rust 测试覆盖率..."
    echo ""
    
    # 运行测试
    echo "运行 Rust 测试..."
    cd src-tauri
    cargo test --quiet 2>&1 | grep -E "(test result|running)" || true
    cd ..
    
    # 如果没有 cargo-tarpaulin，提供替代方案
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo "⚠️  cargo-tarpaulin 未安装，使用基本测试报告"
        echo ""
        echo "安装 cargo-tarpaulin 以生成详细覆盖率报告:"
        echo "  cargo install cargo-tarpaulin"
        echo ""
        
        # 生成基本测试统计
        echo "Rust 测试统计:"
        cd src-tauri
        cargo test --quiet 2>&1 | tail -5 || true
        cd ..
    else
        # 使用 tarpaulin 生成详细报告
        echo "生成 HTML 覆盖率报告..."
        cd src-tauri
        cargo tarpaulin --out Html --output-dir ../target/coverage 2>&1 | tail -10 || true
        cd ..
        
        echo "✅ Rust 覆盖率报告已生成: target/coverage/tarpaulin-report.html"
    fi
    
    echo ""
}

# 前端测试覆盖率
run_frontend_coverage() {
    echo "[3/4] 生成前端测试覆盖率..."
    echo ""
    
    # 检查 node_modules
    if [ ! -d "node_modules" ]; then
        echo "⚠️  node_modules 不存在，正在安装依赖..."
        pnpm install
    fi
    
    # 运行测试并生成覆盖率
    echo "运行前端测试并生成覆盖率..."
    pnpm test:coverage 2>&1 | tail -20 || echo "测试完成"
    
    echo ""
    echo "✅ 前端覆盖率报告已生成: coverage/index.html"
    echo ""
}

# 生成汇总报告
generate_summary() {
    echo "[4/4] 生成汇总报告..."
    echo ""
    
    mkdir -p target/coverage
    
    cat > target/coverage/summary.md << EOF
# HEIC Converter 测试覆盖率报告

生成时间: $(date '+%Y-%m-%d %H:%M:%S')

## 测试统计

### Rust 后端
\`\`\`
$(cd src-tauri && cargo test --quiet 2>&1 | grep "test result" || echo "运行 cargo test 查看结果")
\`\`\`

### 前端
\`\`\`
$(pnpm test:coverage 2>&1 | grep "Test Files" || echo "运行 pnpm test:coverage 查看结果")
\`\`\`

## 覆盖率报告位置

- Rust 后端: \`target/coverage/tarpaulin-report.html\` (需要安装 cargo-tarpaulin)
- 前端: \`coverage/index.html\`

## 生成详细报告

### Rust 覆盖率 (需要 cargo-tarpaulin)
\`\`\`
cargo install cargo-tarpaulin
cd src-tauri
cargo tarpaulin --out Html --output-dir ../target/coverage
\`\`\`

### 前端覆盖率
\`\`\`
pnpm test:coverage
\`\`\`

## 查看报告

- Rust: 打开 \`target/coverage/tarpaulin-report.html\`
- 前端: 打开 \`coverage/index.html\`
EOF
    
    echo "✅ 汇总报告已生成: target/coverage/summary.md"
    echo ""
}

# 主函数
main() {
    check_dependencies
    run_rust_coverage
    run_frontend_coverage
    generate_summary
    
    echo "=========================================="
    echo "✅ 覆盖率报告生成完成"
    echo "=========================================="
    echo ""
    echo "查看汇总报告: cat target/coverage/summary.md"
    echo ""
}

main
