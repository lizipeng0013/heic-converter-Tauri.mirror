# syntax=docker/dockerfile:1
#
# deepin 25 构建环境镜像：把系统依赖（debian/control）+ Rust/pnpm/cargo-deb
# 固化进镜像层，流水线里直接跑构建，不再现装工具链。
# 镜像里没有 Node.js 也没有 npm：Node 由 pnpm 按 package.json 的
# devEngines.runtime 装成项目级依赖，见文件末尾说明。
#
# 架构由 buildArgs.BASE_IMG 切换（amd64 用 linuxdeepin/deepin，
# arm64 用 linuxdeepin/deepin:arm64），本文件本身不含架构相关逻辑。
#
# 缓存策略见 ../.cnb.yml 的 docker.build.versionBy：本文件、debian/control、
# rust-toolchain.toml 任一变化即重建镜像，否则复用上一版缓存层。

ARG BASE_IMG=linuxdeepin/deepin
FROM ${BASE_IMG}

# pnpm 的版本与 package.json 的 packageManager 字段是同一个值，改一处记得改另一处。
# 不从 package.json 读：它一动就废掉整张环境镜像缓存（改依赖也要重建 10 分钟）。
# 下面的安装脚本只认环境变量 PNPM_VERSION，不传就装 latest，等于没锁。
ARG PNPM_VERSION=12.8.1
ARG CARGO_DEB_VERSION=3.8.0

# CNB 不为 rustup / crates.io / npm 提供透明代理，安装期显式走国内镜像；
# 这些变量随 ENV 保留，流水线内的 cargo/pnpm 也复用。
ENV RUSTUP_DIST_SERVER=https://rsproxy.cn \
    RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    NPM_CONFIG_REGISTRY=https://registry.npmmirror.com \
    PNPM_HOME=/opt/pnpm \
    PATH=/opt/pnpm:/opt/pnpm/bin:/root/.cargo/bin:$PATH

# 系统依赖统一由 debian/control 声明，改依赖只改这一个文件。
# tools 段是安装工具链自身所需（curl 等），不属于 debian/control。
COPY debian/ /build/debian/
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
      ca-certificates curl; \
    apt-get build-dep -y --no-install-recommends /build; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/* /build

# Rust 版本不在本文件里抄，从仓库根的 rust-toolchain.toml 读，避免两处漂移。
# Node / pnpm 版本也不在这里声明：见 package.json 的 devEngines.runtime 与
# packageManager，由 pnpm 自己解析，不参与镜像缓存。
# 末尾清 cargo 下载缓存省镜像体积；/root/.cargo/bin 下的可执行文件必须保留。
# 注意：RUN 内部的注释行会被 shell 当注释吃掉，使该行脱离 set -e（甚至整段失效），
# 所以注释一律写在 RUN 之外。
COPY rust-toolchain.toml /build/
RUN set -eux; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --no-modify-path --profile minimal; \
    cd /build && rustup show; \
    cargo install cargo-deb --version "${CARGO_DEB_VERSION}" --locked; \
    rm -rf /build /root/.cargo/registry /root/.cargo/git /root/.cargo/.package-cache

# 安装 pnpm（独立脚本，不依赖 Node.js）
# 脚本内部会调 `pnpm setup` 改 shell 配置，必须给 SHELL，否则报 ERR_PNPM_UNKNOWN_SHELL。
# 不设 npm_config_registry：脚本把下载源硬编码成 registry.npmjs.org，ENV 里的镜像不生效。
RUN set -eux; \
    export SHELL=/bin/bash PNPM_VERSION="${PNPM_VERSION}"; \
    curl -fsSL https://get.pnpm.io/install.sh | sh -; \
    pnpm --version

# 本镜像不装 Node.js，也不装 npm。Node 由 package.json 的 devEngines.runtime
# 声明、`pnpm install` 时作为项目级依赖装进 node_modules/.bin，无需任何全局 node，
# 也就无需 node 镜像源。CI 里所有前端命令都经由 pnpm 执行。

WORKDIR /workspace
