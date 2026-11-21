//! Benchmark suite for impeccable-benchmark

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use impeccable_benchmark::{BenchmarkRunner, BenchmarkDetector, BenchmarkConfig, builtin};

fn bench_detector_startup(c: &mut Criterion) {
    let mut group = c.benchmark_group("detector_startup");
    
    for iterations in [10, 100, 1000] {
        group.bench_with_input(BenchmarkId::new("iterations", iterations), &iterations, |b, &iterations| {
            b.iter(|| {
                let config = BenchmarkConfig { iterations, ..Default::default() };
                let mut runner = BenchmarkRunner::with_config(config);
                builtin::register_all(&mut runner);
                
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let mut detector = BenchmarkDetector::new();
                    runner.run_all(&mut detector).await.unwrap();
                });
            });
        });
    }
    
    group.finish();
}

fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");
    
    for size in [1024, 10240, 102400] {
        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &size| {
            b.iter(|| {
                let _vec: Vec<u8> = vec![0; size];
            });
        });
    }
    
    group.finish();
}

fn bench_json_serialization(c: &mut Criterion) {
    let test_data = serde_json::json!({
        "name": "test",
        "values": (0..100).collect::<Vec<_>>(),
        "nested": {
            "a": 1,
            "b": [1, 2, 3, 4, 5],
            "c": "hello world"
        }
    });
    
    c.bench_function("json_serialization", |b| {
        b.iter(|| {
            let _ = serde_json::to_string(&test_data).unwrap();
        });
    });
}

fn bench_json_deserialization(c: &mut Criterion) {
    let test_json = serde_json::json!({
        "name": "test",
        "values": (0..100).collect::<Vec<_>>(),
        "nested": {
            "a": 1,
            "b": [1, 2, 3, 4, 5],
            "c": "hello world"
        }
    }).to_string();
    
    c.bench_function("json_deserialization", |b| {
        b.iter(|| {
            let _: serde_json::Value = serde_json::from_str(&test_json).unwrap();
        });
    });
}

fn bench_hash_computation(c: &mut Criterion) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let test_strings: Vec<String> = (0..1000).map(|i| format!("test_string_{}", i)).collect();
    
    c.bench_function("hash_computation", |b| {
        b.iter(|| {
            for s in &test_strings {
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                let _ = hasher.finish();
            }
        });
    });
}

fn bench_detector_metrics(c: &mut Criterion) {
    let mut group = c.benchmark_group("detector_metrics");
    
    for iterations in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("record", iterations), &iterations, |b, &iterations| {
            b.iter(|| {
                let mut detector = BenchmarkDetector::new();
                for i in 0..iterations {
                    detector.record("test_metric", i as f64);
                }
                let _ = detector.collect(&BenchmarkConfig::default());
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_detector_startup,
    bench_memory_allocation,
    bench_json_serialization,
    bench_json_deserialization,
    bench_hash_computation,
    bench_detector_metrics
);

criterion_main!(benches);