//! 性能基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use serde_json::json;
use std::time::Duration;

// 模拟导入项目模块（在实际项目中需要调整路径）
// use zrdds_rust::utils::*;
// use zrdds_rust::dds_handlers::*;

// 模拟颜色解析函数（实际应该从项目中导入）
fn parse_color32_from_json_mock(value: &serde_json::Value) -> egui::Color32 {
    if let Some(array) = value.as_array() {
        if array.len() >= 4 {
            return egui::Color32::from_rgba_unmultiplied(
                array[0].as_u64().unwrap_or(255) as u8,
                array[1].as_u64().unwrap_or(255) as u8,
                array[2].as_u64().unwrap_or(255) as u8,
                array[3].as_u64().unwrap_or(255) as u8,
            );
        } else if array.len() >= 3 {
            return egui::Color32::from_rgb(
                array[0].as_u64().unwrap_or(255) as u8,
                array[1].as_u64().unwrap_or(255) as u8,
                array[2].as_u64().unwrap_or(255) as u8,
            );
        }
    }
    egui::Color32::WHITE
}

// 模拟线段与圆相交检测函数
fn line_intersects_circle_mock(x1: f32, y1: f32, x2: f32, y2: f32, cx: f32, cy: f32, radius: f32) -> bool {
    let dist1_sq = (x1 - cx) * (x1 - cx) + (y1 - cy) * (y1 - cy);
    let dist2_sq = (x2 - cx) * (x2 - cx) + (y2 - cy) * (y2 - cy);
    let radius_sq = radius * radius;
    
    if dist1_sq <= radius_sq || dist2_sq <= radius_sq {
        return true;
    }
    
    let line_length_sq = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
    if line_length_sq < 1.0 {
        let mid_x = (x1 + x2) / 2.0;
        let mid_y = (y1 + y2) / 2.0;
        let mid_dist_sq = (mid_x - cx) * (mid_x - cx) + (mid_y - cy) * (mid_y - cy);
        return mid_dist_sq <= radius_sq;
    }
    
    let dx = x2 - x1;
    let dy = y2 - y1;
    let fx = x1 - cx;
    let fy = y1 - cy;
    
    let a = dx * dx + dy * dy;
    let b = 2.0 * (fx * dx + fy * dy);
    let c = (fx * fx + fy * fy) - radius_sq;
    
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return false;
    }
    
    let discriminant = discriminant.sqrt();
    let t1 = (-b - discriminant) / (2.0 * a);
    let t2 = (-b + discriminant) / (2.0 * a);
    
    (t1 >= 0.0 && t1 <= 1.0) || (t2 >= 0.0 && t2 <= 1.0) || (t1 < 0.0 && t2 > 1.0)
}

// 基准测试：颜色解析性能
fn bench_color_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_parsing");
    
    // 测试不同的颜色格式
    let test_cases = vec![
        ("rgba", json!([255, 128, 64, 200])),
        ("rgb", json!([255, 128, 64])),
        ("invalid", json!("not_an_array")),
        ("empty", json!([])),
    ];
    
    for (name, color_json) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("parse_color", name),
            &color_json,
            |b, color_json| {
                b.iter(|| {
                    parse_color32_from_json_mock(black_box(color_json))
                });
            },
        );
    }
    
    group.finish();
}

// 基准测试：几何计算性能
fn bench_geometry_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("geometry");
    
    // 测试不同的几何场景
    let test_cases = vec![
        ("intersecting", (0.0, 0.0, 10.0, 10.0, 5.0, 5.0, 8.0)),
        ("non_intersecting", (0.0, 0.0, 1.0, 1.0, 10.0, 10.0, 2.0)),
        ("tangent", (0.0, 5.0, 10.0, 5.0, 5.0, 0.0, 5.0)),
        ("short_line", (5.0, 5.0, 5.1, 5.1, 5.0, 5.0, 2.0)),
    ];
    
    for (name, (x1, y1, x2, y2, cx, cy, radius)) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("line_circle_intersection", name),
            &(x1, y1, x2, y2, cx, cy, radius),
            |b, &(x1, y1, x2, y2, cx, cy, radius)| {
                b.iter(|| {
                    line_intersects_circle_mock(
                        black_box(x1), black_box(y1),
                        black_box(x2), black_box(y2),
                        black_box(cx), black_box(cy),
                        black_box(radius)
                    )
                });
            },
        );
    }
    
    group.finish();
}

// 基准测试：JSON解析性能
fn bench_json_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_parsing");
    
    // 测试不同大小的JSON数据
    let small_json = json!({
        "type": "draw",
        "color": [255, 0, 0, 255],
        "points": [[10, 20], [30, 40]]
    });
    
    let medium_json = json!({
        "type": "draw",
        "color": [255, 0, 0, 255],
        "points": (0..100).map(|i| [i, i * 2]).collect::<Vec<_>>(),
        "metadata": {
            "timestamp": 1234567890,
            "user_id": "user123",
            "session_id": "session456"
        }
    });
    
    let large_json = json!({
        "type": "draw",
        "color": [255, 0, 0, 255],
        "points": (0..1000).map(|i| [i, i * 2]).collect::<Vec<_>>(),
        "metadata": {
            "timestamp": 1234567890,
            "user_id": "user123",
            "session_id": "session456",
            "additional_data": (0..100).map(|i| format!("data_{}", i)).collect::<Vec<_>>()
        }
    });
    
    group.bench_function("small_json", |b| {
        b.iter(|| {
            let json_str = serde_json::to_string(black_box(&small_json)).unwrap();
            serde_json::from_str::<serde_json::Value>(black_box(&json_str)).unwrap()
        });
    });
    
    group.bench_function("medium_json", |b| {
        b.iter(|| {
            let json_str = serde_json::to_string(black_box(&medium_json)).unwrap();
            serde_json::from_str::<serde_json::Value>(black_box(&json_str)).unwrap()
        });
    });
    
    group.bench_function("large_json", |b| {
        b.iter(|| {
            let json_str = serde_json::to_string(black_box(&large_json)).unwrap();
            serde_json::from_str::<serde_json::Value>(black_box(&json_str)).unwrap()
        });
    });
    
    group.finish();
}

// 基准测试：批量操作性能
fn bench_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_operations");
    group.measurement_time(Duration::from_secs(10));
    
    // 测试批量颜色解析
    let colors: Vec<_> = (0..1000)
        .map(|i| json!([i % 256, (i * 2) % 256, (i * 3) % 256, 255]))
        .collect();
    
    group.bench_function("batch_color_parsing", |b| {
        b.iter(|| {
            for color in black_box(&colors) {
                parse_color32_from_json_mock(color);
            }
        });
    });
    
    // 测试批量几何计算
    let geometry_data: Vec<_> = (0..1000)
        .map(|i| {
            let f = i as f32;
            (f, f, f + 10.0, f + 10.0, f + 5.0, f + 5.0, 3.0)
        })
        .collect();
    
    group.bench_function("batch_geometry_calculations", |b| {
        b.iter(|| {
            for &(x1, y1, x2, y2, cx, cy, radius) in black_box(&geometry_data) {
                line_intersects_circle_mock(x1, y1, x2, y2, cx, cy, radius);
            }
        });
    });
    
    group.finish();
}

// 基准测试：内存分配性能
fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");
    
    group.bench_function("vec_allocation_small", |b| {
        b.iter(|| {
            let mut vec: Vec<i32> = Vec::new();
            for i in 0..100 {
                vec.push(black_box(i));
            }
            vec
        });
    });
    
    group.bench_function("vec_allocation_large", |b| {
        b.iter(|| {
            let mut vec: Vec<i32> = Vec::new();
            for i in 0..10000 {
                vec.push(black_box(i));
            }
            vec
        });
    });
    
    group.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            let mut vec: Vec<i32> = Vec::with_capacity(10000);
            for i in 0..10000 {
                vec.push(black_box(i));
            }
            vec
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_color_parsing,
    bench_geometry_calculations,
    bench_json_parsing,
    bench_batch_operations,
    bench_memory_operations
);
criterion_main!(benches);