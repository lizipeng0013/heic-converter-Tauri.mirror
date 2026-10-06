#!/bin/sh
# ============================================================
# 通用 release 正文生成脚本（POSIX sh，兼容 alpine ash）
# 目标：从 debian/changelog 中提取当前发布版本的变更描述，格式化为
#       Markdown Release 正文（* 转 -），供 CNB Release 与 atomgit Release
#       描述使用。一套脚本全项目复用——项目特有值全部经环境变量传入。
#
# 输入环境变量：
#   CNB_BRANCH       必填。tag_push 事件下即 tag 名（如 v0.7.0）。
#   CHANGELOG_FILE   可选。changelog 路径，默认 "debian/changelog"。
#   OUTPUT_FILE      可选。输出文件路径，默认 ".stage/RELEASE_BODY.md"。
#   ARTIFACT_SUMMARY 可选。产物说明（如架构范围），未命中 changelog 时用作兜底描述。
#
# 行为：从 tag 提取版本号（去 v 前缀与 -suffix）→ 在 debian/changelog
#       中查找对应版本块 → 命中则提取（行首 * 转 -），否则输出兜底描述。
# 用法：sh ci/gen-release-body.sh
# ============================================================
set -e

# ---- 0) 参数归一 ----
TAG="${CNB_BRANCH:?CNB_BRANCH 未设置}"
CHANGELOG="${CHANGELOG_FILE:-debian/changelog}"
OUTPUT="${OUTPUT_FILE:-.stage/RELEASE_BODY.md}"

# 确保输出目录存在
OUTDIR=$(dirname "$OUTPUT")
mkdir -p "$OUTDIR" 2>/dev/null || true

# 从 tag 提取版本号：去 v/V 前缀、去 -suffix
VER=$(printf '%s' "$TAG" | sed -e 's/^[vV]//' -e 's/-.*$//')

echo "== tag=${TAG} -> version=${VER} =="

# ---- 1) 在 debian/changelog 中查找并提取版本块 ----
# changelog 每个版本块以 "pkgname (ver) dist; urgency=medium" 行开头、
# 以 " -- Maintainer <email>  date" 行结束。
# awk 状态机：匹配到目标版本行后进入块内，提取变更行——顶层 "  * " 转 "- "，
# 嵌套 "    - " 转 "  - "（markdown 嵌套列表），手动换行续行折进上一条
# bullet（4+ 空格缩进在 markdown 里会变代码块，必须折叠）；遇下一个版本行
# 或 maintainer 行即停止。
changes=$(awk -v ver="$VER" '
  function emit_pending() {
    if (pending != "") { print pending; pending = "" }
  }
  /^[^ ]+ \(/ && /; urgency=/ {
    # 提取括号内版本号
    if (match($0, /\([0-9][^)]*\)/)) {
      v = substr($0, RSTART+1, RLENGTH-2)
      base = v
      sub(/-.*$/, "", base)
      if (base == ver) { in_block=1; next }
      if (in_block) { emit_pending(); exit }
    }
    next
  }
  in_block && /^ -- / { emit_pending(); exit }   # maintainer 行 = 块结束
  in_block && /^  \* / {
    emit_pending()
    pending = "- " substr($0, 5)
    next
  }
  in_block && /^    - / {
    emit_pending()
    pending = "  - " substr($0, 7)
    next
  }
  in_block && /^  / {
    # 续行：折进上一条 bullet
    text = $0
    sub(/^ +/, "", text)
    sub(/ +$/, "", text)
    if (pending != "") pending = pending " " text
    next
  }
  in_block && /^$/ {
    emit_pending()   # 块内空行：冲刷挂起 bullet，不输出（保持紧凑列表）
    next
  }
  END { emit_pending() }
' "$CHANGELOG")

# ---- 2) 生成正文 ----
if [ -n "$changes" ]; then
  # 命中 changelog 版本块：版本标题 + 变更列表
  {
    printf '## 更新内容\n\n'
    printf '%s\n' "$changes"
  } > "$OUTPUT"
  echo "== changelog matched for version ${VER} =="
else
  # 未命中：兜底描述
  SUMMARY="${ARTIFACT_SUMMARY:-}"
  echo "warn: debian/changelog 中未找到版本 ${VER}，使用兜底描述"
  {
    printf '## %s\n\n' "$VER"
    [ -n "$SUMMARY" ] && printf '%s\n\n' "$SUMMARY"
    printf '本版本由 CNB 流水线自动构建发布。\n'
  } > "$OUTPUT"
fi

echo "== output: $OUTPUT =="
cat "$OUTPUT"
