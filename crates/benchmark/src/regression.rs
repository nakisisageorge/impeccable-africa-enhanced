//! Regression detection for benchmark metrics

use crate::{BenchmarkCategory, BenchmarkMetric, MetricValue, BenchmarkStorage, StoredBenchmark};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Regression detection threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionThreshold {
    /// Warning threshold (5% by default)
    pub warning: f64,
    /// Critical threshold (10% by default)
    pub critical: f64,
    /// Minimum samples required for comparison
    pub min_samples: usize,
}

impl Default for RegressionThreshold {
    fn default() -> Self {
        Self {
            warning: 0.05,  // 5%
            critical: 0.10, // 10%
            min_samples: 10,
        }
    }
}

/// Regression severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegressionSeverity {
    /// No regression detected
    None,
    /// Performance improved
    Improved,
    /// Minor regression (warning)
    Warning,
    /// Major regression (critical)
    Critical,
}

/// Individual regression finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionFinding {
    /// Metric name
    pub metric_name: String,
    /// Metric category
    pub category: BenchmarkCategory,
    /// Current value
    pub current_value: f64,
    /// Baseline value
    pub baseline_value: f64,
    /// Relative change (positive = regression for time metrics)
    pub relative_change: f64,
    /// Severity level
    pub severity: RegressionSeverity,
    /// Whether higher values are worse (default: true for time/memory)
    pub higher_is_worse: bool,
    /// Statistical confidence (0-1)
    pub confidence: f64,
}

impl RegressionFinding {
    /// Create a new regression finding
    pub fn new(
        metric: &BenchmarkMetric,
        baseline: &StoredBenchmark,
        threshold: &RegressionThreshold,
    ) -> Self {
        let current = metric.base_value();
        let baseline_val = baseline.unit.to_base(baseline.value.mean);
        let relative_change = if baseline_val != 0.0 {
            (current - baseline_val) / baseline_val
        } else {
            0.0
        };
        
        let higher_is_worse = matches!(
            metric.unit,
            crate::metrics::MetricUnit::Nanoseconds
                | crate::metrics::MetricUnit::Microseconds
                | crate::metrics::MetricUnit::Milliseconds
                | crate::metrics::MetricUnit::Seconds
                | crate::metrics::MetricUnit::Bytes
                | crate::metrics::MetricUnit::Kilobytes
                | crate::metrics::MetricUnit::Megabytes
        );
        
        let severity = if relative_change.abs() < threshold.warning {
            RegressionSeverity::None
        } else if relative_change < -threshold.warning {
            RegressionSeverity::Improved
        } else if relative_change >= threshold.critical {
            RegressionSeverity::Critical
        } else {
            RegressionSeverity::Warning
        };
        
        // Calculate confidence based on sample sizes and variance
        let confidence = calculate_confidence(&metric.value, &baseline.value);
        
        Self {
            metric_name: metric.name.clone(),
            category: metric.category,
            current_value: current,
            baseline_value: baseline_val,
            relative_change,
            severity,
            higher_is_worse,
            confidence,
        }
    }
    
    /// Get human-readable description
    pub fn description(&self) -> String {
        let direction = if self.relative_change > 0.0 { "increased" } else { "decreased" };
        let pct = (self.relative_change * 100.0).abs();
        
        match self.severity {
            RegressionSeverity::None => format!(
                "{} {} by {:.1}% (within threshold)",
                self.metric_name, direction, pct
            ),
            RegressionSeverity::Improved => format!(
                "{} improved by {:.1}%",
                self.metric_name, pct
            ),
            RegressionSeverity::Warning => format!(
                "⚠ {} {} by {:.1}% (warning threshold exceeded)",
                self.metric_name, direction, pct
            ),
            RegressionSeverity::Critical => format!(
                "🚨 {} {} by {:.1}% (CRITICAL - critical threshold exceeded)",
                self.metric_name, direction, pct
            ),
        }
    }
}

/// Complete regression report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
    /// Timestamp of report generation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Git commit compared against
    pub baseline_commit: Option<String>,
    /// Current commit
    pub current_commit: Option<String>,
    /// All findings
    pub findings: Vec<RegressionFinding>,
    /// Summary counts
    pub summary: RegressionSummary,
    /// Overall status
    pub status: RegressionStatus,
}

/// Summary of regression findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionSummary {
    pub total: usize,
    pub improved: usize,
    pub stable: usize,
    pub warnings: usize,
    pub critical: usize,
}

/// Overall regression status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegressionStatus {
    /// All metrics within thresholds
    Pass,
    /// Some warnings but no critical issues
    Warning,
    /// Critical regressions detected
    Fail,
}

/// Regression detector
pub struct RegressionDetector {
    threshold: RegressionThreshold,
}

impl RegressionDetector {
    /// Create a new regression detector
    pub fn new(threshold: RegressionThreshold) -> Self {
        Self { threshold }
    }
    
    /// Check current metrics against historical data for regressions
    pub async fn check(
        &self,
        metrics: &[BenchmarkMetric],
        storage: &BenchmarkStorage,
    ) -> Result<RegressionReport, RegressionError> {
        let mut findings = Vec::new();
        let current_commit = metrics.first().and_then(|m| m.commit.clone());
        let baseline_commit = storage.get_latest_baseline_commit().await?;
        
        for metric in metrics {
            if let Some(baseline) = storage.get_latest_baseline(&metric.name).await? {
                if baseline.value.sample_count >= self.threshold.min_samples
                    && metric.value.sample_count >= self.threshold.min_samples
                {
                    let finding = RegressionFinding::new(metric, &baseline, &self.threshold);
                    findings.push(finding);
                }
            }
        }
        
        let summary = RegressionSummary {
            total: findings.len(),
            improved: findings.iter().filter(|f| f.severity == RegressionSeverity::Improved).count(),
            stable: findings.iter().filter(|f| f.severity == RegressionSeverity::None).count(),
            warnings: findings.iter().filter(|f| f.severity == RegressionSeverity::Warning).count(),
            critical: findings.iter().filter(|f| f.severity == RegressionSeverity::Critical).count(),
        };
        
        let status = if summary.critical > 0 {
            RegressionStatus::Fail
        } else if summary.warnings > 0 {
            RegressionStatus::Warning
        } else {
            RegressionStatus::Pass
        };
        
        Ok(RegressionReport {
            timestamp: chrono::Utc::now(),
            baseline_commit,
            current_commit,
            findings,
            summary,
            status,
        })
    }
    
    /// Compare against a specific baseline
    pub async fn compare_with_baseline(
        &self,
        metrics: &[BenchmarkMetric],
        baseline_name: &str,
        storage: &BenchmarkStorage,
    ) -> Result<RegressionReport, RegressionError> {
        let mut findings = Vec::new();
        let current_commit = metrics.first().and_then(|m| m.commit.clone());
        
        for metric in metrics {
            if let Some(baseline) = storage.get_baseline_by_name(baseline_name, &metric.name).await? {
                if baseline.value.sample_count >= self.threshold.min_samples
                    && metric.value.sample_count >= self.threshold.min_samples
                {
                    let finding = RegressionFinding::new(metric, &baseline, &self.threshold);
                    findings.push(finding);
                }
            }
        }
        
        let summary = RegressionSummary {
            total: findings.len(),
            improved: findings.iter().filter(|f| f.severity == RegressionSeverity::Improved).count(),
            stable: findings.iter().filter(|f| f.severity == RegressionSeverity::None).count(),
            warnings: findings.iter().filter(|f| f.severity == RegressionSeverity::Warning).count(),
            critical: findings.iter().filter(|f| f.severity == RegressionSeverity::Critical).count(),
        };
        
        let status = if summary.critical > 0 {
            RegressionStatus::Fail
        } else if summary.warnings > 0 {
            RegressionStatus::Warning
        } else {
            RegressionStatus::Pass
        };
        
        Ok(RegressionReport {
            timestamp: chrono::Utc::now(),
            baseline_commit: Some(baseline_name.to_string()),
            current_commit,
            findings,
            summary,
            status,
        })
    }
}

/// Calculate statistical confidence for comparison
fn calculate_confidence(current: &crate::metrics::MetricValue, baseline: &crate::metrics::MetricValue) -> f64 {
    // Simplified confidence based on sample size and coefficient of variation
    let min_samples = current.sample_count.min(baseline.sample_count) as f64;
    let max_cv = current.cv().max(baseline.cv());
    
    let sample_confidence = (min_samples / 30.0).min(1.0); // Max confidence at 30 samples
    let variance_confidence = (1.0 - max_cv).max(0.0); // Higher variance = lower confidence
    
    (sample_confidence * variance_confidence).clamp(0.0, 1.0)
}

/// Regression detection errors
#[derive(Error, Debug)]
pub enum RegressionError {
    #[error("Storage error: {0}")]
    Storage(#[from] crate::storage::StorageError),
    #[error("Insufficient data for comparison: {0}")]
    InsufficientData(String),
    #[error("Configuration error: {0}")]
    Config(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MetricValue, BenchmarkMetric, MetricUnit, Percentiles, StoredBenchmark};
    use chrono::Utc;
    
    fn create_test_metric(name: &str, mean: f64) -> BenchmarkMetric {
        BenchmarkMetric::new(
            name,
            BenchmarkCategory::Detector,
            MetricValue {
                mean,
                median: mean,
                std_dev: mean * 0.02,
                min: mean * 0.95,
                max: mean * 1.05,
                sample_count: 30,
                percentiles: Percentiles {
                    p50: mean,
                    p90: mean * 1.03,
                    p95: mean * 1.05,
                    p99: mean * 1.08,
                },
            },
            MetricUnit::Milliseconds,
        )
    }
    
    fn create_test_baseline(name: &str, mean: f64) -> StoredBenchmark {
        StoredBenchmark {
            id: 1,
            name: name.to_string(),
            category: BenchmarkCategory::Detector,
            value: MetricValue {
                mean,
                median: mean,
                std_dev: mean * 0.02,
                min: mean * 0.95,
                max: mean * 1.05,
                sample_count: 30,
                percentiles: Percentiles {
                    p50: mean,
                    p90: mean * 1.03,
                    p95: mean * 1.05,
                    p99: mean * 1.08,
                },
            },
            unit: MetricUnit::Milliseconds,
            timestamp: Utc::now(),
            commit: Some("abc123".to_string()),
            branch: Some("main".to_string()),
            metadata: Default::default(),
            config_hash: "test".to_string(),
        }
    }
    
    #[test]
    fn test_regression_finding_creation() {
        let metric = create_test_metric("test", 110.0); // 10% regression
        let baseline = create_test_baseline("test", 100.0);
        let threshold = RegressionThreshold::default();
        
        let finding = RegressionFinding::new(&metric, &baseline, &threshold);
        
        assert_eq!(finding.metric_name, "test");
        assert!((finding.relative_change - 0.1).abs() < 0.01);
        assert_eq!(finding.severity, RegressionSeverity::Critical);
        assert!(finding.higher_is_worse);
    }
    
    #[test]
    fn test_improvement_detection() {
        let metric = create_test_metric("test", 90.0); // 10% improvement
        let baseline = create_test_baseline("test", 100.0);
        let threshold = RegressionThreshold::default();
        
        let finding = RegressionFinding::new(&metric, &baseline, &threshold);
        
        assert!((finding.relative_change - (-0.1)).abs() < 0.01);
        assert_eq!(finding.severity, RegressionSeverity::Improved);
    }
    
    #[test]
    fn test_stable_detection() {
        let metric = create_test_metric("test", 102.0); // 2% change - within warning threshold
        let baseline = create_test_baseline("test", 100.0);
        let threshold = RegressionThreshold::default();
        
        let finding = RegressionFinding::new(&metric, &baseline, &threshold);
        
        assert_eq!(finding.severity, RegressionSeverity::None);
    }
}