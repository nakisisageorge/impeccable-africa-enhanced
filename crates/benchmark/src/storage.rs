//! Storage layer for benchmark results using SQLite

use crate::{BenchmarkMetric, BenchmarkCategory, MetricValue, MetricUnit, Percentiles, StoredBenchmark};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use std::collections::HashMap;
use thiserror::Error;

/// Benchmark storage using SQLite
pub struct BenchmarkStorage {
    pool: SqlitePool,
}

impl BenchmarkStorage {
    /// Create a new benchmark storage
    pub async fn new(db_path: &str) -> Result<Self, StorageError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_path)
            .await?;
        
        let storage = Self { pool };
        storage.migrate().await?;
        Ok(storage)
    }
    
    /// Run database migrations
    async fn migrate(&self) -> Result<(), StorageError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS benchmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                value_json TEXT NOT NULL,
                unit TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                "commit" TEXT,
                branch TEXT,
                metadata_json TEXT NOT NULL DEFAULT '{}',
                config_hash TEXT NOT NULL
            );
            
            CREATE INDEX IF NOT EXISTS idx_benchmarks_name ON benchmarks(name);
            CREATE INDEX IF NOT EXISTS idx_benchmarks_category ON benchmarks(category);
            CREATE INDEX IF NOT EXISTS idx_benchmarks_timestamp ON benchmarks(timestamp);
            CREATE INDEX IF NOT EXISTS idx_benchmarks_commit ON benchmarks("commit");
            
            CREATE TABLE IF NOT EXISTS baselines (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TEXT NOT NULL,
                "commit" TEXT,
                is_active BOOLEAN NOT NULL DEFAULT 1
            );
            
            CREATE TABLE IF NOT EXISTS baseline_metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                baseline_id INTEGER NOT NULL REFERENCES baselines(id) ON DELETE CASCADE,
                metric_name TEXT NOT NULL,
                value_json TEXT NOT NULL,
                unit TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                "commit" TEXT,
                branch TEXT,
                metadata_json TEXT NOT NULL DEFAULT '{}',
                config_hash TEXT NOT NULL,
                UNIQUE(baseline_id, metric_name)
            );
            "#
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Store a benchmark metric
    pub async fn store(&self, metric: &BenchmarkMetric) -> Result<i64, StorageError> {
        let value_json = serde_json::to_string(&metric.value)?;
        let metadata_json = serde_json::to_string(&metric.metadata)?;
        
        let result = sqlx::query(
            r#"
            INSERT INTO benchmarks (name, category, value_json, unit, timestamp, "commit", branch, metadata_json, config_hash)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&metric.name)
        .bind(metric.category.to_string())
        .bind(&value_json)
        .bind(metric.unit.to_string())
        .bind(metric.timestamp.to_rfc3339())
        .bind(&metric.commit)
        .bind(&metric.branch)
        .bind(&metadata_json)
        .bind(&metric.config_hash)
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }
    
    /// Get historical data for a metric
    pub async fn get_history(&self, metric_name: &str) -> Result<Vec<StoredBenchmark>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, category, value_json, unit, timestamp, "commit", branch, metadata_json, config_hash
            FROM benchmarks
            WHERE name = ?
            ORDER BY timestamp DESC
            LIMIT 100
            "#
        )
        .bind(metric_name)
        .fetch_all(&self.pool)
        .await?;
        
        let mut benchmarks = Vec::new();
        for row in rows {
            benchmarks.push(self.row_to_stored_benchmark(row)?);
        }
        
        Ok(benchmarks)
    }
    
    /// Get the latest baseline commit
    pub async fn get_latest_baseline_commit(&self) -> Result<Option<String>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT "commit" FROM benchmarks WHERE "commit" IS NOT NULL ORDER BY timestamp DESC LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.and_then(|r| r.get::<Option<String>, _>("commit")))
    }
    
    /// Get latest baseline for a metric
    pub async fn get_latest_baseline(&self, metric_name: &str) -> Result<Option<StoredBenchmark>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, category, value_json, unit, timestamp, "commit", branch, metadata_json, config_hash
            FROM benchmarks
            WHERE name = ?
            ORDER BY timestamp ASC
            LIMIT 1
            "#
        )
        .bind(metric_name)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some(self.row_to_stored_benchmark(row)?))
        } else {
            Ok(None)
        }
    }
    
    /// Get baseline by name and metric
    pub async fn get_baseline_by_name(&self, baseline_name: &str, metric_name: &str) -> Result<Option<StoredBenchmark>, StorageError> {
        // First get baseline ID
        let baseline_id = sqlx::query(
            "SELECT id FROM baselines WHERE name = ? AND is_active = 1"
        )
        .bind(baseline_name)
        .fetch_optional(&self.pool)
        .await?;
        
        let Some(baseline_id) = baseline_id else {
            return Ok(None);
        };
        
        let baseline_id: i64 = baseline_id.get("id");
        
        let row = sqlx::query(
            r#"
            SELECT bm.id, bm.metric_name as name, b.category, bm.value_json, bm.unit, bm.timestamp, bm."commit", bm.branch, bm.metadata_json, bm.config_hash
            FROM baseline_metrics bm
            JOIN baselines b ON bm.baseline_id = b.id
            WHERE bm.baseline_id = ? AND bm.metric_name = ?
            "#
        )
        .bind(baseline_id)
        .bind(metric_name)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some(self.baseline_row_to_stored_benchmark(row)?))
        } else {
            Ok(None)
        }
    }
    
    /// Create a new baseline
    pub async fn create_baseline(&self, name: &str, description: Option<&str>, commit: Option<&str>) -> Result<i64, StorageError> {
        let result = sqlx::query(
            r#"
            INSERT INTO baselines (name, description, created_at, "commit")
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(name)
        .bind(description)
        .bind(Utc::now().to_rfc3339())
        .bind(commit)
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }
    
    /// Add metrics to a baseline
    pub async fn add_baseline_metrics(&self, baseline_id: i64, metrics: &[BenchmarkMetric]) -> Result<(), StorageError> {
        for metric in metrics {
            let value_json = serde_json::to_string(&metric.value)?;
            let metadata_json = serde_json::to_string(&metric.metadata)?;
            
            sqlx::query(
                r#"
                INSERT INTO baseline_metrics (baseline_id, metric_name, value_json, unit, timestamp, "commit", branch, metadata_json, config_hash)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(baseline_id, metric_name) DO UPDATE SET
                    value_json = excluded.value_json,
                    unit = excluded.unit,
                    timestamp = excluded.timestamp,
                    "commit" = excluded."commit",
                    branch = excluded.branch,
                    metadata_json = excluded.metadata_json,
                    config_hash = excluded.config_hash
                "#
            )
            .bind(baseline_id)
            .bind(&metric.name)
            .bind(&value_json)
            .bind(metric.unit.to_string())
            .bind(metric.timestamp.to_rfc3339())
            .bind(&metric.commit)
            .bind(&metric.branch)
            .bind(&metadata_json)
            .bind(&metric.config_hash)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }

        /// Convert row to StoredBenchmark
        fn row_to_stored_benchmark(&self, row: sqlx::sqlite::SqliteRow) -> Result<StoredBenchmark, StorageError> {
            let value: MetricValue = serde_json::from_str(row.get("value_json"))?;
            let metadata: HashMap<String, Value> = serde_json::from_str(row.get("metadata_json"))?;
            let category: BenchmarkCategory = row.get::<String, _>("category").parse()
                .unwrap_or(BenchmarkCategory::RuleEvaluation);
            let unit: MetricUnit = row.get::<String, _>("unit").parse()
                .unwrap_or(MetricUnit::Nanoseconds);

            Ok(StoredBenchmark {
                id: row.get("id"),
                name: row.get("name"),
                category,
                value,
                unit,
                timestamp: DateTime::parse_from_rfc3339(&row.get::<String, _>("timestamp"))?.with_timezone(&Utc),
                commit: row.get("commit"),
                branch: row.get("branch"),
                metadata,
                config_hash: row.get("config_hash"),
            })
        }

        fn baseline_row_to_stored_benchmark(&self, row: sqlx::sqlite::SqliteRow) -> Result<StoredBenchmark, StorageError> {
            let value: MetricValue = serde_json::from_str(row.get("value_json"))?;
            let metadata: HashMap<String, Value> = serde_json::from_str(row.get("metadata_json"))?;
            let category: BenchmarkCategory = row.get::<String, _>("category").parse()
                .unwrap_or(BenchmarkCategory::RuleEvaluation);
            let unit: MetricUnit = row.get::<String, _>("unit").parse()
                .unwrap_or(MetricUnit::Nanoseconds);

            Ok(StoredBenchmark {
                id: row.get("id"),
                name: row.get("name"),
                category,
                value,
                unit,
                timestamp: DateTime::parse_from_rfc3339(&row.get::<String, _>("timestamp"))?.with_timezone(&Utc),
                commit: row.get("commit"),
                branch: row.get("branch"),
                metadata,
                config_hash: row.get("config_hash"),
            })
        }
    }

/// Errors for storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("Benchmark not found: {0}")]
    NotFound(String),
    #[error("Migration error: {0}")]
    Migration(String),
}

/// Implement Display for MetricUnit
impl std::fmt::Display for MetricUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricUnit::Nanoseconds => write!(f, "ns"),
            MetricUnit::Microseconds => write!(f, "µs"),
            MetricUnit::Milliseconds => write!(f, "ms"),
            MetricUnit::Seconds => write!(f, "s"),
            MetricUnit::Bytes => write!(f, "B"),
            MetricUnit::Kilobytes => write!(f, "KB"),
            MetricUnit::Megabytes => write!(f, "MB"),
            MetricUnit::OpsPerSecond => write!(f, "ops/s"),
            MetricUnit::RequestsPerSecond => write!(f, "req/s"),
            MetricUnit::Count => write!(f, "count"),
            MetricUnit::Percentage => write!(f, "%"),
            MetricUnit::Custom => write!(f, "custom"),
        }
    }
}

/// Implement FromStr for MetricUnit
impl std::str::FromStr for MetricUnit {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ns" | "nanoseconds" => Ok(MetricUnit::Nanoseconds),
            "µs" | "us" | "microseconds" => Ok(MetricUnit::Microseconds),
            "ms" | "milliseconds" => Ok(MetricUnit::Milliseconds),
            "s" | "seconds" => Ok(MetricUnit::Seconds),
            "B" | "bytes" => Ok(MetricUnit::Bytes),
            "KB" | "kilobytes" => Ok(MetricUnit::Kilobytes),
            "MB" | "megabytes" => Ok(MetricUnit::Megabytes),
            "ops/s" | "ops_per_second" => Ok(MetricUnit::OpsPerSecond),
            "req/s" | "requests_per_second" => Ok(MetricUnit::RequestsPerSecond),
            "count" => Ok(MetricUnit::Count),
            "%" | "percentage" => Ok(MetricUnit::Percentage),
            "custom" => Ok(MetricUnit::Custom),
            _ => Err(format!("Unknown unit: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BenchmarkMetric, MetricValue, MetricUnit, BenchmarkCategory, Percentiles};
    use chrono::Utc;
    
    #[tokio::test]
    async fn test_storage_creation() {
        let storage = BenchmarkStorage::new(":memory:").await;
        assert!(storage.is_ok());
    }
    
    #[tokio::test]
    async fn test_store_and_retrieve() {
        let storage = BenchmarkStorage::new(":memory:").await.unwrap();
        
        let metric = BenchmarkMetric::new(
            "test_metric",
            BenchmarkCategory::Detector,
            MetricValue {
                mean: 100.0,
                median: 100.0,
                std_dev: 5.0,
                min: 90.0,
                max: 110.0,
                sample_count: 10,
                percentiles: Percentiles { p50: 100.0, p90: 105.0, p95: 108.0, p99: 110.0 },
            },
            MetricUnit::Milliseconds,
        );
        
        let id = storage.store(&metric).await.unwrap();
        assert!(id > 0);
        
        let history = storage.get_history("test_metric").await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].name, "test_metric");
        assert_eq!(history[0].value.mean, 100.0);
    }
}