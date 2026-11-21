//! Metric types and value representations for benchmarks

use crate::BenchmarkCategory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// A single benchmark metric measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetric {
    /// Unique metric identifier
    pub name: String,
    /// Category this metric belongs to
    pub category: BenchmarkCategory,
    /// Measured value
    pub value: MetricValue,
    /// Unit of measurement
    pub unit: MetricUnit,
    /// Timestamp of measurement
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Git commit hash at measurement time
    pub commit: Option<String>,
    /// Git branch
    pub branch: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Configuration used for this run
    pub config_hash: String,
}

/// Metric value with statistical information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    /// Mean value
    pub mean: f64,
    /// Median value
    pub median: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Number of samples
    pub sample_count: usize,
    /// Percentiles (p50, p90, p95, p99)
    pub percentiles: Percentiles,
}

/// Percentile values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Percentiles {
    pub p50: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
}

/// Unit of measurement for metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricUnit {
    /// Nanoseconds
    Nanoseconds,
    /// Microseconds
    Microseconds,
    /// Milliseconds
    Milliseconds,
    /// Seconds
    Seconds,
    /// Bytes
    Bytes,
    /// Kilobytes
    Kilobytes,
    /// Megabytes
    Megabytes,
    /// Operations per second
    OpsPerSecond,
    /// Requests per second
    RequestsPerSecond,
    /// Count (dimensionless)
    Count,
    /// Percentage
    Percentage,
    /// Custom unit (stored as string in metadata)
    Custom,
}

impl MetricUnit {
    /// Convert value to base unit for comparison
    pub fn to_base(&self, value: f64) -> f64 {
        match self {
            MetricUnit::Nanoseconds => value,
            MetricUnit::Microseconds => value * 1_000.0,
            MetricUnit::Milliseconds => value * 1_000_000.0,
            MetricUnit::Seconds => value * 1_000_000_000.0,
            MetricUnit::Bytes => value,
            MetricUnit::Kilobytes => value * 1_024.0,
            MetricUnit::Megabytes => value * 1_024.0 * 1_024.0,
            MetricUnit::OpsPerSecond => value,
            MetricUnit::RequestsPerSecond => value,
            MetricUnit::Count => value,
            MetricUnit::Percentage => value,
            MetricUnit::Custom => value,
        }
    }
    
    /// Get display name
    pub fn display(&self) -> &'static str {
        match self {
            MetricUnit::Nanoseconds => "ns",
            MetricUnit::Microseconds => "µs",
            MetricUnit::Milliseconds => "ms",
            MetricUnit::Seconds => "s",
            MetricUnit::Bytes => "B",
            MetricUnit::Kilobytes => "KB",
            MetricUnit::Megabytes => "MB",
            MetricUnit::OpsPerSecond => "ops/s",
            MetricUnit::RequestsPerSecond => "req/s",
            MetricUnit::Count => "count",
            MetricUnit::Percentage => "%",
            MetricUnit::Custom => "custom",
        }
    }
}

impl MetricValue {
    /// Create a new metric value from samples
    pub fn from_samples(samples: &[f64]) -> Self {
        if samples.is_empty() {
            return Self::default();
        }
        
        let mut sorted = samples.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let count = samples.len();
        let mean = samples.iter().sum::<f64>() / count as f64;
        let median = if count % 2 == 0 {
            (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
        } else {
            sorted[count / 2]
        };
        
        let variance = samples.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();
        
        let percentile = |p: f64| {
            let idx = ((p / 100.0) * (count - 1) as f64).round() as usize;
            sorted[idx.min(count - 1)]
        };
        
        Self {
            mean,
            median,
            std_dev,
            min: sorted[0],
            max: sorted[count - 1],
            sample_count: count,
            percentiles: Percentiles {
                p50: percentile(50.0),
                p90: percentile(90.0),
                p95: percentile(95.0),
                p99: percentile(99.0),
            },
        }
    }
    
    /// Calculate coefficient of variation
    pub fn cv(&self) -> f64 {
        if self.mean == 0.0 {
            0.0
        } else {
            self.std_dev / self.mean
        }
    }
    
    /// Check if value is stable (CV < 5%)
    pub fn is_stable(&self) -> bool {
        self.cv() < 0.05
    }
}

impl Default for MetricValue {
    fn default() -> Self {
        Self {
            mean: 0.0,
            median: 0.0,
            std_dev: 0.0,
            min: 0.0,
            max: 0.0,
            sample_count: 0,
            percentiles: Percentiles {
                p50: 0.0,
                p90: 0.0,
                p95: 0.0,
                p99: 0.0,
            },
        }
    }
}

impl BenchmarkMetric {
    /// Create a new benchmark metric
    pub fn new(
        name: impl Into<String>,
        category: crate::BenchmarkCategory,
        value: MetricValue,
        unit: MetricUnit,
    ) -> Self {
        Self {
            name: name.into(),
            category,
            value,
            unit,
            timestamp: chrono::Utc::now(),
            commit: get_git_commit(),
            branch: get_git_branch(),
            metadata: HashMap::new(),
            config_hash: compute_config_hash(),
        }
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Get value in base units for comparison
    pub fn base_value(&self) -> f64 {
        self.unit.to_base(self.value.mean)
    }
    
    /// Calculate relative change from another metric
    pub fn relative_change(&self, other: &BenchmarkMetric) -> f64 {
        if other.value.mean == 0.0 {
            return 0.0;
        }
        (self.base_value() - other.base_value()) / other.base_value()
    }
}

/// Get current git commit hash
fn get_git_commit() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Get current git branch
fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Compute hash of current benchmark configuration
fn compute_config_hash() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::env::var("CARGO_PKG_VERSION").unwrap_or_default().hash(&mut hasher);
    std::env::var("RUSTFLAGS").unwrap_or_default().hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metric_value_from_samples() {
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let metric = MetricValue::from_samples(&samples);
        assert_eq!(metric.mean, 3.0);
        assert_eq!(metric.median, 3.0);
        assert_eq!(metric.min, 1.0);
        assert_eq!(metric.max, 5.0);
        assert_eq!(metric.sample_count, 5);
    }
    
    #[test]
    fn test_metric_unit_conversion() {
        assert_eq!(MetricUnit::Milliseconds.to_base(1.0), 1_000_000.0);
        assert_eq!(MetricUnit::Microseconds.to_base(1.0), 1_000.0);
        assert_eq!(MetricUnit::Seconds.to_base(1.0), 1_000_000_000.0);
    }
    
    #[test]
    fn test_relative_change() {
        let value1 = MetricValue { mean: 100.0, ..Default::default() };
        let value2 = MetricValue { mean: 110.0, ..Default::default() };
        
        let metric1 = BenchmarkMetric::new("test", crate::BenchmarkCategory::Detector, value1, MetricUnit::Milliseconds);
        let metric2 = BenchmarkMetric::new("test", crate::BenchmarkCategory::Detector, value2, MetricUnit::Milliseconds);
        
        let change = metric2.relative_change(&metric1);
        assert!((change - 0.1).abs() < 0.001);
    }
}