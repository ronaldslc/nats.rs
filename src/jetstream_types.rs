#![allow(missing_docs)]
#![allow(unused)]

use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};

use chrono::{DateTime as ChronoDateTime, Utc};

/// A UTC time
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DateTime(pub ChronoDateTime<Utc>);

impl Default for DateTime {
    fn default() -> DateTime {
        DateTime(UNIX_EPOCH.into())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateConsumerRequest {
    pub stream_name: String,
    pub config: ConsumerConfig,
}

/// Configuration for consumers
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ConsumerConfig {
    pub durable_name: Option<String>,
    pub deliver_subject: Option<String>,
    pub deliver_policy: DeliverPolicy,
    pub opt_start_seq: Option<i64>,
    pub opt_start_time: Option<DateTime>,
    pub ack_policy: AckPolicy,
    pub ack_wait: Option<isize>,
    pub max_deliver: Option<i64>,
    pub filter_subject: Option<String>,
    pub replay_policy: ReplayPolicy,
    pub rate_limit: Option<i64>,
    pub sample_frequency: Option<String>,
    pub max_waiting: Option<i64>,
    pub max_ack_pending: Option<i64>,
}

impl From<&str> for ConsumerConfig {
    fn from(s: &str) -> ConsumerConfig {
        ConsumerConfig {
            durable_name: Some(s.to_string()),
            ..Default::default()
        }
    }
}

/// StreamConfig will determine the properties for a stream.
/// There are sensible defaults for most. If no subjects are
/// given the name will be used as the only subject.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StreamConfig {
    pub subjects: Option<Vec<String>>,
    pub name: String,
    pub retention: RetentionPolicy,
    pub max_consumers: isize,
    pub max_msgs: i64,
    pub max_bytes: i64,
    pub discard: DiscardPolicy,
    pub max_age: isize,
    pub max_msg_size: Option<i32>,
    pub storage: StorageType,
    pub num_replicas: usize,
    pub no_ack: Option<bool>,
    pub template_owner: Option<String>,
    pub duplicate_window: Option<isize>,
}

impl From<&str> for StreamConfig {
    fn from(s: &str) -> StreamConfig {
        StreamConfig {
            name: s.to_string(),
            ..Default::default()
        }
    }
}

/// StreamInfo shows config and current state for this stream.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StreamInfo {
    pub r#type: String,
    pub config: StreamConfig,
    pub created: DateTime,
    pub state: StreamState,
}

// StreamStats is information about the given stream.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StreamState {
    #[serde(default)]
    pub msgs: u64,
    pub bytes: u64,
    pub first_seq: u64,
    pub first_ts: String,
    pub last_seq: u64,
    pub last_ts: DateTime,
    pub consumer_count: usize,
}

// DeliverPolicy determines how the consumer should select the first message to deliver.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum DeliverPolicy {
    // DeliverAllPolicy will be the default so can be omitted from the request.
    #[serde(rename = "all")]
    DeliverAllPolicy = 0,
    // DeliverLastPolicy will start the consumer with the last sequence received.
    #[serde(rename = "last")]
    DeliverLastPolicy = 1,
    // DeliverNewPolicy will only deliver new messages that are sent
    // after the consumer is created.
    #[serde(rename = "new")]
    DeliverNewPolicy = 2,
    // DeliverByStartSequencePolicy will look for a defined starting sequence to start.
    #[serde(rename = "by_start_sequence")]
    DeliverByStartSequencePolicy = 3,
    // StartTime will select the first messsage with a timestamp >= to StartTime.
    #[serde(rename = "by_start_time")]
    DeliverByStartTimePolicy = 4,
}

impl Default for DeliverPolicy {
    fn default() -> DeliverPolicy {
        DeliverPolicy::DeliverAllPolicy
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum AckPolicy {
    #[serde(rename = "none")]
    AckNone = 0,
    #[serde(rename = "all")]
    AckAll = 1,
    #[serde(rename = "explicit")]
    AckExplicit = 2,
    // For setting
    AckPolicyNotSet = 99,
}

impl Default for AckPolicy {
    fn default() -> AckPolicy {
        AckPolicy::AckExplicit
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum ReplayPolicy {
    #[serde(rename = "instant")]
    ReplayInstant = 0,
    #[serde(rename = "original")]
    ReplayOriginal = 1,
}

impl Default for ReplayPolicy {
    fn default() -> ReplayPolicy {
        ReplayPolicy::ReplayInstant
    }
}

// RetentionPolicy determines how messages in a set are retained.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum RetentionPolicy {
    // LimitsPolicy (default) means that messages are retained until any given limit is reached.
    // This could be one of MaxMsgs, MaxBytes, or MaxAge.
    #[serde(rename = "limits")]
    LimitsPolicy = 0,
    // InterestPolicy specifies that when all known observables have acknowledged a message it can be removed.
    #[serde(rename = "interest")]
    InterestPolicy = 1,
    // WorkQueuePolicy specifies that when the first worker or subscriber acknowledges the message it can be removed.
    #[serde(rename = "workqueue")]
    WorkQueuePolicy = 2,
}

impl Default for RetentionPolicy {
    fn default() -> RetentionPolicy {
        RetentionPolicy::LimitsPolicy
    }
}

// Discard Policy determines how we proceed when limits of messages or bytes are hit. The default, DicscardOld will
// remove older messages. DiscardNew will fail to store the new message.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum DiscardPolicy {
    // DiscardOld will remove older messages to return to the limits.
    #[serde(rename = "old")]
    DiscardOld = 0,
    //DiscardNew will error on a StoreMsg call
    #[serde(rename = "new")]
    DiscardNew = 1,
}

impl Default for DiscardPolicy {
    fn default() -> DiscardPolicy {
        DiscardPolicy::DiscardOld
    }
}

// StorageType determines how messages are stored for retention.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum StorageType {
    // FileStorage specifies on disk storage. It's the default.
    #[serde(rename = "file")]
    FileStorage = 0,
    // MemoryStorage specifies in memory only.
    #[serde(rename = "memory")]
    MemoryStorage = 1,
}

impl Default for StorageType {
    fn default() -> StorageType {
        StorageType::FileStorage
    }
}

// AccountLimits is for the information about
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct AccountLimits {
    pub max_memory: i64,
    pub max_storage: i64,
    pub max_streams: i64,
    pub max_consumers: i64,
}

// AccountStats returns current statistics about the account's JetStream usage.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct AccountStats {
    pub memory: u64,
    pub storage: u64,
    pub streams: usize,
    pub limits: AccountLimits,
}

///
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PubAck {
    pub stream: String,
    pub seq: u64,
    pub duplicate: Option<bool>,
}

///
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ConsumerInfo {
    pub r#type: String,
    pub stream_name: String,
    pub name: String,
    pub created: DateTime,
    pub config: ConsumerConfig,
    pub delivered: SequencePair,
    pub ack_floor: SequencePair,
    pub num_ack_pending: usize,
    pub num_redelivered: usize,
    pub num_waiting: usize,
    pub num_pending: u64,
    pub cluster: ClusterInfo,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ClusterInfo {
    pub leader: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct SequencePair {
    pub consumer_seq: u64,
    pub stream_seq: u64,
}

// NextRequest is for getting next messages for pull based consumers.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct NextRequest {
    pub expires: DateTime,
    pub batch: Option<usize>,
    pub no_wait: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StreamRequest {
    pub subject: Option<String>,
}

///
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SubOpts {
    // For attaching.
    pub stream: String,
    pub consumer: String,
    // For pull based consumers, batch size for pull
    pub pull: usize,
    // For manual ack
    pub mack: bool,
    // For creating or updating.
    pub cfg: ConsumerConfig,
}

///
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PubOpts {
    pub ttl: isize,
    pub id: String,
    // Expected last msgId
    pub lid: String,
    // Expected stream name
    pub str: String,
    // Expected last sequence
    pub seq: u64,
}

/// AccountInfo contains info about the JetStream usage from the current account.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    pub r#type: String,
    pub memory: i64,
    pub storage: i64,
    pub streams: i64,
    pub consumers: i64,
    pub api: ApiStats,
    pub limits: AccountLimits,
}

/// ApiStats reports on API calls to JetStream for this account.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct ApiStats {
    pub total: u64,
    pub errors: u64,
}
