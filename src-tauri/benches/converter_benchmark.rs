//! HEIC Converter 性能基准测试
//!
//! 使用方法:
//! ```bash
//! cd src-tauri
//! cargo bench
//! ```
//!
//! 基准测试使用 criterion 库，需要先安装:
//! ```bash
//! cargo install cargo-criterion
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heic_converter_lib::converters::common::OutputFormat;
use image::{ImageBuffer, RgbImage};
use tempfile::TempDir;

/// 测试 JPEG 编码性能
fn bench_jpeg_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("jpeg_encoding");
    
    // 创建测试图像
    let buffer: RgbImage = ImageBuffer::new(1920, 1080);
    let temp_dir = TempDir::new().unwrap();
    
    // 测试不同质量的 JPEG 编码
    for quality in [60, 80, 90, 100] {
        group.bench_function(&format!("quality_{}", quality), |b| {
            let output_path = temp_dir.path().join(format!("test_{}.jpg", quality));
            b.iter(|| {
                let result = heic_converter_lib::converters::common::save_image_buffer(
                    black_box(&buffer),
                    black_box(output_path.to_str().unwrap()),
                    black_box(OutputFormat::Jpeg(quality)),
                );
                assert!(result.is_ok());
            });
        });
    }
    
    group.finish();
}

/// 测试不同图像尺寸的编码性能
fn bench_image_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_sizes");
    
    let sizes = [
        (640, 480, "640x480"),
        (1280, 720, "1280x720"),
        (1920, 1080, "1920x1080"),
        (3840, 2160, "3840x2160"),
    ];
    
    for (width, height, name) in sizes {
        group.bench_function(name, |b| {
            let buffer: RgbImage = ImageBuffer::new(width, height);
            let temp_dir = TempDir::new().unwrap();
            let output_path = temp_dir.path().join(format!("{}.jpg", name));
            
            b.iter(|| {
                let result = heic_converter_lib::converters::common::save_image_buffer(
                    black_box(&buffer),
                    black_box(output_path.to_str().unwrap()),
                    black_box(OutputFormat::Jpeg(90)),
                );
                assert!(result.is_ok());
            });
        });
    }
    
    group.finish();
}

/// 测试不同输出格式的性能
fn bench_output_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("output_formats");
    
    let buffer: RgbImage = ImageBuffer::new(1920, 1080);
    let temp_dir = TempDir::new().unwrap();
    
    let formats = vec![
        (OutputFormat::Jpeg(90), "jpg"),
        (OutputFormat::Png, "png"),
        (OutputFormat::WebP(80), "webp"),
        (OutputFormat::Bmp, "bmp"),
    ];
    
    for (format, name) in formats {
        group.bench_function(name, |b| {
            let output_path = temp_dir.path().join(format!("test.{}", name));
            b.iter(|| {
                let result = heic_converter_lib::converters::common::save_image_buffer(
                    black_box(&buffer),
                    black_box(output_path.to_str().unwrap()),
                    black_box(format.clone()),
                );
                assert!(result.is_ok());
            });
        });
    }
    
    group.finish();
}

/// 测试路径构建性能
fn bench_path_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_building");
    
    let test_paths = vec![
        "/path/to/image.heic",
        "/another/path/to/another_image.heic",
        "/very/long/path/with/many/segments/to/test/performance/image.heic",
    ];
    
    for path in test_paths {
        group.bench_function(path, |b| {
            b.iter(|| {
                let result = heic_converter_lib::utils::path::build_target_path(
                    black_box(path),
                    black_box("jpg"),
                    black_box("/output"),
                );
                assert!(result.contains("jpg"));
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_jpeg_encoding,
    bench_image_sizes,
    bench_output_formats,
    bench_path_building,
);
criterion_main!(benches);
