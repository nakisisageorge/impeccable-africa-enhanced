//! Benchmark detector for measuring design quality metrics

use crate::{BenchmarkCategory, BenchmarkMetric, BenchmarkConfig, MetricValue, MetricUnit};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Benchmark detector for measuring various performance metrics
pub struct BenchmarkDetector {
    /// Accumulated samples for each metric
    samples: HashMap<String, Vec<f64>>,
    /// Start times for timing measurements
    start_times: HashMap<String, Instant>,
}

impl BenchmarkDetector {
    /// Create a new benchmark detector
    pub fn new() -> Self {
        Self {
            samples: HashMap::new(),
            start_times: HashMap::new(),
        }
    }
    
    /// Start timing a metric
    pub fn start(&mut self, name: &str) {
        self.start_times.insert(name.to_string(), Instant::now());
    }
    
    /// Stop timing and record the measurement
    pub fn stop(&mut self, name: &str) -> Option<Duration> {
        self.start_times.remove(name).map(|start| start.elapsed())
    }
    
    /// Record a measurement directly
    pub fn record(&mut self, name: &str, value: f64) {
        self.samples.entry(name.to_string()).or_default().push(value);
    }
    
    /// Record a duration measurement in nanoseconds
    pub fn record_duration(&mut self, name: &str, duration: Duration) {
        self.record(name, duration.as_nanos() as f64);
    }
    
    /// Get all collected metrics
    pub fn collect(&self, config: &BenchmarkConfig) -> Vec<BenchmarkMetric> {
        let mut metrics = Vec::new();
        
        for (name, samples) in &self.samples {
            if samples.len() < config.min_samples {
                continue; // Skip metrics with insufficient samples
            }
            
            let value = MetricValue::from_samples(samples);
            let category = Self::infer_category(name);
            let unit = Self::infer_unit(name);
            
            metrics.push(BenchmarkMetric::new(name, category, value, unit));
        }
        
        metrics
    }
    
    /// Infer benchmark category from metric name
    fn infer_category(name: &str) -> BenchmarkCategory {
        if name.contains("detect") || name.contains("rule") {
            BenchmarkCategory::Detector
        } else if name.contains("mcp") || name.contains("transport") {
            BenchmarkCategory::Mcp
        } else if name.contains("live") {
            BenchmarkCategory::Live
        } else if name.contains("memory") || name.contains("alloc") {
            BenchmarkCategory::Memory
        } else if name.contains("startup") || name.contains("init") {
            BenchmarkCategory::Startup
        } else if name.contains("ci") || name.contains("build") {
            BenchmarkCategory::CiCd
        } else {
            BenchmarkCategory::RuleEvaluation
        }
    }
    
    /// Infer metric unit from metric name
    fn infer_unit(name: &str) -> MetricUnit {
        if name.contains("time") || name.contains("duration") || name.contains("latency") {
            MetricUnit::Nanoseconds
        } else if name.contains("memory") || name.contains("alloc") || name.contains("heap") {
            MetricUnit::Bytes
        } else if name.contains("throughput") || name.contains("ops") || name.contains("rate") {
            MetricUnit::OpsPerSecond
        } else if name.contains("count") || name.contains("total") {
            MetricUnit::Count
        } else if name.contains("percent") || name.contains("ratio") {
            MetricUnit::Percentage
        } else {
            MetricUnit::Nanoseconds
        }
    }
    
    /// Clear all collected samples
    pub fn clear(&mut self) {
        self.samples.clear();
        self.start_times.clear();
    }
}

impl Default for BenchmarkDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detector_basic() {
        let mut detector = BenchmarkDetector::new();
        detector.record("test_metric", 100.0);
        detector.record("test_metric", 200.0);
        detector.record("test_metric", 300.0);
        
        let config = BenchmarkConfig { min_samples: 2, ..Default::default() };
        let metrics = detector.collect(&config);
        
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "test_metric");
        assert_eq!(metrics[0].value.mean, 200.0);
    }
    
    #[test]
    fn test_category_inference() {
        assert_eq!(BenchmarkDetector::infer_category("detect_time"), BenchmarkCategory::Detector);
        assert_eq!(BenchmarkDetector::infer_category("mcp_latency"), BenchmarkCategory::Mcp);
        assert_eq!(BenchmarkDetector::infer_category("live_mode"), BenchmarkCategory::Live);
    }
}