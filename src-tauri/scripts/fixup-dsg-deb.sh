#!/usr/bin/env bash
# DSG 包后处理：把 cargo-deb 生成的 /usr/share/doc/{appid}/ 下的 copyright 与
# changelog.gz 移到 DSG 规范位置 /opt/apps/{appid}/entries/doc/{appid}/，
# 并整体重打包（DSG 包不允许写系统目录）。
#
# 两者都必须搬：DSG 规范要求应用文件全在 /opt/apps/{appid}/ 内，包内不得残留 /usr。
set -euo pipefail

APPID="top.hotime.heic-converter"
DEB_DIR="$(cd "$(dirname "$0")/../target/debian" && pwd)"
DEB="$(ls "$DEB_DIR/${APPID}"_*.deb 2>/dev/null | head -1)"

if [ -z "$DEB" ]; then
  echo "错误：$DEB_DIR 下找不到 $APPID 的 deb" >&2
  exit 1
fi

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/root/DEBIAN"

dpkg-deb -x "$DEB" "$WORK/root"
dpkg-deb -e "$DEB" "$WORK/root/DEBIAN"

SRC_DIR="$WORK/root/usr/share/doc/$APPID"
DST_DIR="$WORK/root/opt/apps/$APPID/entries/doc/$APPID"
moved=""
for f in copyright changelog.gz; do
  if [ -f "$SRC_DIR/$f" ]; then
    mkdir -p "$DST_DIR"
    mv "$SRC_DIR/$f" "$DST_DIR/$f"
    moved="$moved $f"
  else
    echo "警告：$SRC_DIR/$f 不存在，跳过" >&2
  fi
done

# 移走后清理空的 usr/share/doc 目录树，直到 usr 本身为空
find "$WORK/root/usr/share/doc" -depth -type d -empty -delete 2>/dev/null || true
rmdir "$WORK/root/usr/share" "$WORK/root/usr" 2>/dev/null || true

if [ -d "$WORK/root/usr" ]; then
  echo "错误：重排后包内仍有 /usr 残留，DSG 规范不允许" >&2
  find "$WORK/root/usr" -type f >&2
  exit 1
fi

OUT="$DEB"
dpkg-deb --root-owner-group --build "$WORK/root" "$OUT"
echo "已重打包：$OUT（已移到 entries/doc/$APPID/：$moved）"
