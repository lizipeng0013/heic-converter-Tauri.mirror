#!/usr/bin/env bash
# DSG 包后处理：把 cargo-deb 硬编码生成的 /usr/share/doc/{appid}/copyright
# 移到 DSG 规范位置 /opt/apps/{appid}/entries/doc/{appid}/copyright，
# 并整体重打包（DSG 包不允许写系统目录）。
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

SRC="$WORK/root/usr/share/doc/$APPID/copyright"
DST_DIR="$WORK/root/opt/apps/$APPID/entries/doc/$APPID"
if [ -f "$SRC" ]; then
  mkdir -p "$DST_DIR"
  mv "$SRC" "$DST_DIR/copyright"
  # 移走后清理空的 usr/share/doc 目录树
  find "$WORK/root/usr/share/doc" -depth -type d -empty -delete 2>/dev/null || true
  rmdir "$WORK/root/usr/share" "$WORK/root/usr" 2>/dev/null || true
fi

OUT="$DEB"
dpkg-deb --root-owner-group --build "$WORK/root" "$OUT"
echo "已重打包：$OUT（copyright 位于 /opt/apps/$APPID/entries/doc/$APPID/copyright）"
