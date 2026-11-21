//! impeccable-benchmark: Benchmark regression detection for design quality metrics.
//!
//! This crate provides a benchmarking framework for measuring and tracking
//! design quality metrics over time, with automatic regression detection
//! using a 5% threshold.

pub mod detector;
pub mod metrics;
pub mod regression;
pub mod storage;
pub mod runner;

use crate::detector::BenchmarkDetector;
use crate::runner::BenchmarkRunner;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use thiserror::Error;

/// Main benchmark engine combining runner, detector, and regression detection
pub struct BenchmarkEngine {
    runner: BenchmarkRunner,
    detector: BenchmarkDetector,
    regression_detector: RegressionDetector,
    storage: BenchmarkStorage,
}

impl BenchmarkEngine {
    /// Create a new benchmark engine with default configuration
    pub async fn new(db_path: &str) -> Result<Self, BenchmarkError> {
        let storage = BenchmarkStorage::new(db_path).await?;
        let runner = BenchmarkRunner::new();
        let detector = BenchmarkDetector::new();
        let regression_detector = RegressionDetector::new(RegressionThreshold::default());
        
        Ok(Self {
            runner,
            detector,
            regression_detector,
            storage,
        })
    }

    /// Run all benchmarks and store results
    pub async fn run_all(&mut self) -> Result<Vec<BenchmarkMetric>, BenchmarkError> {
        let metrics = self.runner.run_all(&mut self.detector).await?;
        
        // Store results
        for metric in &metrics {
            self.storage.store(metric).await?;
        }
        
        Ok(metrics)
    }

    /// Run benchmarks and check for regressions
    pub async fn run_with_regression_check(&mut self) -> Result<RegressionReport, BenchmarkError> {
        let metrics = self.run_all().await?;
        let report = self.regression_detector.check(&metrics, &self.storage).await?;
        Ok(report)
    }

    /// Get historical data for a metric
    pub async fn get_history(&self, metric_name: &str) -> Result<Vec<StoredBenchmark>, BenchmarkError> {
        self.storage.get_history(metric_name).await.map_err(BenchmarkError::Storage)
    }

    /// Compare current metrics against baseline
    pub async fn compare_with_baseline(&mut self, baseline_name: &str) -> Result<RegressionReport, BenchmarkError> {
        let current = self.runner.run_all(&mut self.detector).await?;
        self.regression_detector.compare_with_baseline(&current, baseline_name, &self.storage).await
            .map_err(BenchmarkError::RegressionDetection)
    }
}

/// Configuration for benchmark runs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkConfig {
    /// Number of iterations per benchmark
    pub iterations: usize,
    /// Warmup iterations (not counted in results)
    pub warmup: usize,
    /// Regression threshold (5% by default)
    pub regression_threshold: f64,
    /// Minimum sample size for statistical significance
    pub min_samples: usize,
    /// Output directory for reports
    pub output_dir: Option<String>,
    /// Enable detailed profiling
    pub profile: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            warmup: 10,
            regression_threshold: 0.05, // 5%
            min_samples: 30,
            output_dir: None,
            profile: false,
        }
    }
}

/// Errors that can occur during benchmarking
#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Benchmark error: {0}")]
    Benchmark(String),
    #[error("Regression detected: {0}")]
    Regression(String),
    #[error("Insufficient data for comparison")]
    InsufficientData,
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Regression detection error: {0}")]
    RegressionDetection(#[from] crate::regression::RegressionError),
    #[error("Storage error: {0}")]
    Storage(#[from] crate::storage::StorageError),
    #[error("String error: {0}")]
    String(String),
}

impl From<String> for BenchmarkError {
    fn from(s: String) -> Self {
        BenchmarkError::String(s)
    }
}

impl From<&str> for BenchmarkError {
    fn from(s: &str) -> Self {
        BenchmarkError::String(s.to_string())
    }
}

/// Benchmark categories for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BenchmarkCategory {
    /// Detector rule performance
    Detector,
    /// MCP server latency
    Mcp,
    /// Live mode performance
    Live,
    /// Rule evaluation speed
    RuleEvaluation,
    /// Memory usage
    Memory,
    /// Startup time
    Startup,
    /// CI/CD pipeline performance
    CiCd,
}

impl std::fmt::Display for BenchmarkCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchmarkCategory::Detector => write!(f, "detector"),
            BenchmarkCategory::Mcp => write!(f, "mcp"),
            BenchmarkCategory::Live => write!(f, "live"),
            BenchmarkCategory::RuleEvaluation => write!(f, "rule_evaluation"),
            BenchmarkCategory::Memory => write!(f, "memory"),
            BenchmarkCategory::Startup => write!(f, "startup"),
            BenchmarkCategory::CiCd => write!(f, "ci_cd"),
        }
    }
}

impl std::str::FromStr for BenchmarkCategory {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "detector" => Ok(BenchmarkCategory::Detector),
            "mcp" => Ok(BenchmarkCategory::Mcp),
            "live" => Ok(BenchmarkCategory::Live),
            "rule_evaluation" => Ok(BenchmarkCategory::RuleEvaluation),
            "memory" => Ok(BenchmarkCategory::Memory),
            "startup" => Ok(BenchmarkCategory::Startup),
            "ci_cd" => Ok(BenchmarkCategory::CiCd),
            _ => Err(format!("Unknown category: {}", s)),
        }
    }
}

/// Stored benchmark record from database
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredBenchmark {
    pub id: i64,
    pub name: String,
    pub category: BenchmarkCategory,
    pub value: MetricValue,
    pub unit: MetricUnit,
    pub timestamp: DateTime<Utc>,
    pub commit: Option<String>,
    pub branch: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub config_hash: String,
}

/// Trait for benchmarkable components
#[async_trait::async_trait]
pub trait Benchmarkable {
    /// Run the benchmark and return metrics
    async fn benchmark(&self, config: &BenchmarkConfig) -> Result<Vec<BenchmarkMetric>, BenchmarkError>;
    
    /// Get the benchmark category
    fn category(&self) -> BenchmarkCategory;
    
    /// Get the benchmark name
    fn name(&self) -> &str;
}

// Re-export key types
pub use crate::metrics::{BenchmarkMetric, MetricValue, MetricUnit, Percentiles};
pub use crate::regression::{RegressionDetector, RegressionReport, RegressionThreshold, RegressionFinding, RegressionSeverity, RegressionSummary, RegressionStatus, RegressionError};
pub use crate::storage::{BenchmarkStorage, StorageError};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_benchmark_engine_creation() {
        let engine = BenchmarkEngine::new(":memory:").await;
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_benchmark_config_default() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.iterations, 100);
        assert_eq!(config.regression_threshold, 0.05);
    }
    
    #[test]
    fn test_benchmark_category_display() {
        assert_eq!(BenchmarkCategory::Detector.to_string(), "detector");
        assert_eq!(BenchmarkCategory::Mcp.to_string(), "mcp");
    }
}