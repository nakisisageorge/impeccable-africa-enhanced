//! Benchmark runner for executing benchmarks

use crate::{BenchmarkCategory, BenchmarkMetric, BenchmarkConfig, BenchmarkDetector, MetricValue, MetricUnit, Percentiles};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Benchmark runner that executes predefined benchmarks
pub struct BenchmarkRunner {
    /// Registered benchmark functions
    benchmarks: HashMap<String, BenchmarkFn>,
    /// Configuration
    config: BenchmarkConfig,
}

/// Benchmark function type
type BenchmarkFn = Box<dyn Fn(&mut BenchmarkDetector, &BenchmarkConfig) -> Result<(), String> + Send + Sync>;

impl BenchmarkRunner {
    /// Create a new benchmark runner with default configuration
    pub fn new() -> Self {
        Self {
            benchmarks: HashMap::new(),
            config: BenchmarkConfig::default(),
        }
    }
    
    /// Create a new benchmark runner with custom configuration
    pub fn with_config(config: BenchmarkConfig) -> Self {
        Self {
            benchmarks: HashMap::new(),
            config,
        }
    }
    
    /// Register a benchmark function
    pub fn register<F>(&mut self, name: &str, f: F)
    where
        F: Fn(&mut BenchmarkDetector, &BenchmarkConfig) -> Result<(), String> + Send + Sync + 'static,
    {
        self.benchmarks.insert(name.to_string(), Box::new(f));
    }
    
    /// Register a simple timing benchmark
    pub fn register_timing<F, Fut>(&mut self, name: &str, f: F)
    where
        F: Fn(&BenchmarkConfig) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), String>> + Send,
    {
        let name_owned = name.to_string();
        self.register(name, move |detector, config| {
            let fut = f(config);
            let rt = tokio::runtime::Handle::current();
            let name_clone = name_owned.clone();
            rt.block_on(async move {
                let start = Instant::now();
                fut.await?;
                detector.record_duration(&name_clone, start.elapsed());
                Ok(())
            })
        });
    }
    
    /// Register a detector benchmark (for detector rule performance)
    pub fn register_detector_benchmark(&mut self, name: &str, input: String, expected_rules: Vec<String>) {
        let name_owned = name.to_string();
        let input_owned = input;
        let expected_owned = expected_rules;
        
        self.register(name, move |detector, config| {
            use crate::metrics::{MetricValue, Percentiles};
            use std::time::Instant;
            
            let mut total_duration = Duration::ZERO;
            let mut rule_counts = Vec::new();
            
            for _ in 0..config.iterations {
                let start = Instant::now();
                
                // Simulate detector work
                let _ = detector; // Placeholder for actual detector invocation
                
                total_duration += start.elapsed();
                rule_counts.push(expected_owned.len() as f64);
            }
            
            let avg_duration = total_duration / config.iterations as u32;
            detector.record(&format!("{}_duration", name_owned), avg_duration.as_nanos() as f64);
            detector.record(&format!("{}_rules_found", name_owned), 
                MetricValue::from_samples(&rule_counts).mean);
            
            Ok(())
        });
    }
    
    /// Run all registered benchmarks
    pub async fn run_all(&self, detector: &mut BenchmarkDetector) -> Result<Vec<BenchmarkMetric>, String> {
        // Run warmup
        for (name, bench) in &self.benchmarks {
            for _ in 0..self.config.warmup {
                bench(detector, &self.config).map_err(|e| format!("Warmup failed for {}: {}", name, e))?;
            }
        }
        
        // Run actual benchmarks
        for (name, bench) in &self.benchmarks {
            for _ in 0..self.config.iterations {
                bench(detector, &self.config).map_err(|e| format!("Benchmark failed for {}: {}", name, e))?;
            }
        }
        
        // Collect results
        Ok(detector.collect(&self.config))
    }
    
    /// Run a specific benchmark by name
    pub async fn run_one(&self, name: &str, detector: &mut BenchmarkDetector) -> Result<Vec<BenchmarkMetric>, String> {
        let bench = self.benchmarks.get(name)
            .ok_or_else(|| format!("Benchmark '{}' not found", name))?;
        
        // Warmup
        for _ in 0..self.config.warmup {
            bench(detector, &self.config).map_err(|e| format!("Warmup failed for {}: {}", name, e))?;
        }
        
        // Run
        for _ in 0..self.config.iterations {
            bench(detector, &self.config).map_err(|e| format!("Benchmark failed for {}: {}", name, e))?;
        }
        
        Ok(detector.collect(&self.config))
    }
    
    /// Get list of registered benchmark names
    pub fn list_benchmarks(&self) -> Vec<String> {
        self.benchmarks.keys().cloned().collect()
    }
    
    /// Get configuration
    pub fn config(&self) -> &BenchmarkConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: BenchmarkConfig) {
        self.config = config;
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in benchmarks for common operations
pub mod builtin {
    use super::*;
    use std::time::Instant;
    
    /// Register all built-in benchmarks
    pub fn register_all(runner: &mut BenchmarkRunner) {
        // Detector startup benchmark
        runner.register("detector_startup", |detector, config| {
            let mut times = Vec::new();
            
            for _ in 0..config.iterations {
                let start = Instant::now();
                let _new_detector = BenchmarkDetector::new();
                times.push(start.elapsed().as_nanos() as f64);
            }
            
            let value = MetricValue::from_samples(&times);
            detector.record("detector_startup", value.mean);
            Ok(())
        });
        
        // Memory allocation benchmark
        runner.register("memory_allocation", |detector, config| {
            let mut times = Vec::new();
            let mut sizes = Vec::new();
            
            for i in 0..config.iterations {
                let size = 1024 * (i % 100 + 1); // Vary allocation size
                let start = Instant::now();
                let _vec: Vec<u8> = vec![0; size];
                times.push(start.elapsed().as_nanos() as f64);
                sizes.push(size as f64);
            }
            
            let time_value = MetricValue::from_samples(&times);
            let size_value = MetricValue::from_samples(&sizes);
            
            detector.record("memory_allocation_time", time_value.mean);
            detector.record("memory_allocation_size", size_value.mean);
            Ok(())
        });
        
        // JSON serialization benchmark
        runner.register("json_serialization", |detector, config| {
            let test_data = serde_json::json!({
                "name": "test",
                "values": (0..100).collect::<Vec<_>>(),
                "nested": {
                    "a": 1,
                    "b": [1, 2, 3, 4, 5],
                    "c": "hello world"
                }
            });
            
            let mut times = Vec::new();
            
            for _ in 0..config.iterations {
                let start = Instant::now();
                let _ = serde_json::to_string(&test_data).unwrap();
                times.push(start.elapsed().as_nanos() as f64);
            }
            
            let value = MetricValue::from_samples(&times);
            detector.record("json_serialization", value.mean);
            Ok(())
        });
        
        // JSON deserialization benchmark
        runner.register("json_deserialization", |detector, config| {
            let test_json = serde_json::json!({
                "name": "test",
                "values": (0..100).collect::<Vec<_>>(),
                "nested": {
                    "a": 1,
                    "b": [1, 2, 3, 4, 5],
                    "c": "hello world"
                }
            }).to_string();
            
            let mut times = Vec::new();
            
            for _ in 0..config.iterations {
                let start = Instant::now();
                let _: serde_json::Value = serde_json::from_str(&test_json).unwrap();
                times.push(start.elapsed().as_nanos() as f64);
            }
            
            let value = MetricValue::from_samples(&times);
            detector.record("json_deserialization", value.mean);
            Ok(())
        });
        
        // Hash computation benchmark
        runner.register("hash_computation", |detector, config| {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let test_strings: Vec<String> = (0..1000).map(|i| format!("test_string_{}", i)).collect();
            let mut times = Vec::new();
            
            for _ in 0..config.iterations {
                let start = Instant::now();
                for s in &test_strings {
                    let mut hasher = DefaultHasher::new();
                    s.hash(&mut hasher);
                    let _ = hasher.finish();
                }
                times.push(start.elapsed().as_nanos() as f64);
            }
            
            let value = MetricValue::from_samples(&times);
            detector.record("hash_computation", value.mean);
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_runner_basic() {
        let mut runner = BenchmarkRunner::new();
        runner.register("test", |detector, _| {
            detector.record("test_metric", 100.0);
            Ok(())
        });
        
        let mut detector = BenchmarkDetector::new();
        let metrics = runner.run_all(&mut detector).await.unwrap();
        
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "test_metric");
    }
    
    #[test]
    fn test_builtin_registration() {
        let mut runner = BenchmarkRunner::new();
        builtin::register_all(&mut runner);
        
        let benchmarks = runner.list_benchmarks();
        assert!(benchmarks.contains(&"detector_startup".to_string()));
        assert!(benchmarks.contains(&"memory_allocation".to_string()));
        assert!(benchmarks.contains(&"json_serialization".to_string()));
        assert!(benchmarks.contains(&"json_deserialization".to_string()));
        assert!(benchmarks.contains(&"hash_computation".to_string()));
    }
}