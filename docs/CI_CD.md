# CI/CD 工作流

CI 跑在 **CNB**（`.cnb.yml`），目标平台是 **deepin 25 / amd64 + arm64**，
产出 DSG 与标准 Debian 两套 `.deb`。

> 历史文档描述的 `.gitcode/workflows/*.yml`（AtomGit Action，Node 18）从未提交到仓库，
> 已随本次改造一并删除。

## 组成

| 文件 | 作用 |
|------|------|
| `.cnb.yml` | 流水线定义：触发条件、架构矩阵、产物上传 |
| `ci/deepin-build.Dockerfile` | 构建环境镜像：系统依赖 + Rust + Node + pnpm + cargo-deb |
| `debian/control` | 仅声明 `Build-Depends`，供 `apt-get build-dep .` 预装系统依赖 |
| `rust-toolchain.toml` | 固定 Rust 版本（仓库根，见下） |

## 触发分层

| 事件 | 任务 |
|------|------|
| PR | `pr-gate`：amd64 跑 clippy + vitest + cargo test |
| push `main` | 门禁（amd64）+ 门禁（arm64）+ amd64 打包冒烟 |
| tag push | amd64 / arm64 打包 → SHA256 → 上传 CNB Release 附件 |

打包用 `pnpm deb`（`tauri build` + `cargo deb --variant=dsg` + `fixup-dsg-deb.sh` + `--variant=debian`），
产物留在 `src-tauri/target/debian/`，流水线直接对该目录做 SHA256 并作为附件上传。
不另建 `dist/`：那个路径是 vite 的 `build.outDir`，混装会分不清前端产物和安装包。

## 环境镜像与缓存

CNB **不会**为 rustup / crates.io / npm 提供透明代理，Dockerfile 里显式配置了
`RUSTUP_DIST_SERVER=https://rsproxy.cn` 与 `NPM_CONFIG_REGISTRY=https://registry.npmmirror.com`，
这两个 `ENV` 会随镜像保留，流水线内的 cargo / npm 也复用。

工具链全部烤进镜像层，靠 `docker.build.versionBy` 判定是否重建：
**只有** `ci/deepin-build.Dockerfile`、`debian/control`、`rust-toolchain.toml` 任一变化才重建镜像，
改源码不会。版本号只在这三个文件里出现，Dockerfile 通过 `COPY rust-toolchain.toml` + `rustup show`
读取工具链版本，不重复抄写。

架构通过 `buildArgs.BASE_IMG` 切换：`linuxdeepin/deepin`（amd64）、`linuxdeepin/deepin:arm64`（arm64），
分别跑在 `cnb:arch:amd64` / `cnb:arch:arm64:v8` 原生节点上，不走 QEMU。

## 工具链版本

| 工具 | 版本 | 说明 |
|------|------|------|
| Rust | 1.98.1 | 见 `rust-toolchain.toml` |
| Node | 22.23.3 | deepin 源只有 20.15，低于 `@vitejs/plugin-vue@6` 要求的 `>=22.12` |
| pnpm | 10.15.0 | 随 Node 一起 `npm i -g` |
| cargo-deb | 3.8.0 | apt 源无此包，镜像内 `cargo install --locked` |

`rust-toolchain.toml` 放**仓库根**而不是 `src-tauri/`：rustup 按当前工作目录向上查找该文件，
而 `package.json` 的脚本都在仓库根执行 `cargo --manifest-path src-tauri/Cargo.toml`，
rustup 不会跟随 `--manifest-path`。

不用系统 apt 的 cargo/rustc：deepin 25 是 1.81，解析不了依赖树中
`rav1e → v_frame → av-scenechange → aligned 0.4.3` 使用的 edition2024；
依赖树最高 MSRV 是 1.89.0（`notify-rust 4.18.0`）。

## 依赖维护

系统依赖只写在 `debian/control` 一处，改完 Dockerfile 无需同步。当前刻意**未**列入的：

- `libayatana-appindicator3-dev`：托盘库由 `libappindicator-sys` 纯 dlopen 加载（无 build.rs），
  构建期不需要；运行时库 `libayatana-appindicator3-1` 写在 `Cargo.toml` 的 deb `depends`。
- `libturbojpeg0-dev`：`turbojpeg-sys` 启用 `cmake` + `require-simd`，从源码静态编出
  `libturbojpeg.a`，二进制不链接 `libturbojpeg.so`。
- `libssl-dev`：依赖树里没有 `openssl-sys` / `native-tls`。
- `libxdo-dev` / `librsvg2-dev`：`ldd` 与链接命令里都没有 Xdo / rsvg。
- `patchelf` / `fakeroot`：cargo-deb 用 `dpkg-deb` + `dpkg-shlibdeps` 打包，用不到。

## 可移植性

产物不设 `target-cpu=native`，保证通用 x86-64 也能跑。二进制里仍能搜到少量 AVX-512
（EVEX / `zmm`）指令，来自 `rav1e`（`image` → `ravif` → `rav1e`）自带的手写
`*.asm`，它通过 `is_x86_feature_detected!` 运行时派发，只在支持 AVX-512 的 CPU 上执行。
