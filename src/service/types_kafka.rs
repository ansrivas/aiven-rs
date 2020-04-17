// MIT License
//
// Copyright (c) 2020 Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::customdeser;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AclDefinition {
	pub id: String,
	pub permission: String,
	pub topic: String,
	pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Acl {
	#[serde(rename = "acl")]
	pub acl: Vec<AclDefinition>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConnectorConfig {
	#[serde(rename = "connector.class")]
	pub connector_class: String,
	pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Plugin {
	pub author: String,
	pub class: String,
	#[serde(rename = "docURL")]
	pub doc_url: String,
	#[serde(deserialize_with = "customdeser::bool_from_string")]
	pub preview: bool,
	pub preview_info: String,
	pub title: String,
	#[serde(rename = "type")]
	pub plugin_type: String,
	pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Task {
	pub connector: String,
	#[serde(deserialize_with = "customdeser::from_str")]
	pub task: i32,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Connector {
	pub config: ConnectorConfig,
	pub name: String,
	pub plugin: Plugin,
	pub tasks: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConnectorConfigurationSchema {
	pub default_value: i32,
	pub display_name: String,
	pub documentation: String,
	pub group: String,
	pub importance: String,
	pub name: String,
	pub order: i32,
	#[serde(deserialize_with = "customdeser::bool_from_string")]
	pub required: bool,
	#[serde(rename = "type")]
	pub schema_type: String,
	pub width: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RespKafkaConnector {
	pub connector: Connector,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RespKafkaConnectorsList {
	pub connectors: Vec<Connector>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RespKafkaConnectorEdit {
	pub compatibility: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SchemaCompatibility {
	pub is_compatible: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RespKafkaConnectorConfigSchema {
	pub configuration_schema: Vec<ConnectorConfigurationSchema>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct KafkaConsumerGroup {
	pub group_name: String,
	pub offset: i32,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct KafkaPartition {
	pub consumer_groups: Vec<KafkaConsumerGroup>,
	pub earliest_offset: i32,
	pub isr: i32,
	pub latest_offset: i32,
	pub partition: i32,
	pub size: i32,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Topic {
	pub cleanup_policy: String,
	pub min_insync_replicas: i32,
	pub partitions: i32,
	pub replication: i32,
	pub retention_bytes: i32,
	pub retention_hours: i32,
	pub state: String,
	pub topic_name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TopicInfo {
	pub cleanup_policy: String,
	pub min_insync_replicas: i32,
	pub partitions: Vec<KafkaPartition>,
	pub replication: i32,
	pub retention_bytes: i32,
	pub retention_hours: i32,
	pub state: String,
	pub topic_name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RespKafkaTopicInfo {
	pub topic: TopicInfo,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaSchemaRegistryVersions {
	pub versions: Vec<i32>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RespKafkaTopicList {
	pub topics: Vec<Topic>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConnectTask {
	pub id: i32,
	pub state: String,
	pub trace: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConnectorStatus {
	pub state: String,
	pub tasks: Vec<ConnectTask>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaConnectConnectorStatus {
	pub status: ConnectorStatus,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Message {
	// pub key: String,
	#[serde(with = "serde_bytes")]
	pub key: Vec<u8>,
	pub offset: i64,
	pub partition: i64,
	pub topic: String,
	// pub value: String,
	#[serde(with = "serde_bytes")]
	pub value: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaMessages {
	pub messages: Vec<Message>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConnectorPlugin {
	pub author: String,
	pub class: String,
	#[serde(rename = "docURL")]
	pub doc_url: String,
	pub preview: String,
	pub preview_info: String,
	pub title: String,
	#[serde(rename = "type")]
	pub plugin_type: String,
	pub version: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaConnectConnectorList {
	pub plugins: Vec<ConnectorPlugin>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Offset {
	pub error: String,
	pub error_code: i64,
	pub offset: i64,
	pub partition: i64,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaProduceMessage {
	pub key_schema_id: i64,
	pub offsets: Vec<Offset>,
	pub value_schema_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaSchemaRegistryConfig {
	#[serde(rename = "compatibilityLevel")]
	pub compatibility_level: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct KafkaACL {
	pub id: String,
	pub permission: String,
	pub topic: String,
	pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaACLEntries {
	pub acl: Vec<KafkaACL>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RespKafkaSchemaRegistrySubjects {
	pub subjects: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResKafkaRegisterSchema {
	pub id: i32,
}
