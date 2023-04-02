use iso8601_timestamp::Timestamp;
use serde::Deserialize;
use std::collections::HashMap;

/// Index access information
#[derive(Deserialize, Debug, Clone)]
pub struct IndexAccess {
    /// Operations since timestamp
    pub ops: i32,
    /// Timestamp at which data keeping begun
    pub since: Timestamp,
}

/// Collection index
#[derive(Deserialize, Debug, Clone)]
pub struct Index {
    /// Index name
    pub name: String,
    /// Access information
    pub accesses: IndexAccess,
}

/// Histogram entry
#[derive(Deserialize, Debug, Clone)]
pub struct LatencyHistogramEntry {
    /// Time
    pub micros: i64,
    /// Count
    pub count: i64,
}

/// Collection latency stats
#[derive(Deserialize, Debug, Clone)]
pub struct LatencyStats {
    /// Total operations
    pub ops: i64,
    /// Timestamp at which data keeping begun
    pub latency: i64,
    /// Histogram representation of latency data
    pub histogram: Vec<LatencyHistogramEntry>,
}

/// Collection storage stats
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StorageStats {
    /// Uncompressed data size
    pub size: i64,
    /// Data size on disk
    pub storage_size: i64,
    /// Total size of all indexes
    pub total_index_size: i64,
    /// Sum of storage size and total index size
    pub total_size: i64,
    /// Individual index sizes
    pub index_sizes: HashMap<String, i64>,
    /// Number of documents in collection
    pub count: i64,
    /// Average size of each document
    pub avg_obj_size: i64,
}

/// Query collection scan stats
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionScans {
    /// Number of total collection scans
    pub total: i64,
    /// Number of total collection scans not using a tailable cursor
    pub non_tailable: i64,
}

/// Collection query execution stats
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryExecStats {
    /// Stats regarding collection scans
    pub collection_scans: CollectionScans,
}

/// Collection stats
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionStats {
    /// Namespace
    pub ns: String,
    /// Local time
    pub local_time: Timestamp,
    /// Latency stats
    pub latency_stats: HashMap<String, LatencyStats>,
    /// Query exec stats
    pub query_exec_stats: QueryExecStats,
    /// Number of documents in collection
    pub count: u64,
}

/// Server Stats
#[derive(Deserialize, Debug, Clone)]
pub struct Stats {
    /// Index usage information
    pub indices: HashMap<String, Vec<Index>>,
    /// Collection stats
    pub coll_stats: HashMap<String, CollectionStats>,
}
