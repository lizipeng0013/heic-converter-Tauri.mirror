#!/bin/sh
# ============================================================
# 通用 atomgit Release 同步脚本（POSIX sh，兼容 alpine ash）
# 目标：把本流水线产出的 .deb/.sha256 等制品同步到上游 atomgit(GitCode)
#       Release。一套脚本全项目复用——项目特有值全部经环境变量传入，
#       无任何硬编码，换项目无需改动本脚本。
#
# 输入环境变量（由 .cnb.yml 传入）：
#   ATOMGIT_TOKEN   必填。atomgit 个人访问令牌（经 CNB 密钥仓库 imports 注入）。
#                   未设置或为空时本脚本打印 skip 并以 0 退出（不影响 CNB Release 发布）。
#   ATOMGIT_OWNER   必填。上游仓库所属用户/组织。
#   ATOMGIT_REPO    必填。上游仓库名。
#   CNB_BRANCH      tag_push 事件下即 tag 名（CNB 内置变量），用作 Release 的 tag。
#   ARTIFACT_GLOB   可选。待上传制品 glob，默认覆盖 deb 与源码包及各自 sha256：
#                   "dist/*.deb dist/*.deb.sha256 dist/*.dsc dist/*.dsc.sha256
#                    dist/*.tar.xz dist/*.tar.xz.sha256"（源码包仅 amd64 发布线
#                   产出，glob 无匹配时该循环自动跳过）。
#   RELEASE_BODY_FILE 可选。release 描述文件路径（由 ci/gen-release-body.sh 生成），
#                   创建 atomgit Release 时作为正文；未提供或文件不存在时用兜底描述。
#   RELEASE_STATUS  可选。Release 状态："latest"（设置为最新版本）或 "pre"（预发布）。
#                   未设置时按 tag 名语义自动识别：命中 SemVer 预发布关键字
#                   （-rc/-beta/-alpha/-pre/-preview/-test/-dev/-snapshot/-nightly
#                   等，大小写不敏感）视为 pre，否则 latest。显式设置时优先生效。
#
# 行为：幂等确保该 tag 的 atomgit Release 存在（创建/已存在时均置 release_status=latest，
#       即勾选“设置为最新版本”）→ 逐个取 OBS 预签名 upload_url →
#       原样转发响应 headers（含关键 x-obs-callback）PUT 上传 → 校验 http 200/201，
#       失败即显式 exit 1（杜绝假成功假绿）。
# 用法：sh ci/release-atomgit.sh
# ============================================================
set -e

# ---- 0) 前置检查与参数归一 ----
if [ -z "${ATOMGIT_TOKEN:-}" ]; then
  echo "skip: ATOMGIT_TOKEN 未配置，跳过 atomgit Release 同步"
  exit 0
fi

OWNER="${ATOMGIT_OWNER:?ATOMGIT_OWNER 未设置}"
REPO="${ATOMGIT_REPO:?ATOMGIT_REPO 未设置}"
TAG="${CNB_BRANCH:?CNB_BRANCH 未设置}"   # tag_push 事件下即 tag 名
GLOB="${ARTIFACT_GLOB:-dist/*.deb dist/*.deb.sha256 dist/*.dsc dist/*.dsc.sha256 dist/*.tar.xz dist/*.tar.xz.sha256}"
BODY_FILE="${RELEASE_BODY_FILE:-.stage/RELEASE_BODY.md}"

# ---- 0.5) Release 状态：显式环境变量 > tag 名语义自动识别 ----
# 仅匹配预发布关键字，不把任意 "-后缀" 都当预发布，避免误伤 Debian 修订号
# 形式的 tag（如 v0.7.0-1 中 "-1" 只是 Debian revision，仍是正式版）。
if [ -n "${RELEASE_STATUS:-}" ]; then
  STATUS="$RELEASE_STATUS"
else
  lower_tag=$(printf '%s' "$TAG" | tr '[:upper:]' '[:lower:]')
  case "$lower_tag" in
    *-rc|*-rc[0-9.]*|*-beta|*-beta[0-9.]*|*-alpha|*-alpha[0-9.]*|*-pre|*-pre[0-9.]*|*-preview*|*-dev|*-dev[0-9.]*|*-snapshot*|*-test*|*-nightly*)
      STATUS=pre ;;
    *)
      STATUS=latest ;;
  esac
  echo "== auto detect release_status=${STATUS} from tag ${TAG} =="
fi

API="https://atomgit.com/api/v5/repos/${OWNER}/${REPO}"
AUTH="Authorization: ${ATOMGIT_TOKEN}"

# ---- 1) 幂等确保 atomgit Release 存在 ----
echo "== ensure atomgit release for tag ${TAG} (幂等) =="
http_code=$(curl -s -o /tmp/rel.json -w '%{http_code}' -H "$AUTH" "$API/releases/tags/$TAG")
if [ "$http_code" = "404" ]; then
  # 正文优先用 gen-release-body.sh 生成的 Markdown 文件，无则用兜底描述。
  if [ -f "$BODY_FILE" ]; then
    BODY=$(cat "$BODY_FILE")
  else
    BODY="${REPO} ${TAG} 构建产物（CNB 流水线自动发布）"
    echo "warn: ${BODY_FILE} 不存在，使用兜底描述"
  fi
  # 用 jq 构造 JSON 确保多行正文正确转义；release_status=latest 即“设置为最新版本”
  payload=$(jq -n --arg tag "$TAG" --arg name "$TAG" --arg body "$BODY" --arg status "$STATUS" \
    '{tag_name:$tag, name:$name, body:$body, release_status:$status}')
  curl -s -X POST -H "$AUTH" -H 'Content-Type: application/json' \
    -d "$payload" "$API/releases" >/dev/null
else
  # Release 已存在（如重跑流水线）：PATCH 幂等地将其置为最新版本，
  # 覆盖“先建的 Release 未勾选设置为最新版本”的历史数据。
  # name/body 为该接口必填字段，取 GET 结果中的原值回传，避免清空已有描述。
  echo "== PATCH release ${TAG}: release_status=${STATUS} =="
  exist_name=$(jq -r '.name // .tag_name // empty' /tmp/rel.json)
  exist_body=$(jq -r '.body // ""' /tmp/rel.json)
  patch_payload=$(jq -n --arg name "$exist_name" --arg body "$exist_body" --arg status "$STATUS" \
    '{name:$name, body:$body, release_status:$status}')
  http_code=$(curl -s -o /tmp/rel_patch.out -w '%{http_code}' -X PATCH \
    -H "$AUTH" -H 'Content-Type: application/json' \
    -d "$patch_payload" "$API/releases/$TAG")
  if [ "$http_code" != "200" ]; then
    echo "warn: PATCH release_status 失败 http=${http_code}（继续上传制品）"
    cat /tmp/rel_patch.out
  fi
fi

# ---- 2) 逐个上传制品 ----
echo "== upload assets to atomgit release =="
# shellcheck disable=SC2086  # GLOB 有意按单词拆分
for f in $GLOB; do
  [ -e "$f" ] || continue
  name=$(basename "$f")
  resp=$(curl -s -H "$AUTH" "$API/releases/$TAG/upload_url?file_name=$name")
  url=$(printf '%s' "$resp" | jq -r '.url // empty')
  if [ -z "$url" ]; then
    echo "ERROR: 未取得 ${name} 的上传地址"
    printf 'upload_url resp=%s\n' "$resp"
    exit 1
  fi
  echo "== PUT ${name} to OBS presigned url =="
  # 预签名 URL 自带 AccessKeyId/Signature 等查询串鉴权，不再带仓库鉴权头；
  # 但需原样转发 upload_url 响应 headers 中的 OBS 头（含关键的 x-obs-callback，
  # 缺失会导致文件虽上传到 OBS 却不挂接到 atomgit Release）。
  # 用 curl 配置文件(-K)承载 -H 参数，避免依赖 bash 数组/进程替换，兼容 ash/POSIX。
  printf '%s' "$resp" > /tmp/upload_url_resp.json
  jq -r '.headers | to_entries[] | "-H \"" + (.key) + ": " + (.value) + "\""' \
    /tmp/upload_url_resp.json > /tmp/obs_hdr_cfg
  http_code=$(curl -s -o /tmp/put.out -w '%{http_code}' -X PUT \
    -K /tmp/obs_hdr_cfg --data-binary "@$f" "$url")
  if [ "$http_code" != "200" ] && [ "$http_code" != "201" ]; then
    echo "ERROR: PUT ${name} 失败 http=${http_code}"
    cat /tmp/put.out
    exit 1
  fi
  echo "uploaded: ${name} (http=${http_code})"
done

echo "== atomgit sync done =="
