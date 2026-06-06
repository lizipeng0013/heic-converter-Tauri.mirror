#!/bin/bash
# 重建 deb 包为 Deepin 规范格式
# 基于 bundle 目录直接处理，保留 DEBIAN 控制文件

set -e

# ============================================
# 配置
# ============================================
APPID="tech.hotime.heic-converter"
APPNAME="heic-converter"
VERSION="0.4.0"
ARCH="amd64"

# 安装前缀（相对于根目录的路径）
INSTALL_PREFIX="/opt/apps/${APPID}"

# 获取脚本所在目录（src-tauri/scripts/）
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# ============================================
# 工作目录
# ============================================
WORK_DIR=$(mktemp -d)
echo "工作目录: $WORK_DIR"

# 目标目录（在工作目录中的完整路径）
TEMP_TARGET_DIR="${WORK_DIR}/deepin${INSTALL_PREFIX}"
ENTRIES_DIR="${TEMP_TARGET_DIR}/entries"
FILES_DIR="${TEMP_TARGET_DIR}/files"
echo "目标目录: $TEMP_TARGET_DIR"

# ============================================
# 1. 查找 bundle 目录
# ============================================
cd src-tauri/
TARGET_DIR=$(cargo metadata --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['target_directory'])")
BUNDLE_DIR="${TARGET_DIR}/release/bundle/deb/${APPNAME}_${VERSION}_${ARCH}"

if [ ! -d "$BUNDLE_DIR" ]; then
    echo "错误: 找不到 bundle 目录: $BUNDLE_DIR"
    echo "请先运行: pnpm tauri build --bundles deb"
    rm -rf "$WORK_DIR"
    exit 1
fi

echo "找到 bundle 目录: $BUNDLE_DIR"

# ============================================
# 2. 重组 bundle 目录结构
# ============================================
echo "重组 bundle 目录结构..."

# 创建工作目录
mkdir -p "$WORK_DIR/deepin"

# 复制 control/ 目录到 DEBIAN/
if [ -d "$BUNDLE_DIR/control" ]; then
    cp -r "$BUNDLE_DIR/control" "$WORK_DIR/deepin/DEBIAN"
    echo "  → 已复制 DEBIAN 控制文件"
else
    echo "错误: 未找到 control 目录"
    rm -rf "$WORK_DIR"
    exit 1
fi

# 复制 data/ 目录的内容到工作目录
if [ -d "$BUNDLE_DIR/data" ]; then
    cp -r "$BUNDLE_DIR/data/"* "$WORK_DIR/deepin/"
    echo "  → 已复制数据文件"
else
    echo "错误: 未找到 data 目录"
    rm -rf "$WORK_DIR"
    exit 1
fi

cd "$WORK_DIR/deepin"

# ============================================
# 3. 移动可执行文件
# ============================================
echo "移动可执行文件..."
if [ -f "usr/bin/${APPNAME}" ]; then
    mkdir -p "${TEMP_TARGET_DIR}/files"
    mv "usr/bin/${APPNAME}" "${TEMP_TARGET_DIR}/files/"
    echo "  → 可执行文件: ${INSTALL_PREFIX}/files/${APPNAME}"
elif [ -f "usr/bin/heic-converter" ]; then
    # 备用：如果旧名称存在
    mkdir -p "${TEMP_TARGET_DIR}/files"
    mv "usr/bin/heic-converter" "${TEMP_TARGET_DIR}/files/${APPNAME}"
    echo "  → 可执行文件: ${INSTALL_PREFIX}/files/${APPNAME} (从 heic-converter 重命名)"
else
    echo "  → 未找到可执行文件"
fi

# ============================================
# 4. 移动图标文件
# ============================================
echo "移动图标文件..."
if [ -d "usr/share/icons" ]; then
    find "usr/share/icons" -type f \( -name "*.png" -o -name "*.svg" \) | while read icon; do
        # 计算相对路径
        rel_path="${icon#usr/share/icons/}"
        target_icon_dir="${TEMP_TARGET_DIR}/entries/icons/$(dirname "$rel_path")"
        target_icon_dir=$(echo "$target_icon_dir" | sed 's/@2//')
        mkdir -p "$target_icon_dir"
        mv "$icon" "$target_icon_dir/${APPID}.png"
        echo "  → $(basename "$icon") -> ${INSTALL_PREFIX}/entries/icons/hicolor/$rel_path"
    done
else
    echo "  → 未找到图标文件"
fi

# ============================================
# 5. 移动 desktop 文件
# ============================================
echo "移动 desktop 文件..."
if [ -f "usr/share/applications/${APPNAME}.desktop" ]; then
    mkdir -p "${TEMP_TARGET_DIR}/entries/applications"
    mv "usr/share/applications/${APPNAME}.desktop" "${TEMP_TARGET_DIR}/entries/applications/"
    echo "  → desktop 文件: ${INSTALL_PREFIX}/entries/applications/${APPNAME}.desktop"
elif [ -f "usr/share/applications/heic-converter.desktop" ]; then
    # 备用：如果旧名称存在
    mkdir -p "${TEMP_TARGET_DIR}/entries/applications"
    mv "usr/share/applications/heic-converter.desktop" "${TEMP_TARGET_DIR}/entries/applications/${APPID}.desktop"
    echo "  → desktop 文件: ${INSTALL_PREFIX}/entries/applications/${APPID}.desktop (从 heic-converter.desktop 重命名)"
else
    echo "  → 未找到 desktop 文件"
fi

# ============================================
# 5.1. 替换 desktop 文件中的路径和名称
# ============================================
echo "更新 desktop 文件路径和名称..."
if [ -d "${TEMP_TARGET_DIR}/entries/applications" ]; then
    for desktop_file in "${TEMP_TARGET_DIR}/entries/applications/"*.desktop; do
        if [ -f "$desktop_file" ]; then
            # 替换 Exec 路径
            sed -i "s|^Exec=.*|Exec=${INSTALL_PREFIX}/files/${APPNAME}|" "$desktop_file"
            # 替换 Icon 名称
            sed -i "s|^Icon=.*|Icon=${APPID}|" "$desktop_file"
            # 替换 StartupWMClass
            sed -i "s|^StartupWMClass=.*|StartupWMClass=${APPID}|" "$desktop_file"
            echo "  → 已更新: $(basename "$desktop_file")"
        fi
    done
else
    echo "  → 未找到 desktop 文件目录"
fi

# ============================================
# 6. 添加自定义 Deepin 文件: info
# ============================================
echo "添加自定义 Deepin 文件：info..."
if [ -f "$PROJECT_ROOT/scripts/info" ]; then
    mkdir -p "${TEMP_TARGET_DIR}"
    cp -r "$PROJECT_ROOT/scripts/info" "${TEMP_TARGET_DIR}/"
    echo "  → 已添加 info"
else
    echo "  → 未找到自定义 Deepin 文件：info"
fi

# ============================================
# 7. 清理原始 usr 目录
# ============================================
echo "清理原始 usr 目录..."
rm -rf usr/

# ============================================
# 8. 更新 DEBIAN/md5sums（如果文件路径改变）
# ============================================
echo "更新文件校验和..."
if [ -f "DEBIAN/md5sums" ]; then
    # 重新生成 md5sums
    (cd . && find . -type f -print0 | xargs -0 md5sum | sort > DEBIAN/md5sums)
    echo "  → 已更新 DEBIAN/md5sums"
else
    echo "  → DEBIAN/md5sums 不存在，跳过"
fi

# ============================================
# 9. 重新打包
# ============================================
echo "重新打包为 Deepin 格式..."
fakeroot dpkg-deb -b . "${TARGET_DIR}/release/bundle/deb/${APPNAME}_${VERSION}_deepin_${ARCH}.deb"

# ============================================
# 10. 清理工作目录
# ============================================
echo "清理工作目录..."
cd "$PROJECT_ROOT"
rm -rf "$WORK_DIR"

# ============================================
# 完成
# ============================================
echo ""
echo "✅ 重建完成！"
echo "   输出: ${TARGET_DIR}/release/bundle/deb/${APPNAME}_${VERSION}_deepin_${ARCH}.deb"
echo ""
echo "验证命令："
echo "  dpkg-deb -c ${TARGET_DIR}/release/bundle/deb/${APPNAME}_${VERSION}_deepin_${ARCH}.deb | grep -A 20 'opt/apps'"
