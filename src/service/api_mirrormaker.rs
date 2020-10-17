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
};

use crate::service::types_mirrormaker::*;
use serde::Serialize;
pub struct ServiceKafkaMirrorMaker {
	http_client: HTTPClient,
}

impl ServiceKafkaMirrorMaker {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Create a replication flow
	///
	/// https://api.aiven.io/doc/#operation/ServiceKafkaMirrorMakerCreateReplicationFlow
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	///
	/// let mut json_body = json!({
	///     "enabled": "true",
	///     "source_cluster": "string",
	///     "target_cluster": "string",
	///     "topics": [
	///     {
	///         "blacklist":
	///                 []
	///             }
	///         ]
	/// });
	/// let response = client
	///         .service_kafka_mirrormaker()
	///         .create_replication_flow("project", "service-name", &json_body)
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn create_replication_flow<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let _response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(())
	}

	/// Get replication flows
	///
	/// https://api.aiven.io/doc/#operation/ServiceKafkaMirrorMakerGetReplicationFlows
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	///
	/// let response = client
	///         .service_kafka_mirrormaker()
	///         .get_replication_flows("project", "service-name")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn get_replication_flows(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ReplicationFlows, AivenError> {
		let url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Delete a replication flow
	///
	/// https://api.aiven.io/doc/#operation/ServiceKafkaMirrorMakerDeleteReplicationFlow
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	///
	/// let response = client
	///         .service_kafka_mirrormaker()
	///         .delete_replication_flow("project",
	///          "service-name", "source-cluster", "target-cluster")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_replication_flow(
		&self,
		project: &str,
		service_name: &str,
		source_cluster: &str,
		target_cluster: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows/\
			 {source_cluster}/{target_cluster}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			source_cluster = encode_param(source_cluster),
			target_cluster = encode_param(target_cluster)
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Get a replication flow
	///
	/// https://api.aiven.io/doc/#operation/ServiceKafkaMirrorMakerGetReplicationFlow
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	///
	/// let response = client
	///         .service_kafka_mirrormaker()
	///         .get_replication_flow("project",
	///         "service-name", "source-cluster", "target-cluster")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn get_replication_flow(
		&self,
		project: &str,
		service_name: &str,
		source_cluster: &str,
		target_cluster: &str,
	) -> Result<ReplicationFlowResponse, AivenError> {
		let url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows/\
			 {source_cluster}/{target_cluster}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			source_cluster = encode_param(source_cluster),
			target_cluster = encode_param(target_cluster)
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get a replication flow
	///
	/// https://api.aiven.io/doc/#operation/ServiceKafkaMirrorMakerGetReplicationFlow
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	///
	/// let payload = json!({"enabled": true, "topics": [{"blacklist": ["something"]}]});
	/// let response = client
	///         .service_kafka_mirrormaker()
	///         .update_replication_flow("project",
	///         "service-name", "source-cluster", "target-cluster", &payload)
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn update_replication_flow<T: ?Sized + Serialize>(
		&self,
		project: &str,
		service_name: &str,
		source_cluster: &str,
		target_cluster: &str,
		json_body: &T,
	) -> Result<ReplicationFlowResponse, AivenError> {
		let url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows/\
			 {source_cluster}/{target_cluster}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			source_cluster = encode_param(source_cluster),
			target_cluster = encode_param(target_cluster)
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::{client::encode_param, testutil};
	use serde_json::json;

	#[tokio::test]
	async fn test_service_mirrormaker_create_replication_flow() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows",
			project = encode_param("project"),
			service_name = encode_param("service_name"),
		);

		// let test_data =
		// testutil::get_test_data("tests/testdata/account/new_auth_method.json");
		let _m = testutil::create_mock_server(&query_url, "", "POST");

		let data = json!({
			"enabled": "true",
			"source_cluster": "string",
			"target_cluster": "string",
			"topics": [
			{
				"blacklist":
						[]
					}
				]
		});
		match client
			.service_kafka_mirrormaker()
			.create_replication_flow("project", "service_name", &data)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_mirrormaker_get_replication_flows() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows",
			project = encode_param("project"),
			service_name = encode_param("service_name"),
		);

		let test_data = testutil::get_test_data(
			"tests/testdata/service/mirrormaker/get_replication_flows.json",
		);
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");
		match client
			.service_kafka_mirrormaker()
			.get_replication_flows("project", "service_name")
			.await
		{
			Ok(response) => {
				assert!(response.replication_flows[0].target_cluster == "target-cluster")
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_mirrormaker_delete_replication_flow() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows/\
			 {source_cluster}/{target_cluster}",
			project = encode_param("project"),
			service_name = encode_param("service_name"),
			source_cluster = encode_param("source-cluster"),
			target_cluster = encode_param("target-cluster")
		);

		// let test_data = testutil::get_test_data(
		// 	"tests/testdata/service/mirrormaker/delete_replication_flow.json",
		// );
		let _m = testutil::create_mock_server(&query_url, "", "DELETE");
		match client
			.service_kafka_mirrormaker()
			.delete_replication_flow(
				"project",
				"service_name",
				"source-cluster",
				"target-cluster",
			)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_mirrormaker_get_replication_flow() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows/\
			 {source_cluster}/{target_cluster}",
			project = encode_param("project"),
			service_name = encode_param("service_name"),
			source_cluster = encode_param("source-cluster"),
			target_cluster = encode_param("target-cluster")
		);

		let test_data =
			testutil::get_test_data("tests/testdata/service/mirrormaker/get_replication_flow.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");
		match client
			.service_kafka_mirrormaker()
			.get_replication_flow(
				"project",
				"service_name",
				"source-cluster",
				"target-cluster",
			)
			.await
		{
			Ok(response) => assert!(response.replication_flow.source_cluster == "source-cluster"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_mirrormaker_update_replication_flow() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/project/{project}/service/{service_name}/mirrormaker/replication-flows/\
			 {source_cluster}/{target_cluster}",
			project = encode_param("project"),
			service_name = encode_param("service_name"),
			source_cluster = encode_param("source-cluster"),
			target_cluster = encode_param("target-cluster")
		);

		let test_data = testutil::get_test_data(
			"tests/testdata/service/mirrormaker/update_replication_flow.json",
		);
		let _m = testutil::create_mock_server(&query_url, &test_data, "PUT");

		let payload = json!({
		"enabled": true,
		"topics": [
			{
			"blacklist": [
				{}
			]
			}
		]
		});
		match client
			.service_kafka_mirrormaker()
			.update_replication_flow(
				"project",
				"service_name",
				"source-cluster",
				"target-cluster",
				&payload,
			)
			.await
		{
			Ok(response) => assert!(response.replication_flow.source_cluster == "source-cluster"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
