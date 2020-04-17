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

use crate::{
	client::{encode_param, HTTPClient},
	errors::AivenError,
	make_json_request, make_request,
	response::APIResponse,
};

use crate::service::types_kafka::*;
use serde::Serialize;
pub struct ServiceKafkaApi {
	http_client: HTTPClient,
}

impl ServiceKafkaApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Add a Kafka ACL entry
	///
	/// https://api.aiven.io/doc/#api-Service_-_Kafka-ServiceKafkaAclAdd
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` -  Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	/// "permission": "readwrite",
	/// "topic": "top*",
	/// "username": "admin*"
	/// });
	/// let response = client
	///         .service_kafka()
	///         .add_kafka_acl_entry("some-project", "service-name", &json_body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn add_kafka_acl_entry<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<Acl, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/acl",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Check compatibility of schema in Schema Registry
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	/// * `version_id` - Version Id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	///         "schema": "{\"type\": \"string\"}"
	/// });
	/// let response = client
	///         .service_kafka()
	///         .check_compatibility_schema_registry(
	///         "myproject",
	///         "myservicename",
	///         "mysubjectname",
	///         "myversionid",
	///         &json_body)
	///        .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn check_compatibility_schema_registry<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
		version_id: &str,
		json_body: &T,
	) -> Result<SchemaCompatibility, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/compatibility/subjects/\
			 {subject_name}/versions/{version_id}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
			version_id = encode_param(version_id),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Create a Kafka Connect connector
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	///         "connector.class": "io.debezium.connector.postgresql.PostgresConnector",
	///         "name": "s3-sink-us-east-1"
	/// });
	/// let response = client
	///             .service_kafka()
	///             .create_kafka_connector("myproject", "myservicename", &json_body)
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_kafka_connector<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<RespKafkaConnector, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Create a Kafka topic
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	///         "cleanup_policy": "delete",
	///         "min_insync_replicas": 2,
	///         "partitions": 1,
	///         "replication": 1,
	///         "retention_bytes": 72,
	///         "retention_hours": 72,
	///         "topic_name": "mytopic"
	/// });
	/// let response = client
	///         .service_kafka()
	///        .create_kafka_topic("myproject", "myservicename", &json_body)
	///        .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_kafka_topic<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/topic",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let _response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(())
	}

	/// Delete kafka connect connector.
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .delete_kafka_connector("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_kafka_connector(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors/{connector_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Delete Schema Registry subject version
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	/// * `version_id` - Version Id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .delete_schema_registry_subject_version(
	///                 "myproject",
	///                 "myservicename",
	///                 "mysubject",
	///                 "versionid")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_schema_registry_subject_version(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
		version_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/subjects/{subject_name}/\
			 versions/{version_id}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
			version_id = encode_param(version_id),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Delete Schema Registry subject
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .delete_schema_registry_subject("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_schema_registry_subject(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/subjects/{subject_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Delete a Kafka ACL entry
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `kafka_acl_id` - Kafka ACL ID
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .delete_acl_entry("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_acl_entry(
		&self,
		project: &str,
		service_name: &str,
		kafka_acl_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/acl/{kafka_acl_id}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			kafka_acl_id = encode_param(kafka_acl_id),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Delete a Kafka topic
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `topic_name` - Kafka topic name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .delete_topic("myproject", "myservicename", "mytopic")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_topic(
		&self,
		project: &str,
		service_name: &str,
		topic_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/topic/{topic_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			topic_name = encode_param(topic_name),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Edit Kafka Connect connector.
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	/// "connector.class": "io.debezium.connector.postgresql.PostgresConnector",
	/// "name": "s3-sink-us-east-1"
	/// });
	/// let response = client
	///             .service_kafka()
	///             .edit_kafka_connector("myproject", "myservicename", "myconnector", &json_body)
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn edit_kafka_connector<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
		json_body: &T,
	) -> Result<RespKafkaConnector, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors/{connector_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Edit configuration for Schema Registry subject.
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({ "compatibility": "FULL" });
	/// let response = client
	///             .service_kafka()
	///             .edit_schema_registry_config(
	///                 "myproject",
	///                 "myservicename",
	///                 "mysubjectname",
	///                 &json_body)
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn edit_schema_registry_config<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
		json_body: &T,
	) -> Result<RespKafkaConnectorEdit, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/config/{subject_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
		);
		let _response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(_response.json().await?)
	}

	/// Edit global configuration for Schema Registry
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .edit_schema_registry_config_global("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn edit_schema_registry_config_global<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<RespKafkaConnectorEdit, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/config",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let _response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(_response.json().await?)
	}

	/// Get Kafka Connect connector configuration schema
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_kafka_connect_configuration_schema(
	///                 "myproject",
	///                 "myservicename",
	///                 "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_kafka_connect_configuration_schema(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
	) -> Result<RespKafkaConnectorConfigSchema, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connector-plugins/{connector_name}/\
			 configuration",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
		);
		let _response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(_response.json().await?)
	}

	/// Get Kafka topic info
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `topic_name` - Kafka topic name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_topic_info("myproject", "myservicename", "mytopic")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_topic_info(
		&self,
		project: &str,
		service_name: &str,
		topic_name: &str,
	) -> Result<RespKafkaTopicInfo, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/topic/{topic_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			topic_name = encode_param(topic_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get Kafka topic list
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_topic_list("myproject", "myservicename")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_topic_list(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<RespKafkaTopicList, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/topic",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get Schema Registry Subject version
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	/// * `version_id` - Version Id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_schema_registry_subject_version(
	///               "project",
	///               "servicename",
	///               "connector",
	///               "versionid")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_schema_registry_subject_version(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
		version_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/subjects/{subject_name}/\
			 versions/{version_id}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
			version_id = encode_param(version_id),
		);
		let _response = make_request!(self, reqwest::Method::GET, &url)?;
		// Ok(response.json().await?)
		Ok(())
	}

	/// Get Schema Registry Subject versions
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_schema_registry_subject_versions("myproject", "myservicename", "mysubject")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_schema_registry_subject_versions(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
	) -> Result<ResKafkaSchemaRegistryVersions, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/subjects/{subject_name}/\
			 versions",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get a Kafka Connect Connector status
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_kafka_connect_status("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_kafka_connect_status(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
	) -> Result<ResKafkaConnectConnectorStatus, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors/{connector_name}/status",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get available Kafka Connect connectors
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_kafka_connect_connectors("myproject", "myservicename")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_kafka_connect_connectors(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResKafkaConnectConnectorList, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/available-connectors",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get configuration for Schema Registry subject
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_config_schema_registry("myproject", "myservicename", "mysubject")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_config_schema_registry(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
	) -> Result<ResKafkaSchemaRegistryConfig, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/config/{subject_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get configuration for Schema Registry subject
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_config_schema_registry_global("myproject", "myservicename")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_config_schema_registry_global(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResKafkaSchemaRegistryConfig, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/config",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get schema in Schema Registry
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `schema_id` - Schema Id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_schema_in_schema_registry("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_schema_in_schema_registry(
		&self,
		project: &str,
		service_name: &str,
		schema_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/schemas/ids/{schema_id}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			schema_id = encode_param(schema_id),
		);
		let _response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(())
	}

	/// Get schema of a specific version in Schema Registry
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	/// * `version_id` - Version Id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .get_schema_in_schema_registry_by_version(
	///                 "myproject",
	///                 "myservicename",
	///                 "myconnector",
	///                 "versionid")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_schema_in_schema_registry_by_version(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
		version_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/subjects/{subject_name}/\
			 versions/{version_id}/schema",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
			version_id = encode_param(version_id),
		);
		let _response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(())
	}

	/// List Kafka ACL entries
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	/// * `version_id` - Version Id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .list_acl_entries("myproject", "myservicename")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_acl_entries(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResKafkaACLEntries, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/acl",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List kafka topic messages
	///
	/// https://api.aiven.io/doc/#api-Service_-_Kafka-ServiceKafkaTopicMessageList
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `topic` - Kafka topic name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	/// "max_bytes": 67108864,
	/// "partitions": {
	///     "offset": 0
	/// },
	/// "timeout": 3000
	/// });
	/// let response = client
	///             .service_kafka()
	///             .list_topic_messages("myproject", "myservicename", "mytopic", &json_body)
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_topic_messages<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		topic: &str,
		json_body: &T,
	) -> Result<ResKafkaMessages, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/rest/topics/{topic}/messages",
			project = encode_param(project),
			service_name = encode_param(service_name),
			topic = encode_param(topic),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}
	/// Lists Kafka connectors
	///
	/// https://api.aiven.io/doc/#api-Service_-_Kafka-ServiceKafkaConnectList
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .list_kafka_connectors("myproject", "myservicename")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_kafka_connectors(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<RespKafkaConnectorsList, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Lists Schema Registry subjects
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .list_schema_registry_subjects("myproject", "myservicename")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_schema_registry_subjects(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<RespKafkaSchemaRegistrySubjects, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/subjects",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Pause a Kafka Connect Connector
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .pause_kafka_connector("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn pause_kafka_connector(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors/{connector_name}/pause",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
		);
		let _response = make_request!(self, reqwest::Method::POST, &url)?;
		Ok(())
	}

	/// Produce message into a kafka topic
	///
	/// https://api.aiven.io/doc/#api-Service_-_Kafka-ServiceKafkaTopicMessageProduce
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `topic` - topic name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let json_body = json!({
	///  "format": "binary",
	///  "key_schema": "{'name':'int','type': 'int'}",
	///  "key_schema_id": 123,
	///  "records": [
	///      {
	///          "key": {},
	///          "partition": 1,
	///          "value": {}
	///      }
	///  ],
	///  "value_schema": "{'name':'int','type': 'int'}",
	///  "value_schema_id": 123
	///  });
	/// let response = client
	///             .service_kafka()
	///             .produce_message("myproject", "myservicename", "topic", &json_body)
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn produce_message<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		topic: &str,
		json_body: &T,
	) -> Result<ResKafkaProduceMessage, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/rest/topics/{topic}/produce",
			project = encode_param(project),
			service_name = encode_param(service_name),
			topic = encode_param(topic),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Register a new Schema in Schema Registry
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `subject_name` - Subject name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	///         "schema": "{\"type\": \"string\"}"
	/// });
	/// let response = client
	///             .service_kafka()
	///             .register_schema("myproject", "myservicename", "mysubjectname", &json_body)
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn register_schema<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		subject_name: &str,
		json_body: &T,
	) -> Result<ResKafkaRegisterSchema, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kafka/schema/subjects/{subject_name}/\
			 versions",
			project = encode_param(project),
			service_name = encode_param(service_name),
			subject_name = encode_param(subject_name),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Restart a Kafka Connect Connector task
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	/// * `task_id` - Service task
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .restart_kafka_connect_connector_task(
	///                 "myproject",
	///                 "myservicename",
	///                 "myconnector",
	///                 "taskid")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn restart_kafka_connect_connector_task(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
		task_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors/{connector_name}/tasks/{task_id}/\
			 restart",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
			task_id = encode_param(task_id),
		);
		let _response = make_request!(self, reqwest::Method::POST, &url)?;
		Ok(())
	}

	/// Restart a Kafka Connect Connector
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .restart_kafka_connect_connector("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn restart_kafka_connect_connector(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors/{connector_name}/restart",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
		);
		let _response = make_request!(self, reqwest::Method::POST, &url)?;
		Ok(())
	}

	/// Resume a Kafka Connect Connector
	///
	/// https://api.aiven.io/doc/#api-Service_-_Kafka-ServiceKafkaConnectResumeConnector
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `connector_name` - Connector name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///             .service_kafka()
	///             .resume_kafka_connect_connector("myproject", "myservicename", "myconnector")
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn resume_kafka_connect_connector(
		&self,
		project: &str,
		service_name: &str,
		connector_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/connectors/{connector_name}/resume",
			project = encode_param(project),
			service_name = encode_param(service_name),
			connector_name = encode_param(connector_name),
		);
		let _response = make_request!(self, reqwest::Method::POST, &url)?;
		Ok(())
	}

	/// Update a Kafka topic
	///
	/// https://api.aiven.io/doc/#api-Service_-_Kafka-ServiceKafkaTopicUpdate
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `service_name` - Service name
	/// * `topic_name` - Kafka topic name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	/// "min_insync_replicas": 2,
	/// "partitions": 1,
	/// "replication": 1,
	/// "retention_bytes": 72,
	/// "retention_hours": 72
	/// });
	/// let response = client
	///             .service_kafka()
	///             .update_topic("myproject", "myservicename", "topic", &json_body)
	///             .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_topic<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		topic_name: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/topic/{topic_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			topic_name = encode_param(topic_name),
		);
		let _response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use crate::testutil;
	use serde_json::json;

	#[tokio::test]
	async fn test_service_kafka_add_kafka_acl_entry() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/acl";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/add_kafka_acl_entry.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"permission": "readwrite",
			"topic": "top*",
			"username": "admin*"
		});
		match client
			.service_kafka()
			.add_kafka_acl_entry("myproject", "myservicename", &json_body)
			.await
		{
			Ok(response) => {
				assert!(response.acl.len() > 0);
				assert!(response.acl[0].id == "id-e11v3n")
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_check_compatibility_schema_registry() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/compatibility/\
		                 subjects/mysubjectname/versions/myversionid";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/kafka/check_compatibility_schema_registry.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"schema": "{\"type\": \"string\"}"
		});

		match client
			.service_kafka()
			.check_compatibility_schema_registry(
				"myproject",
				"myservicename",
				"mysubjectname",
				"myversionid",
				&json_body,
			)
			.await
		{
			Ok(response) => assert!(!response.is_compatible),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_create_kafka_connector() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/create_kafka_connector.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"connector.class": "io.debezium.connector.postgresql.PostgresConnector",
			"name": "s3-sink-us-east-1"
		});

		match client
			.service_kafka()
			.create_kafka_connector("myproject", "myservicename", &json_body)
			.await
		{
			Ok(response) => assert!(response.connector.name == "elastic-sink"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_kafka_create_kafka_topic() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/topic";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"cleanup_policy": "delete",
			"min_insync_replicas": 2,
			"partitions": 1,
			"replication": 1,
			"retention_bytes": 72,
			"retention_hours": 72,
			"topic_name": "mytopic"
		});

		match client
			.service_kafka()
			.create_kafka_topic("myproject", "myservicename", &json_body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_delete_kafka_connector() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors/myconnector";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.service_kafka()
			.delete_kafka_connector("myproject", "myservicename", "myconnector")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_delete_schema_registry_subject_version() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/subjects/mysubject/\
		                 versions/myversion";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.service_kafka()
			.delete_schema_registry_subject_version(
				"myproject",
				"myservicename",
				"mysubject",
				"myversion",
			)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_delete_schema_registry_subject() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/subjects/mysubject";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.service_kafka()
			.delete_schema_registry_subject("myproject", "myservicename", "mysubject")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_delete_acl_entry() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/acl/myaclid";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/delete_acl_entry.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.service_kafka()
			.delete_acl_entry("myproject", "myservicename", "myaclid")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_kafka_delete_topic() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/topic/mytopic";
		let test_data = "".to_string();

		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.service_kafka()
			.delete_topic("myproject", "myservicename", "mytopic")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_edit_kafka_connector() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors/myconnector";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/edit_kafka_connector.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");
		let json_body = json!({
			"connector.class": "io.debezium.connector.postgresql.PostgresConnector",
			"name": "s3-sink-us-east-1"
		});
		match client
			.service_kafka()
			.edit_kafka_connector("myproject", "myservicename", "myconnector", &json_body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_edit_schema_registry_config() {
		let client = testutil::prepare_test_client();
		let query_url =
			"/project/myproject/service/myservicename/kafka/schema/config/mysubjectname";
		let test_data = r#"{"compatibility": "FULL"}"#;
		let _m = testutil::create_mock_server(query_url, test_data, "PUT");
		let json_body = json!({
			"compatibility": "FULL"
		});
		match client
			.service_kafka()
			.edit_schema_registry_config("myproject", "myservicename", "mysubjectname", &json_body)
			.await
		{
			Ok(response) => assert!(response.compatibility == "FULL"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_edit_schema_registry_config_global() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/config";
		let test_data = r#"{"compatibility": "FULL"}"#;
		let _m = testutil::create_mock_server(query_url, test_data, "PUT");
		let json_body = json!({
			"compatibility": "FULL"
		});
		match client
			.service_kafka()
			.edit_schema_registry_config_global("myproject", "myservicename", &json_body)
			.await
		{
			Ok(response) => assert!(response.compatibility == "FULL"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_kafka_connect_configuration_schema() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connector-plugins/\
		                 myconnectorname/configuration";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/kafka/get_kafka_connect_configuration_schema.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_kafka_connect_configuration_schema("myproject", "myservicename", "myconnectorname")
			.await
		{
			Ok(response) => {
				assert!(response.configuration_schema.len() > 0);
				assert!(response.configuration_schema[0].default_value == 1)
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_topic_info() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/topic/mytopic";
		let test_data = testutil::get_test_data("tests/testdata/service/kafka/get_topic_info.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_topic_info("myproject", "myservicename", "mytopic")
			.await
		{
			Ok(response) => {
				assert!(response.topic.cleanup_policy == "delete");
				assert!(response.topic.topic_name == "mytopic")
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_topic_list() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/topic";
		let test_data = testutil::get_test_data("tests/testdata/service/kafka/get_topic_list.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_topic_list("myproject", "myservicename")
			.await
		{
			Ok(response) => {
				assert!(response.topics.len() > 0);
				assert!(response.topics[0].topic_name == "mytopic");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[ignore]
	#[tokio::test]
	async fn test_service_kafka_get_schema_registry_subject_version() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/subjects/mysubject/\
		                 versions/myversion";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/kafka/get_schema_registry_subject_version.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_schema_registry_subject_version(
				"myproject",
				"myservicename",
				"mysubject",
				"myversion",
			)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_schema_registry_subject_versions() {
		let client = testutil::prepare_test_client();
		let query_url =
			"/project/myproject/service/myservicename/kafka/schema/subjects/mysubject/versions";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/kafka/get_schema_registry_subject_versions.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_schema_registry_subject_versions("myproject", "myservicename", "mysubject")
			.await
		{
			Ok(response) => assert!(response.versions.len() > 0),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_kafka_connect_status() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors/myconnector/status";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/get_kafka_connect_status.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_kafka_connect_status("myproject", "myservicename", "myconnector")
			.await
		{
			Ok(response) => assert!(response.status.tasks.len() > 0),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_kafka_connect_connectors() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/available-connectors";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/kafka/get_kafka_connect_connectors.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_kafka_connect_connectors("myproject", "myservicename")
			.await
		{
			Ok(response) => {
				assert!(response.plugins.len() > 0);
				assert!(response.plugins[0].author == "Debezium")
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_config_schema_registry() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/config/mysubject";
		let test_data = r#"{ "compatibilityLevel": "FULL"}"#;
		let _m = testutil::create_mock_server(query_url, test_data, "GET");

		match client
			.service_kafka()
			.get_config_schema_registry("myproject", "myservicename", "mysubject")
			.await
		{
			Ok(response) => assert!(response.compatibility_level == "FULL"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_get_config_schema_registry_global() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/config";
		let test_data = r#"{ "compatibilityLevel": "FULL"}"#;
		let _m = testutil::create_mock_server(query_url, test_data, "GET");

		match client
			.service_kafka()
			.get_config_schema_registry_global("myproject", "myservicename")
			.await
		{
			Ok(response) => assert!(response.compatibility_level == "FULL"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[ignore]
	#[tokio::test]
	async fn test_service_kafka_get_schema_in_schema_registry() {
		let client = testutil::prepare_test_client();
		let query_url =
			"/project/myproject/service/myservicename/kafka/schema/schemas/ids/myschemaid";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_schema_in_schema_registry("myproject", "myservicename", "myschemaid")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[ignore]
	#[tokio::test]
	async fn test_service_kafka_get_schema_in_schema_registry_by_version() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/subjects/\
		                 mysubjectname/versions/myversionid/schema";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.get_schema_in_schema_registry_by_version(
				"myproject",
				"myservicename",
				"mysubjectname",
				"myversionid",
			)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_list_acl_entries() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/acl";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/list_acl_entries.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.list_acl_entries("myproject", "myservicename")
			.await
		{
			Ok(response) => assert!(response.acl.len() > 0),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_list_topic_messages() {
		let client = testutil::prepare_test_client();
		let query_url =
			"/project/myproject/service/myservicename/kafka/rest/topics/mytopic/messages";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/list_topic_messages.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"max_bytes": 67108864,
			"partitions": {
				"offset": 0
			},
			"timeout": 3000
		});
		match client
			.service_kafka()
			.list_topic_messages("myproject", "myservicename", "mytopic", &json_body)
			.await
		{
			Ok(response) => {
				assert!(response.messages.len() > 0);
				assert!(response.messages[0].offset == 10);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_list_kafka_connectors() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/list_kafka_connectors.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.list_kafka_connectors("myproject", "myservicename")
			.await
		{
			Ok(response) => {
				assert!(response.connectors.len() > 0);
				assert!(response.connectors[0].name == "elastic-sink");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_kafka_list_schema_registry_subjects() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/kafka/schema/subjects";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/kafka/list_schema_registry_subjects.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_kafka()
			.list_schema_registry_subjects("myproject", "myservicename")
			.await
		{
			Ok(response) => {
				assert!(response.subjects.len() > 0);
				assert!(response.subjects[0] == "topic1-values");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_pause_kafka_connector() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors/myconnectorname/pause";
		let test_data = "".to_string();

		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.service_kafka()
			.pause_kafka_connector("myproject", "myservicename", "myconnectorname")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_produce_message() {
		let client = testutil::prepare_test_client();
		let query_url =
			"/project/myproject/service/myservicename/kafka/rest/topics/mytopic/produce";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/produce_message.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"format": "binary",
			"key_schema": "{'name':'int','type': 'int'}",
			"key_schema_id": 123,
			"records": [
				{
					"key": {},
					"partition": 1,
					"value": {}
				}
			],
			"value_schema": "{'name':'int','type': 'int'}",
			"value_schema_id": 123
		});
		match client
			.service_kafka()
			.produce_message("myproject", "myservicename", "mytopic", &json_body)
			.await
		{
			Ok(response) => {
				assert!(response.key_schema_id == 1);
				assert!(response.offsets.len() > 0);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_register_schema() {
		let client = testutil::prepare_test_client();
		let query_url =
			"/project/myproject/service/myservicename/kafka/schema/subjects/mysubject/versions";
		let test_data =
			testutil::get_test_data("tests/testdata/service/kafka/register_schema.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"schema": "{\"type\": \"string\"}"
		});
		match client
			.service_kafka()
			.register_schema("myproject", "myservicename", "mysubject", &json_body)
			.await
		{
			Ok(response) => assert!(response.id == 1),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_restart_kafka_connect_connector() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors/myconnector/restart";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.service_kafka()
			.restart_kafka_connect_connector("myproject", "myservicename", "myconnector")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_restart_kafka_connect_connector_task() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors/myconnector/tasks/\
		                 mytaskid/restart";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.service_kafka()
			.restart_kafka_connect_connector_task(
				"myproject",
				"myservicename",
				"myconnector",
				"mytaskid",
			)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_resume_kafka_connect_connector() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/connectors/myconnector/resume";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.service_kafka()
			.resume_kafka_connect_connector("myproject", "myservicename", "myconnector")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_kafka_update_topic() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservicename/topic/mytopic";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let json_body = json!({
			"min_insync_replicas": 2,
			"partitions": 1,
			"replication": 1,
			"retention_bytes": 72,
			"retention_hours": 72
		});
		match client
			.service_kafka()
			.update_topic("myproject", "myservicename", "mytopic", &json_body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
