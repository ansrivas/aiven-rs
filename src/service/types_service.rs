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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Alert {
	pub create_time: String,
	pub event: String,
	pub project_name: String,
	pub service_name: String,
	pub service_type: String,
	pub severity: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResAlerts {
	pub alerts: Vec<Alert>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ACL {
	pub id: String,
	pub permission: String,
	pub topic: String,
	pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Backup {
	pub backup_name: String,
	pub backup_time: String,
	pub data_size: i64,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Component {
	pub component: String,
	pub host: String,
	pub kafka_authentication_method: Option<String>,
	pub port: i32,
	pub route: Option<String>,
	pub ssl: Option<bool>,
	pub usage: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConnectionPool {
	pub connection_uri: String,
	pub database: String,
	pub pool_mode: String,
	pub pool_name: String,
	pub pool_size: i32,
	pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Update {
	pub deadline: String,
	pub description: String,
	pub start_after: String,
	pub start_at: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Maintenance {
	pub dow: String,
	pub time: String,
	pub updates: Vec<Update>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProgressUpdate {
	pub completed: bool,
	pub current: i64,
	pub max: i64,
	pub min: i64,
	pub phase: String,
	pub unit: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct NodeState {
	pub name: String,
	pub progress_updates: Vec<ProgressUpdate>,
	pub state: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ServiceIntegration {
	pub active: bool,
	pub description: String,
	pub dest_endpoint: String,
	pub dest_endpoint_id: String,
	pub dest_project: String,
	pub dest_service: String,
	pub dest_service_type: String,
	pub enabled: bool,
	pub integration_status: HashMap<String, serde_json::Value>,
	pub integration_type: String,
	pub service_integration_id: String,
	pub source_endpoint: String,
	pub source_endpoint_id: String,
	pub source_project: String,
	pub source_service: String,
	pub source_service_type: String,
	pub user_config: HashMap<String, serde_json::Value>,
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
pub struct User {
	pub access_cert: String,
	pub access_key: String,
	pub authentication: Option<String>,
	pub password: String,
	#[serde(rename = "type")]
	pub user_type: String,
	pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Service {
	pub acl: Option<Vec<ACL>>,
	pub backups: Vec<Backup>,
	pub cloud_description: String,
	pub cloud_name: String,
	pub components: Vec<Component>,
	pub connection_info: HashMap<String, serde_json::Value>,
	pub connection_pools: Option<Vec<ConnectionPool>>,
	pub create_time: String,
	pub disk_space_mb: i64,
	pub databases: Option<Vec<String>>,
	pub features: HashMap<String, serde_json::Value>,
	pub group_list: Vec<String>,
	pub maintenance: Maintenance,
	pub metadata: HashMap<String, serde_json::Value>,
	pub node_count: i32,
	pub node_cpu_count: i32,
	pub node_memory_mb: i32,
	pub node_states: Vec<NodeState>,
	pub plan: String,
	pub project_vpc_id: Option<String>,
	pub service_integrations: Vec<ServiceIntegration>,
	pub service_name: String,
	pub service_type: String,
	pub service_type_description: String,
	pub service_uri: String,
	pub service_uri_params: HashMap<String, serde_json::Value>,
	pub state: String,
	pub termination_protection: bool,
	pub topics: Option<Vec<Topic>>,
	pub update_time: String,
	pub user_config: HashMap<String, serde_json::Value>,
	pub users: Vec<User>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResService {
	pub service: Service,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResMigrationStatus {
	pub migration: MigrationStatus,
	pub migration_detail: Vec<MigrationDetail>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct MigrationStatus {
	pub error: String,
	pub method: String,
	pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct MigrationDetail {
	pub dbname: String,
	pub error: String,
	pub method: String,
	pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ServiceCancelQuery {
	pub success: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ServiceUser {
	pub access_cert: String,
	pub access_key: String,
	pub authentication: String,
	pub password: String,
	#[serde(rename = "type")]
	pub account_type: String,
	pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceUser {
	pub user: ServiceUser,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResResetQueryStats {
	pub queries: Vec<serde_json::Value>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BackupConfig {
	pub interval: i32,
	pub max_count: i32,
	pub recovery_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Region {
	pub disk_space_mb: i32,
	pub node_cpu_count: i32,
	pub node_memory_mb: i32,
	pub price_usd: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ServicePlan {
	pub backup_config: BackupConfig,
	pub node_count: i64,
	pub regions: HashMap<String, Region>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServices {
	pub services: Vec<Service>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ServiceDescription {
	pub description: String,
	pub service_plans: Vec<ServicePlan>,
	pub latest_available_version: Option<String>,
	pub user_config_schema: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserConfigSchema {
	#[serde(rename = "additionalProperties")]
	pub additional_properties: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceTypes {
	pub service_types: HashMap<String, ServiceDescription>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DatabaseName {
	pub database_name: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResDatabaseNames {
	pub databases: Vec<DatabaseName>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Database {
	pub database_name: String,
	pub lc_collate: String,
	pub lc_ctype: String,
	pub owner: String,
	pub quoted_owner: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceDatabaseList {
	pub databases: Vec<Database>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResEnableWrites {
	pub until: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceCA {
	// PEM encoded certificate
	pub certificate: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceKeyPair {
	// PEM encoded certificate
	pub certificate: String,
	pub key: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Task {
	pub create_time: String,
	pub result: String,
	pub success: bool,
	pub task_type: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResTask {
	pub task: Task,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Log {
	pub msg: String,
	pub time: String,
	pub unit: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResLogs {
	pub first_log_offset: String,
	pub logs: Vec<Log>,
	pub offset: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResQueries {
	pub queries: Vec<Query>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Query {
	pub active_channel_subscriptions: i32,
	pub active_database: String,
	pub active_pattern_matching_channel_subscriptions: i32,
	pub application_name: String,
	pub backend_start: String,
	pub backend_type: String,
	pub backend_xid: String,
	pub backend_xmin: String,
	pub client_addr: String,
	pub client_hostname: String,
	pub client_port: i32,
	pub connection_age_seconds: i32,
	pub connection_idle_seconds: i32,
	pub datid: i32,
	pub datname: String,
	pub flags: Vec<String>,
	pub flags_raw: String,
	pub id: String,
	pub multi_exec_commands: i32,
	pub name: String,
	pub output_buffer: i32,
	pub output_buffer_memory: i32,
	pub output_list_length: i32,
	pub pid: i32,
	pub query: String,
	pub query_buffer: i32,
	pub query_buffer_free: i32,
	pub query_duration: f32,
	pub query_start: String,
	pub state: String,
	pub state_change: String,
	pub usename: String,
	pub usesysid: i32,
	pub wait_event: String,
	pub wait_event_type: String,
	pub waiting: bool,
	pub xact_start: String,
}
