# syntax=docker/dockerfile:1
#
# deepin 25 构建环境镜像：把系统依赖（debian/control）+ Rust/Node/pnpm/cargo-deb
# 全部固化进镜像层，流水线里直接跑构建，不再现装工具链。
#
# 架构由 buildArgs.BASE_IMG 切换（amd64 用 linuxdeepin/deepin，
# arm64 用 linuxdeepin/deepin:arm64），本文件本身不含架构相关逻辑。
#
# 缓存策略见 ../.cnb.yml 的 docker.build.versionBy：本文件与 debian/control
# 任一变化即重建镜像，否则复用上一版缓存层。

ARG BASE_IMG=linuxdeepin/deepin
FROM ${BASE_IMG}

ARG NODE_VERSION=22.23.3
ARG PNPM_VERSION=10.15.0
ARG CARGO_DEB_VERSION=3.8.0

# CNB 不为 rustup / crates.io / npm 提供透明代理（见 docs/CI_CD.md），
# 安装期显式走国内镜像；这些变量随 ENV 保留，流水线内的 cargo/npm 也复用。
ENV RUSTUP_DIST_SERVER=https://rsproxy.cn \
    RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    NPM_CONFIG_REGISTRY=https://registry.npmmirror.com \
    PNPM_HOME=/opt/pnpm \
    PATH=/opt/node/bin:/opt/pnpm:/root/.cargo/bin:$PATH

# 系统依赖统一由 debian/control 声明，改依赖只改这一个文件。
# tools 段是安装工具链自身所需（curl/xz 等），不属于 debian/control。
COPY debian/ /build/debian/
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
      ca-certificates curl xz-utils; \
    apt-get build-dep -y /build; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/* /build; \
    rm -rf /var/cache/cargo /root/.cargo/registry

# rustup + 工具链：版本来自仓库根 rust-toolchain.toml，此处直接读它，
# 避免 Dockerfile 里再抄一份版本号导致两处漂移。
COPY rust-toolchain.toml /build/rust-toolchain.toml
RUN set -eux; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --no-modify-path --profile minimal; \
    cd /build && rustup show; \
    cargo install cargo-deb --version "${CARGO_DEB_VERSION}" --locked; \
    rm -rf /build

# Node 22（deepin 源里只有 20.15，低于 @vitejs/plugin-vue@6 要求的 >=22.12）
# + pnpm 10；从 npmmirror 的 node 镜像取 tarball，无需额外安装 nvm/fnm。
RUN set -eux; \
    case "$(dpkg --print-architecture)" in \
      amd64) node_arch=x64 ;; \
      arm64) node_arch=arm64 ;; \
      *) echo "unsupported arch: $(dpkg --print-architecture)" >&2; exit 1 ;; \
    esac; \
    curl -fsSL -o /tmp/node.tar.xz \
      "https://cdn.npmmirror.com/binaries/node/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-${node_arch}.tar.xz"; \
    mkdir -p /opt/node; \
    tar -xJf /tmp/node.tar.xz -C /opt/node --strip-components=1; \
    rm -f /tmp/node.tar.xz; \
    npm install -g "pnpm@${PNPM_VERSION}"; \
    node --version; \
    pnpm --version

WORKDIR /workspace
