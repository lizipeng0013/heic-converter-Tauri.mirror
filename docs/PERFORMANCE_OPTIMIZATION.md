# 性能优化计划

> 本文档详细分析了 HEIC 转换器的性能瓶颈，并提出了优化方案。

## 目录

1. [当前架构分析](#当前架构分析)
2. [性能瓶颈识别](#性能瓶颈识别)
3. [优化方案](#优化方案)
4. [实施计划](#实施计划)
5. [预期效果](#预期效果)

---

## 当前架构分析

### 技术栈

| 组件 | 技术 | 用途 |
|------|------|------|
| **并行处理** | rayon | 多线程转换 |
| **JPEG 编码** | turbojpeg | 高性能 JPEG 编码 |
| **内存分配** | mimalloc | 优化内存管理 |
| **HEIC 解码** | libheif-rs | HEIC 格式解码 |

### 现有优化

| 优化项 | 实现方式 | 效果 |
|--------|----------|------|
| **JPEG 编码** | turbojpeg 替代 image crate | 提速 30-50% |
| **内存分配** | mimalloc 全局分配器 | 减少内存碎片 |
| **并行处理** | rayon 线程池 + 原子操作 | 高效并发 |
| **线程池** | 自定义大小（保留 25% 给 UI） | 避免 UI 卡顿 |
| **HEIC 解码** | libheif RGB 内置解码 | 避免 YUV 转换 |

---

## 性能瓶颈识别

### 🔴 瓶颈 1: 文件 I/O（影响：高）

**现状：**
- 同步读写，大文件会阻塞线程
- 每个文件独立 I/O 操作

**机会：**
- 异步 I/O + 预读取
- 批量 I/O 操作

---

### 🔴 瓶颈 2: 内存分配（影响：中）

**现状：**
- 每个转换都分配新缓冲区
- 大图像内存分配开销大

**机会：**
- 内存池 + 缓冲区复用
- 预分配策略

---

### 🔴 瓶颈 3: 线程池启动开销（影响：中）

**现状：**
- 每次批量转换都创建新线程池
- 线程创建和销毁开销

**机会：**
- 全局线程池 + 任务队列

---

### 🔴 瓶颈 4: YUV->RGB 转换（影响：低）

**现状：**
- 纯 Rust 实现，无 SIMD
- 对于某些 HEIC 文件需要 YUV->RGB 转换

**机会：**
- 使用 AVX2/SSE 硬件加速指令

---

### 🟡 瓶颈 5: 小文件批量处理（影响：中）

**现状：**
- 每个文件独立处理
- 线程调度开销占比高

**机会：**
- 批量解码 + 流水线处理

---

## 优化方案

### 阶段 1: 基准测试与性能分析（P0）

**目标：** 建立性能基线，量化优化效果

#### 1.1 完善基准测试

**需要添加的测试：**

```rust
// 真实 HEIC 文件解码测试
fn bench_real_heic_decoding(c: &mut Criterion);

// 批量转换测试
fn bench_batch_conversion(c: &mut Criterion);

// 多线程性能测试
fn bench_thread_scaling(c: &mut Criterion);

// 内存使用测试
fn bench_memory_usage(c: &mut Criterion);
```

#### 1.2 性能监控工具

**监控指标：**
- 转换耗时（平均/最大/最小）
- 内存使用峰值
- CPU 利用率
- 线程池利用率

---

### 阶段 2: 核心优化（P0）

#### 2.1 内存池优化（最高优先级）

**实现方案：**

```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

static BUFFER_POOL: Lazy<Mutex<BufferPool>> = Lazy::new(|| {
    Mutex::new(BufferPool::new(1024 * 1024))
});

struct BufferPool {
    buffers: Vec<Vec<u8>>,
    min_buffer_size: usize,
}

impl BufferPool {
    fn acquire(&mut self, size: usize) -> Vec<u8> {
        // 复用或分配新缓冲区
    }
    
    fn release(&mut self, buffer: Vec<u8>) {
        // 回收缓冲区
    }
}
```

**应用位置：**
- `heic.rs` - HEIC 解码后的 RGB 缓冲区
- `common.rs` - JPEG 编码缓冲区

**预期提升：** 10-20%

---

#### 2.2 全局线程池优化

**实现方案：**

```rust
use once_cell::sync::Lazy;
use rayon::ThreadPoolBuilder;

static CONVERSION_POOL: Lazy<rayon::ThreadPool> = Lazy::new(|| {
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    
    ThreadPoolBuilder::new()
        .num_threads((num_cpus * 3 / 4).max(2))
        .thread_name(|i| format!("converter-{}", i))
        .build()
        .expect("Failed to create conversion pool")
});
```

**修改位置：**
- `conversion.rs` - 使用全局线程池替代局部线程池

**预期提升：** 5-10%

---

#### 2.3 批量解码优化

**实现方案：**

```rust
fn batch_decode_heic(
    files: &[String],
    pool: &rayon::ThreadPool,
) -> Result<Vec<RgbImage>, ConversionError> {
    pool.install(|| {
        files.par_iter()
            .map(|path| decode_heic_image(path))
            .collect::<Result<Vec<_>, _>>()
    })
}
```

**应用位置：**
- `conversion.rs` - 在批量转换前先批量解码

**预期提升：** 15-25%

---

### 阶段 3: SIMD 优化（P1 - 仅 x86_64）

#### 3.1 AVX2 加速实现

**实现方案：**

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn yuv_to_rgb_avx2(
    y_ptr: *const u8,
    cb_ptr: *const u8,
    cr_ptr: *const u8,
    rgb_ptr: *mut u8,
    width: usize,
    height: usize,
) {
    // 使用 AVX2 指令集加速
    // 一次处理 8 个像素
}
```

#### 3.2 运行时检测

**实现方案：**

```rust
fn yuv_to_rgb_optimized(
    y: &[u8],
    cb: &[u8],
    cr: &[u8],
    width: usize,
    height: usize,
) -> RgbImage {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                return yuv_to_rgb_avx2(y, cb, cr, width, height);
            }
        }
    }
    
    // 回退到标量实现
    yuv_to_rgb_scalar(y, cb, cr, width, height)
}
```

**应用位置：**
- `heic.rs` - YUV->RGB 转换部分

**预期提升：** 30-50%

---

### 阶段 4: 硬件加速评估（P2）

#### 4.1 方案评估

| 平台 | 技术 | 复杂度 | 性能提升 |
|------|------|--------|----------|
| **Linux** | VAAPI | 高 | 50-100% |
| **macOS** | VideoToolbox | 中 | 50-100% |
| **Windows** | Media Foundation | 中 | 50-100% |

#### 4.2 推荐方案

**建议：** 先实现软件优化（P0 + P1），硬件加速作为可选功能单独评估。

---

## Tokio 异步运行时可行性分析

### 结论：❌ 不推荐引入

**原因：**

1. **Tauri 运行时冲突**
   - Tauri 内置 tokio 运行时
   - 创建新的 tokio 运行时可能导致冲突
   - 可能阻塞 UI

2. **依赖冲突**
   - 需要确保 tokio 版本与 Tauri 兼容
   - 可能引入版本冲突

3. **复杂度增加**
   - 异步代码更难维护
   - 收益有限

### 替代方案

**保持现状：**
- ✅ 使用 rayon 线程池（已优化）
- ✅ 使用原子操作进行并发控制
- ✅ 无需引入新依赖

---

## 实施计划

### 时间表

| 阶段 | 时间 | 关键任务 |
|------|------|----------|
| **阶段 1** | 2-3 天 | 完善基准测试 + 建立性能基线 |
| **阶段 2** | 3-5 天 | 内存池 + 全局线程池 + 批量解码 |
| **阶段 3** | 5-7 天 | SIMD 优化（仅 x86_64） |
| **阶段 4** | 3-5 天 | 硬件加速评估 |
| **总计** | **13-20 天** | - |

### 优先级

| 优先级 | 优化项 | 预期收益 |
|--------|--------|----------|
| **P0** | 内存池优化 | 10-20% |
| **P0** | 全局线程池 | 5-10% |
| **P0** | 批量解码 | 15-25% |
| **P1** | SIMD 优化 | 30-50% |
| **P2** | 硬件加速 | 50-100% |

---

## 预期效果

### 总体性能提升

通过 P0 + P1 阶段优化，预计可提升 **40-60%** 性能。

### 详细预期

| 优化项 | 预期提升 | 复杂度 | 优先级 |
|--------|----------|--------|--------|
| **内存池优化** | 10-20% | 中 | P0 |
| **全局线程池** | 5-10% | 低 | P0 |
| **批量解码** | 15-25% | 高 | P0 |
| **SIMD 优化** | 30-50% | 高 | P1 |
| **硬件加速** | 50-100% | 高 | P2 |

---

## 测试文件方案

### 方案 A: 生成合成图像（推荐）

**优势：**
- 无需外部依赖
- 可控的文件大小和数量
- 快速生成

**实现：**
```bash
cd src-tauri
cargo run --bin generate_test_files --release
```

### 方案 B: 下载公开测试图片

**实现：**
```bash
./download_test_files.sh
```

**注意：** 当前网络环境可能无法下载。

---

## 参考资源

- [criterion 基准测试](https://bheisler.github.io/criterion.rs/book/index.html)
- [rayon 并行处理](https://docs.rs/rayon/)
- [turbojpeg 高性能 JPEG](https://github.com/turbojpeg/turbojpeg)
- [AVX2 指令集](https://doc.rust-lang.org/core/arch/x86_64/)

---

## 更新日志

- **2026-08-22**: 创建文档，完成基准测试和 tokio 分析
