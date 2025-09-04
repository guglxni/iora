//! Historical Data Management Module (Task 2.2.3)
//!
//! This module implements efficient historical data management for I.O.R.A. that provides:
//! - Efficient historical data fetching and storage
//! - Data deduplication and compression algorithms
//! - Historical data validation and gap filling
//! - Time-series optimization for RAG training

use crate::modules::fetcher::{
    ApiProvider, ApiError, MultiApiClient
};

use chrono::{DateTime, Utc, Duration, NaiveDate};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;
use std::fs;

/// Time series data point for optimized storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub source: ApiProvider,
    pub quality_score: Option<f64>,
}

/// Compressed time series data block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedBlock {
    pub symbol: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub interval_seconds: i64,
    pub data_points: Vec<CompressedPoint>,
    pub compression_ratio: f64,
    pub checksum: u64,
}

/// Compressed data point using delta encoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedPoint {
    pub timestamp_offset: i64,  // seconds from block start
    pub open: i32,             // delta from previous close * 10000
    pub high: i32,             // delta from open * 10000
    pub low: i32,              // delta from open * 10000
    pub close: i32,            // delta from open * 10000
    pub volume: i32,           // compressed volume
}

/// Historical data metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMetadata {
    pub symbol: String,
    pub data_range: DateRange,
    pub total_points: usize,
    pub compressed_blocks: usize,
    pub last_updated: DateTime<Utc>,
    pub sources: HashSet<ApiProvider>,
    pub quality_metrics: QualityMetrics,
    pub gaps_filled: usize,
    pub deduplication_stats: DeduplicationStats,
}

/// Date range for data queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Quality metrics for historical data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub completeness_score: f64,      // 0.0 to 1.0
    pub consistency_score: f64,       // 0.0 to 1.0
    pub accuracy_score: f64,          // 0.0 to 1.0
    pub gap_percentage: f64,          // percentage of missing data
    pub outlier_percentage: f64,      // percentage of outlier data points
}

/// Deduplication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationStats {
    pub original_points: usize,
    pub unique_points: usize,
    pub duplicates_removed: usize,
    pub deduplication_ratio: f64,
}

/// Gap filling configuration
#[derive(Debug, Clone)]
pub struct GapFillingConfig {
    pub max_gap_size: Duration,        // Maximum gap size to fill
    pub interpolation_method: InterpolationMethod,
    pub min_data_points: usize,       // Minimum points needed for interpolation
    pub outlier_threshold: f64,       // Outlier detection threshold
}

/// Interpolation methods for gap filling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterpolationMethod {
    Linear,
    CubicSpline,
    LastKnownValue,
    AverageValue,
}

/// Time series optimization configuration
#[derive(Debug, Clone)]
pub struct TimeSeriesConfig {
    pub compression_enabled: bool,
    pub compression_threshold: usize,     // Points per block
    pub deduplication_enabled: bool,
    pub gap_filling_enabled: bool,
    pub validation_enabled: bool,
    pub storage_path: PathBuf,
    pub max_memory_cache: usize,          // Max points in memory
    pub prefetch_window: Duration,        // How far ahead to prefetch
}

impl Default for TimeSeriesConfig {
    fn default() -> Self {
        Self {
            compression_enabled: true,
            compression_threshold: 1000,
            deduplication_enabled: true,
            gap_filling_enabled: true,
            validation_enabled: true,
            storage_path: PathBuf::from("./data/historical"),
            max_memory_cache: 10000,
            prefetch_window: Duration::days(7),
        }
    }
}

/// Historical data manager
pub struct HistoricalDataManager {
    /// Time series configuration
    config: TimeSeriesConfig,
    /// In-memory cache for frequently accessed data
    memory_cache: Arc<RwLock<HashMap<String, Vec<TimeSeriesPoint>>>>,
    /// Compressed data blocks storage
    compressed_blocks: Arc<RwLock<HashMap<String, Vec<CompressedBlock>>>>,
    /// Metadata for all symbols
    metadata: Arc<RwLock<HashMap<String, HistoricalMetadata>>>,
    /// Gap filling configuration
    gap_config: GapFillingConfig,
    /// LRU cache for access tracking
    access_order: Arc<RwLock<VecDeque<String>>>,
    /// Storage statistics
    stats: Arc<RwLock<StorageStats>>,
}

/// Storage statistics
#[derive(Debug, Clone, Default)]
pub struct StorageStats {
    pub total_symbols: usize,
    pub total_points: usize,
    pub compressed_size: usize,
    pub uncompressed_size: usize,
    pub compression_ratio: f64,
    pub cache_hit_rate: f64,
    pub average_query_time: Duration,
}

impl HistoricalDataManager {
    /// Create a new historical data manager
    pub fn new(config: TimeSeriesConfig) -> Self {
        let gap_config = GapFillingConfig {
            max_gap_size: Duration::hours(4),
            interpolation_method: InterpolationMethod::Linear,
            min_data_points: 2,
            outlier_threshold: 3.0,
        };

        Self {
            config,
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            compressed_blocks: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            gap_config,
            access_order: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(StorageStats::default())),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        let config = TimeSeriesConfig {
            compression_enabled: true,
            compression_threshold: 1000,
            deduplication_enabled: true,
            gap_filling_enabled: true,
            validation_enabled: true,
            storage_path: PathBuf::from("./data/historical"),
            max_memory_cache: 10000,
            prefetch_window: Duration::days(7),
        };
        Self::new(config)
    }

    /// Fetch and store historical data for a symbol
    pub async fn fetch_and_store_historical(
        &self,
        client: &MultiApiClient,
        symbol: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        interval: &str,
    ) -> Result<(), ApiError> {
        println!("📈 Fetching historical data for {} from {} to {} (interval: {})",
                 symbol, start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"), interval);

        // Calculate number of days for the request
        let days = ((end_date - start_date).num_days() + 1) as u32;

        // Fetch data using the client's intelligent method
        match client.get_historical_data_intelligent(symbol, days).await {
            Ok(data) => {
                println!("📊 Received {} historical data points from APIs", data.len());
                // Convert HistoricalData to TimeSeriesPoint
                let all_data: Vec<TimeSeriesPoint> = data.into_iter().map(|item| {
                    TimeSeriesPoint {
                        timestamp: item.timestamp,
                        open: item.open,
                        high: item.high,
                        low: item.low,
                        close: item.close,
                        volume: item.volume.unwrap_or(0.0),
                        source: ApiProvider::CoinGecko, // This would be dynamic in a real implementation
                        quality_score: Some(0.9),
                    }
                }).collect();

                if all_data.is_empty() {
                    return Err(ApiError::Unknown("No historical data available".to_string()));
                }

                // Process the data
                let processed_data = self.process_historical_data(all_data, symbol).await?;

                // Store the processed data
                let data_len = processed_data.len();
                self.store_processed_data(symbol, processed_data).await?;

                println!("✅ Successfully stored {} data points for {}", data_len, symbol);
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to fetch historical data: {}", e);
                Err(e)
            }
        }
    }



    /// Process historical data with deduplication, validation, and gap filling
    async fn process_historical_data(
        &self,
        mut data: Vec<TimeSeriesPoint>,
        symbol: &str,
    ) -> Result<Vec<TimeSeriesPoint>, ApiError> {
        println!("🔄 Processing {} raw data points for {}", data.len(), symbol);

        // Sort by timestamp
        data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Remove duplicates
        let deduped_data = if self.config.deduplication_enabled {
            self.deduplicate_data(data)?
        } else {
            data
        };

        println!("🧹 After deduplication: {} points", deduped_data.len());

        // Validate data
        let validated_data = if self.config.validation_enabled {
            self.validate_historical_data(deduped_data).await?
        } else {
            deduped_data
        };

        // Fill gaps
        let filled_data = if self.config.gap_filling_enabled {
            self.fill_gaps(validated_data).await?
        } else {
            validated_data
        };

        println!("✅ Final processed data: {} points", filled_data.len());
        Ok(filled_data)
    }

    /// Deduplicate data based on timestamp
    fn deduplicate_data(&self, data: Vec<TimeSeriesPoint>) -> Result<Vec<TimeSeriesPoint>, ApiError> {
        let mut seen_timestamps = HashSet::new();
        let mut deduped = Vec::new();
        let mut duplicates = 0;

        for point in data {
            if seen_timestamps.insert(point.timestamp) {
                deduped.push(point);
            } else {
                duplicates += 1;
            }
        }

        println!("🗑️  Removed {} duplicate entries", duplicates);
        Ok(deduped)
    }

    /// Validate historical data quality
    async fn validate_historical_data(&self, data: Vec<TimeSeriesPoint>) -> Result<Vec<TimeSeriesPoint>, ApiError> {
        let mut validated = Vec::new();
        let mut outliers = 0;

        for mut point in data {
            // Basic validation
            if self.is_valid_data_point(&point) {
                // Outlier detection (simplified)
                if self.is_outlier(&point, &validated) {
                    outliers += 1;
                    // Could mark as outlier or remove
                    point.quality_score = Some(0.5); // Lower quality for outliers
                }
                validated.push(point);
            }
        }

        println!("🔍 Validation complete: {} outliers detected", outliers);
        Ok(validated)
    }

    /// Check if data point is valid
    fn is_valid_data_point(&self, point: &TimeSeriesPoint) -> bool {
        point.open > 0.0 &&
        point.high >= point.open &&
        point.low <= point.open &&
        point.close > 0.0 &&
        point.volume >= 0.0 &&
        point.timestamp >= DateTime::<Utc>::from_naive_utc_and_offset(NaiveDate::from_ymd_opt(2009, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(), Utc) // After Bitcoin genesis
    }

    /// Simple outlier detection based on price movement
    fn is_outlier(&self, point: &TimeSeriesPoint, historical: &[TimeSeriesPoint]) -> bool {
        if historical.len() < 5 {
            return false;
        }

        // Calculate average price change in recent data
        let recent_prices: Vec<f64> = historical.iter()
            .rev()
            .take(10)
            .map(|p| p.close)
            .collect();

        if recent_prices.len() < 2 {
            return false;
        }

        let avg_change = recent_prices.windows(2)
            .map(|w| (w[1] - w[0]).abs() / w[0])
            .sum::<f64>() / (recent_prices.len() - 1) as f64;

        // Check if current price change is an outlier
        let prev_close = recent_prices[0];
        let current_change = (point.close - prev_close).abs() / prev_close;

        current_change > avg_change * self.gap_config.outlier_threshold
    }

    /// Fill gaps in historical data
    async fn fill_gaps(&self, data: Vec<TimeSeriesPoint>) -> Result<Vec<TimeSeriesPoint>, ApiError> {
        if data.len() < 2 {
            return Ok(data);
        }

        let mut filled_data = Vec::new();
        let mut gaps_filled = 0;

        for i in 0..data.len() - 1 {
            filled_data.push(data[i].clone());

            let gap_duration = data[i + 1].timestamp - data[i].timestamp;

            // Check if gap needs filling
            if gap_duration > self.gap_config.max_gap_size {
                // Calculate number of points to fill
                let interval_seconds = 3600; // Assume hourly data
                let gaps_to_fill = (gap_duration.num_seconds() / interval_seconds) - 1;

                if gaps_to_fill > 0 && gaps_to_fill <= 24 { // Max 24 hours of gaps
                    for j in 1..=gaps_to_fill {
                        let interpolated_point = self.interpolate_point(
                            &data[i],
                            &data[i + 1],
                            j as f64 / (gaps_to_fill + 1) as f64,
                        );
                        filled_data.push(interpolated_point);
                        gaps_filled += 1;
                    }
                }
            }
        }

        filled_data.push(data.last().unwrap().clone());

        println!("🔧 Filled {} gaps in data", gaps_filled);
        Ok(filled_data)
    }

    /// Interpolate a data point between two points
    fn interpolate_point(&self, start: &TimeSeriesPoint, end: &TimeSeriesPoint, ratio: f64) -> TimeSeriesPoint {
        TimeSeriesPoint {
            timestamp: start.timestamp + Duration::seconds(
                ((end.timestamp - start.timestamp).num_seconds() as f64 * ratio) as i64
            ),
            open: start.close + (end.open - start.close) * ratio,
            high: start.high + (end.high - start.high) * ratio,
            low: start.low + (end.low - start.low) * ratio,
            close: start.close + (end.close - start.close) * ratio,
            volume: start.volume + (end.volume - start.volume) * ratio,
            source: ApiProvider::CoinGecko, // Interpolated data
            quality_score: Some(0.7), // Lower quality for interpolated data
        }
    }

    /// Store processed data with compression
    async fn store_processed_data(&self, symbol: &str, data: Vec<TimeSeriesPoint>) -> Result<(), ApiError> {
        // Ensure storage directory exists
        fs::create_dir_all(&self.config.storage_path)
            .map_err(|e| ApiError::Unknown(format!("Failed to create storage directory: {}", e)))?;

        // Compress data if enabled
        if self.config.compression_enabled && data.len() >= self.config.compression_threshold {
            let compressed = self.compress_time_series(&data)?;
            self.store_compressed_blocks(symbol, compressed).await?;
        } else {
            // Store uncompressed in memory cache
            let data_clone = data.clone();
            self.memory_cache.write().await.insert(symbol.to_string(), data);
            // Update metadata
            self.update_metadata(symbol, &data_clone).await?;
        }

        // Update storage statistics
        self.update_storage_stats().await?;

        Ok(())
    }

    /// Compress time series data
    fn compress_time_series(&self, data: &[TimeSeriesPoint]) -> Result<Vec<CompressedBlock>, ApiError> {
        let mut blocks = Vec::new();

        for chunk in data.chunks(self.config.compression_threshold) {
            if chunk.is_empty() {
                continue;
            }

            let block = self.compress_block(chunk)?;
            blocks.push(block);
        }

        Ok(blocks)
    }

    /// Compress a block of time series data
    fn compress_block(&self, data: &[TimeSeriesPoint]) -> Result<CompressedBlock, ApiError> {
        if data.is_empty() {
            return Err(ApiError::Unknown("Empty data block".to_string()));
        }

        let start_time = data[0].timestamp;
        let end_time = data.last().unwrap().timestamp;
        let interval_seconds = if data.len() > 1 {
            (data[1].timestamp - data[0].timestamp).num_seconds()
        } else {
            3600 // Default to 1 hour
        };

        let mut compressed_points = Vec::new();
        let mut prev_close = data[0].open;

        for point in data {
            let timestamp_offset = (point.timestamp - start_time).num_seconds();

            // Delta encoding
            let open_delta = ((point.open - prev_close) * 10000.0) as i32;
            let high_delta = ((point.high - point.open) * 10000.0) as i32;
            let low_delta = ((point.low - point.open) * 10000.0) as i32;
            let close_delta = ((point.close - point.open) * 10000.0) as i32;

            // Simple volume compression (log scale)
            let volume_compressed = if point.volume > 0.0 {
                (point.volume.log10() * 1000.0) as i32
            } else {
                0
            };

            compressed_points.push(CompressedPoint {
                timestamp_offset,
                open: open_delta,
                high: high_delta,
                low: low_delta,
                close: close_delta,
                volume: volume_compressed,
            });

            prev_close = point.close;
        }

        // Calculate compression ratio
        let original_size = std::mem::size_of::<TimeSeriesPoint>() * data.len();
        let compressed_size = std::mem::size_of::<CompressedPoint>() * compressed_points.len();
        let compression_ratio = original_size as f64 / compressed_size as f64;

        // Simple checksum
        let checksum = compressed_points.iter()
            .map(|p| p.timestamp_offset as u64)
            .sum();

        Ok(CompressedBlock {
            symbol: data[0].source.to_string(),
            start_time,
            end_time,
            interval_seconds,
            data_points: compressed_points,
            compression_ratio,
            checksum,
        })
    }

    /// Store compressed blocks
    async fn store_compressed_blocks(&self, symbol: &str, blocks: Vec<CompressedBlock>) -> Result<(), ApiError> {
        self.compressed_blocks.write().await.insert(symbol.to_string(), blocks);
        Ok(())
    }

    /// Update metadata for a symbol
    async fn update_metadata(&self, symbol: &str, data: &[TimeSeriesPoint]) -> Result<(), ApiError> {
        let data_range = if !data.is_empty() {
            DateRange {
                start: data[0].timestamp,
                end: data.last().unwrap().timestamp,
            }
        } else {
            DateRange {
                start: Utc::now(),
                end: Utc::now(),
            }
        };

        let sources: HashSet<ApiProvider> = data.iter()
            .map(|p| p.source)
            .collect();

        let quality_metrics = self.calculate_quality_metrics(data);

        let dedup_stats = DeduplicationStats {
            original_points: data.len(),
            unique_points: data.len(),
            duplicates_removed: 0,
            deduplication_ratio: 1.0,
        };

        let metadata = HistoricalMetadata {
            symbol: symbol.to_string(),
            data_range,
            total_points: data.len(),
            compressed_blocks: 0, // Will be updated when compression is implemented
            last_updated: Utc::now(),
            sources,
            quality_metrics,
            gaps_filled: 0, // Will be tracked during gap filling
            deduplication_stats: dedup_stats,
        };

        self.metadata.write().await.insert(symbol.to_string(), metadata);
        Ok(())
    }

    /// Calculate quality metrics for data
    fn calculate_quality_metrics(&self, data: &[TimeSeriesPoint]) -> QualityMetrics {
        if data.is_empty() {
            return QualityMetrics {
                completeness_score: 0.0,
                consistency_score: 0.0,
                accuracy_score: 0.0,
                gap_percentage: 1.0,
                outlier_percentage: 0.0,
            };
        }

        // Calculate completeness (inverse of gaps)
        let expected_intervals = if data.len() > 1 {
            ((data.last().unwrap().timestamp - data[0].timestamp).num_hours()) as usize
        } else {
            1
        };
        let completeness_score = (data.len() as f64 / expected_intervals as f64).min(1.0);

        // Calculate consistency (price movement reasonableness)
        let price_changes: Vec<f64> = data.windows(2)
            .map(|w| (w[1].close - w[0].close).abs() / w[0].close)
            .collect();
        let avg_change = price_changes.iter().sum::<f64>() / price_changes.len() as f64;
        let consistency_score = (1.0 / (1.0 + avg_change)).min(1.0);

        // Calculate accuracy (based on quality scores)
        let avg_quality = data.iter()
            .filter_map(|p| p.quality_score)
            .sum::<f64>() / data.len() as f64;
        let accuracy_score = avg_quality;

        // Calculate gap percentage
        let gap_percentage = 1.0 - completeness_score;

        // Calculate outlier percentage (simplified)
        let outlier_count = data.iter()
            .filter(|p| p.quality_score.unwrap_or(1.0) < 0.8)
            .count();
        let outlier_percentage = outlier_count as f64 / data.len() as f64;

        QualityMetrics {
            completeness_score,
            consistency_score,
            accuracy_score,
            gap_percentage,
            outlier_percentage,
        }
    }

    /// Update storage statistics
    async fn update_storage_stats(&self) -> Result<(), ApiError> {
        let metadata = self.metadata.read().await;
        let compressed_blocks = self.compressed_blocks.read().await;
        let _memory_cache = self.memory_cache.read().await;

        let total_symbols = metadata.len();
        let total_points = metadata.values().map(|m| m.total_points).sum::<usize>();
        let compressed_blocks_count = compressed_blocks.values().map(|v| v.len()).sum::<usize>();

        // Estimate sizes
        let compressed_size = compressed_blocks_count * 1000; // Rough estimate
        let uncompressed_size = total_points * std::mem::size_of::<TimeSeriesPoint>();
        let compression_ratio = if compressed_size > 0 {
            uncompressed_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        let mut stats = self.stats.write().await;
        stats.total_symbols = total_symbols;
        stats.total_points = total_points;
        stats.compressed_size = compressed_size;
        stats.uncompressed_size = uncompressed_size;
        stats.compression_ratio = compression_ratio;

        Ok(())
    }

    /// Query historical data with time series optimization
    pub async fn query_historical_data(
        &self,
        symbol: &str,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<TimeSeriesPoint>, ApiError> {
        // Check memory cache first
        if let Some(cached_data) = self.memory_cache.read().await.get(symbol) {
            let filtered = self.filter_data(cached_data, start_date, end_date, limit);
            if !filtered.is_empty() {
                return Ok(filtered);
            }
        }

        // Check compressed blocks
        if let Some(blocks) = self.compressed_blocks.read().await.get(symbol) {
            let mut all_data = Vec::new();
            for block in blocks {
                let decompressed = self.decompress_block(block)?;
                all_data.extend(decompressed);
            }

            all_data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            let filtered = self.filter_data(&all_data, start_date, end_date, limit);
            return Ok(filtered);
        }

        Err(ApiError::Unknown(format!("No historical data found for {}", symbol)))
    }

    /// Filter data by date range and limit
    fn filter_data(
        &self,
        data: &[TimeSeriesPoint],
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        limit: Option<usize>,
    ) -> Vec<TimeSeriesPoint> {
        let mut filtered: Vec<TimeSeriesPoint> = data.iter()
            .filter(|point| {
                let start_ok = start_date.map_or(true, |start| point.timestamp >= start);
                let end_ok = end_date.map_or(true, |end| point.timestamp <= end);
                start_ok && end_ok
            })
            .cloned()
            .collect();

        // Apply limit if specified
        if let Some(limit) = limit {
            filtered.truncate(limit);
        }

        filtered
    }

    /// Decompress a block of data
    fn decompress_block(&self, block: &CompressedBlock) -> Result<Vec<TimeSeriesPoint>, ApiError> {
        let mut points = Vec::new();
        let mut prev_close = 0.0;

        for compressed_point in &block.data_points {
            let timestamp = block.start_time + Duration::seconds(compressed_point.timestamp_offset);

            // Reverse delta encoding
            let open = prev_close + (compressed_point.open as f64 / 10000.0);
            let high = open + (compressed_point.high as f64 / 10000.0);
            let low = open + (compressed_point.low as f64 / 10000.0);
            let close = open + (compressed_point.close as f64 / 10000.0);

            // Reverse volume compression
            let volume = if compressed_point.volume > 0 {
                10_f64.powf(compressed_point.volume as f64 / 1000.0)
            } else {
                0.0
            };

            points.push(TimeSeriesPoint {
                timestamp,
                open,
                high,
                low,
                close,
                volume,
                source: ApiProvider::CoinGecko, // Would need to store this in compressed format
                quality_score: Some(0.8), // Default quality for decompressed data
            });

            prev_close = close;
        }

        Ok(points)
    }

    /// Get metadata for a symbol
    pub async fn get_metadata(&self, symbol: &str) -> Option<HistoricalMetadata> {
        self.metadata.read().await.get(symbol).cloned()
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> StorageStats {
        self.stats.read().await.clone()
    }

    /// Optimize data for RAG training
    pub async fn optimize_for_rag(&self, symbol: &str) -> Result<Vec<String>, ApiError> {
        let data = self.query_historical_data(symbol, None, None, Some(1000)).await?;

        if data.is_empty() {
            return Ok(vec![]);
        }

        // Generate time series patterns and insights
        let mut insights = Vec::new();

        // Trend analysis
        if let Some(trend) = self.analyze_trend(&data) {
            insights.push(trend);
        }

        // Volatility analysis
        if let Some(volatility) = self.analyze_volatility(&data) {
            insights.push(volatility);
        }

        // Volume analysis
        if let Some(volume_pattern) = self.analyze_volume(&data) {
            insights.push(volume_pattern);
        }

        // Support/resistance levels
        if let Some(levels) = self.analyze_support_resistance(&data) {
            insights.push(levels);
        }

        Ok(insights)
    }

    /// Analyze price trend
    fn analyze_trend(&self, data: &[TimeSeriesPoint]) -> Option<String> {
        if data.len() < 10 {
            return None;
        }

        let start_price = data[0].close;
        let end_price = data.last().unwrap().close;
        let change_percent = ((end_price - start_price) / start_price) * 100.0;

        let direction = if change_percent > 5.0 {
            "upward"
        } else if change_percent < -5.0 {
            "downward"
        } else {
            "sideways"
        };

        Some(format!("Price trend analysis: {:.2}% {} movement over {} data points",
                    change_percent.abs(), direction, data.len()))
    }

    /// Analyze volatility
    fn analyze_volatility(&self, data: &[TimeSeriesPoint]) -> Option<String> {
        if data.len() < 5 {
            return None;
        }

        let returns: Vec<f64> = data.windows(2)
            .map(|w| (w[1].close - w[0].close) / w[0].close)
            .collect();

        let avg_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter()
            .map(|r| (r - avg_return).powi(2))
            .sum::<f64>() / returns.len() as f64;
        let volatility = variance.sqrt() * 100.0; // As percentage

        let volatility_level = if volatility > 10.0 {
            "high"
        } else if volatility > 5.0 {
            "moderate"
        } else {
            "low"
        };

        Some(format!("Volatility analysis: {:.2}% {} volatility based on {} price movements",
                    volatility, volatility_level, returns.len()))
    }

    /// Analyze volume patterns
    fn analyze_volume(&self, data: &[TimeSeriesPoint]) -> Option<String> {
        if data.is_empty() {
            return None;
        }

        let avg_volume = data.iter().map(|p| p.volume).sum::<f64>() / data.len() as f64;
        let max_volume = data.iter().map(|p| p.volume).fold(0.0, f64::max);

        let volume_trend = if max_volume > avg_volume * 2.0 {
            "with significant volume spikes"
        } else {
            "with consistent volume"
        };

        Some(format!("Volume analysis: Average volume {:.0} units {}",
                    avg_volume, volume_trend))
    }

    /// Analyze support and resistance levels
    fn analyze_support_resistance(&self, data: &[TimeSeriesPoint]) -> Option<String> {
        if data.len() < 20 {
            return None;
        }

        // Simple pivot point analysis
        let highs: Vec<f64> = data.iter().map(|p| p.high).collect();
        let lows: Vec<f64> = data.iter().map(|p| p.low).collect();

        let resistance_level = highs.iter().fold(0.0f64, |a, &b| a.max(b));
        let support_level = lows.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        Some(format!("Technical analysis: Resistance at ${:.2}, Support at ${:.2}",
                    resistance_level, support_level))
    }

    /// Health check for the historical data system
    pub async fn health_check(&self) -> bool {
        // Check if we can access all data structures
        let cache_ok = self.memory_cache.try_read().is_ok();
        let blocks_ok = self.compressed_blocks.try_read().is_ok();
        let metadata_ok = self.metadata.try_read().is_ok();
        let stats_ok = self.stats.try_read().is_ok();

        cache_ok && blocks_ok && metadata_ok && stats_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_historical_manager_creation() {
        let manager = HistoricalDataManager::default();
        assert!(manager.health_check().await);
    }

    #[test]
    fn test_time_series_config_defaults() {
        let config = TimeSeriesConfig {
            compression_enabled: true,
            compression_threshold: 1000,
            deduplication_enabled: true,
            gap_filling_enabled: true,
            validation_enabled: true,
            storage_path: PathBuf::from("./data/historical"),
            max_memory_cache: 10000,
            prefetch_window: Duration::days(7),
        };

        assert_eq!(config.compression_threshold, 1000);
        assert_eq!(config.max_memory_cache, 10000);
        assert!(config.compression_enabled);
        assert!(config.deduplication_enabled);
    }

    #[test]
    fn test_data_validation() {
        let manager = HistoricalDataManager::default();

        // Valid data point
        let valid_point = TimeSeriesPoint {
            timestamp: Utc::now(),
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 1000.0,
            source: ApiProvider::CoinGecko,
            quality_score: Some(0.9),
        };

        assert!(manager.is_valid_data_point(&valid_point));

        // Invalid data point (negative price)
        let invalid_point = TimeSeriesPoint {
            timestamp: Utc::now(),
            open: -100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 1000.0,
            source: ApiProvider::CoinGecko,
            quality_score: Some(0.9),
        };

        assert!(!manager.is_valid_data_point(&invalid_point));
    }

    #[test]
    fn test_deduplication() {
        let manager = HistoricalDataManager::default();

        // Use the same timestamp for both points to ensure they're actually duplicates
        let timestamp = Utc::now();

        let point1 = TimeSeriesPoint {
            timestamp: timestamp.clone(),
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 1000.0,
            source: ApiProvider::CoinGecko,
            quality_score: Some(0.9),
        };

        let point2 = TimeSeriesPoint {
            timestamp: timestamp.clone(), // Same timestamp
            open: 101.0,
            high: 111.0,
            low: 91.0,
            close: 106.0,
            volume: 1001.0,
            source: ApiProvider::CoinMarketCap,
            quality_score: Some(0.8),
        };

        let data = vec![point1, point2];
        let deduped = manager.deduplicate_data(data).unwrap();

        // Should have only 1 point after deduplication
        assert_eq!(deduped.len(), 1);
    }

    #[test]
    fn test_interpolation_method_enum() {
        // Test that all interpolation methods can be created
        let linear = InterpolationMethod::Linear;
        let cubic = InterpolationMethod::CubicSpline;
        let last_value = InterpolationMethod::LastKnownValue;
        let average = InterpolationMethod::AverageValue;

        match linear {
            InterpolationMethod::Linear => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_gap_filling_config() {
        let config = GapFillingConfig {
            max_gap_size: Duration::hours(4),
            interpolation_method: InterpolationMethod::Linear,
            min_data_points: 2,
            outlier_threshold: 3.0,
        };

        assert_eq!(config.max_gap_size, Duration::hours(4));
        assert_eq!(config.min_data_points, 2);
        assert_eq!(config.outlier_threshold, 3.0);
    }

    #[test]
    fn test_storage_stats_defaults() {
        let stats = StorageStats::default();
        assert_eq!(stats.total_symbols, 0);
        assert_eq!(stats.total_points, 0);
        assert_eq!(stats.compressed_size, 0);
        assert_eq!(stats.uncompressed_size, 0);
        assert_eq!(stats.compression_ratio, 0.0);
    }
}
