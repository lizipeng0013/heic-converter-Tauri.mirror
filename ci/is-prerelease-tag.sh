#!/bin/sh
# ============================================================
# 预发布 tag 判定（POSIX sh，全项目复用）
# 判据：tag 后缀命中 SemVer 预发布关键字（大小写不敏感）：
#       -rc / -beta / -alpha / -pre / -preview / -test / -dev /
#       -snapshot / -nightly
# 注意：故意不用任意 "-后缀" 通配，避免误伤 Debian 修订号形式
#       tag（如 v0.7.0-1 中 "-1" 只是 Debian revision，仍是正式版）。
#
# 用法：sh ci/is-prerelease-tag.sh [tag]
#   tag 未传时依次取环境变量 TAG、CNB_BRANCH（tag_push 事件下即 tag 名）。
# 退出码：0 = 是预发布；1 = 不是预发布。
# 使用方：
#   - ci/release-atomgit.sh  决定 atomgit release_status（pre/latest）
#   - .cnb.yml git:release   决定 CNB Release preRelease/latest
# ============================================================
set -e
TAG="${1:-${TAG:-${CNB_BRANCH:?TAG/CNB_BRANCH 未设置}}}"

lower_tag=$(printf '%s' "$TAG" | tr '[:upper:]' '[:lower:]')
case "$lower_tag" in
  *-rc|*-rc[0-9.]*|*-beta|*-beta[0-9.]*|*-alpha|*-alpha[0-9.]*|*-pre|*-pre[0-9.]*|*-preview*|*-dev|*-dev[0-9.]*|*-snapshot*|*-test*|*-nightly*)
    exit 0 ;;
  *)
    exit 1 ;;
esac
