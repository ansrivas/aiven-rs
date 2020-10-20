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
use serde::Serialize;

use crate::service::types_integrations::*;

pub struct ServiceIntegrationsApi {
	http_client: HTTPClient,
}

impl ServiceIntegrationsApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Create a new service integration endpoint
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationEndpointCreate
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let json_body = json!({
	/// "endpoint_name": "Testing Datadog account",
	/// "endpoint_type": "dashboard",
	/// "user_config": {}
	/// });
	/// let response = client
	///         .service_integrations()
	///         .create_integration_endpoint("my-project", &json_body )
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn create_integration_endpoint<T: Serialize + ?Sized>(
		&self,
		project: &str,
		json_body: &T,
	) -> Result<ResServiceIntegrationEndPoint, AivenError> {
		let url = format!(
			"project/{project}/integration_endpoint",
			project = encode_param(project),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Create a new service integration
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationCreate
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let json_body = json!({
	/// "dest_endpoint_id": "5bdeef77-a901-4487-a0db-54eba23539f8",
	/// "dest_service": "service2",
	/// "integration_type": "dashboard",
	/// "source_endpoint_id": "543e420d-aa63-43e8-b8e8-294a78c600e7",
	/// "source_service": "service1",
	/// "user_config": {}
	/// });
	/// let response = client
	///         .service_integrations()
	///         .create_integration("my-project", &json_body )
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn create_integration<T: Serialize + ?Sized>(
		&self,
		project: &str,
		json_body: &T,
	) -> Result<ResServiceIntegration, AivenError> {
		let url = format!(
			"project/{project}/integration",
			project = encode_param(project),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Delete a service integration endpoint
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationEndpointDelete
	///
	/// # Arguments
	///
	/// * `project` -  Project name
	/// * `integration_endpoint_id` - Endpoint ID
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .delete_integration_endpoint("my-project", "my-id" )
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_integration_endpoint(
		&self,
		project: &str,
		integration_endpoint_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/integration_endpoint/{integration_endpoint_id}",
			project = encode_param(project),
			integration_endpoint_id = encode_param(integration_endpoint_id),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Delete a service integration
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationDelete
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `integration_id` - Integration ID
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .delete_integration("my-project", "my-id" )
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_integration(
		&self,
		project: &str,
		integration_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/integration/{integration_id}",
			project = encode_param(project),
			integration_id = encode_param(integration_id),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Get service integration
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationGet
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `integration_id` - Integration ID
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .get("my-project", "my-id" )
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn get(
		&self,
		project: &str,
		integration_id: &str,
	) -> Result<ResServiceIntegration, AivenError> {
		let url = format!(
			"project/{project}/integration/{integration_id}",
			project = encode_param(project),
			integration_id = encode_param(integration_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List available integration endpoints for project
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationEndpointList
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .list_endpoints_by_project("my-project")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn list_endpoints_by_project(
		&self,
		project: &str,
	) -> Result<ResServiceIntegrationEndPoints, AivenError> {
		let url = format!(
			"project/{project}/integration_endpoint",
			project = encode_param(project),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List available integrations for a service
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationList
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `service_name` - Service name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .list_integrations_for_service("my-project", "myservice")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn list_integrations_for_service(
		&self,
		project: &str,
		service: &str,
	) -> Result<ResServiceIntegrations, AivenError> {
		let url = format!(
			"project/{project}/service/{service}/integration",
			project = encode_param(project),
			service = encode_param(service),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List available service integration endpoint types
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationEndpointTypes
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .list_integration_endpoint_types("my-project")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn list_integration_endpoint_types(
		&self,
		project: &str,
	) -> Result<ResEndpointTypes, AivenError> {
		let url = format!(
			"project/{project}/integration_endpoint_types",
			project = encode_param(project),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List available service integration types
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationTypes
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .list_available_integration_types("my-project")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn list_available_integration_types(
		&self,
		project: &str,
	) -> Result<ResIntegrationTypes, AivenError> {
		let url = format!(
			"project/{project}/integration_endpoint_types",
			project = encode_param(project),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Update a service integration
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationUpdate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `integration_id` - integration id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_integrations()
	///         .update_integration("my-project", "integration_id", "some-user-config")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_integration<T: Serialize + ?Sized>(
		&self,
		project: &str,
		integration_id: &str,
		user_config: &T,
	) -> Result<ResServiceIntegration, AivenError> {
		let url = format!(
			"/project/{project}/integration/{integration_id}",
			project = encode_param(project),
			integration_id = encode_param(integration_id),
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, user_config)?;
		Ok(response.json().await?)
	}

	/// Update service integration endpoint
	///
	/// https://api.aiven.io/doc/#operation/ServiceIntegrationEndpointUpdate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `integration_endpoint_id` - Endpoint id
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let body = json!({
	/// "user_config": {}
	/// });
	/// let response = client
	///         .service_integrations()
	///         .update_integration_endpoint("my-project", "endpoint-id", &body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_integration_endpoint<T: Serialize + ?Sized>(
		&self,
		project: &str,
		endpoint_id: &str,
		user_config: &T,
	) -> Result<ResServiceIntegrationEndPoint, AivenError> {
		let url = format!(
			"/project/{project}/integration_endpoint/{endpoint_id}",
			project = encode_param(project),
			endpoint_id = encode_param(endpoint_id),
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, user_config)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::testutil;
	use serde_json::json;

	#[tokio::test]
	async fn test_service_integrations_create_integration_endpoint() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration_endpoint";
		let test_data =
			testutil::get_test_data("tests/testdata/service/integrations/create_endpoint.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"endpoint_name": "Testing Datadog account",
			"endpoint_type": "dashboard",
			"user_config": {}
		});

		let client = client.service_integrations();
		match client
			.create_integration_endpoint("myproject", &json_body)
			.await
		{
			Ok(resp) => assert!(resp.service_integration_endpoint.endpoint_type == "datadog"),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_create_integration() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration";
		let test_data = testutil::get_test_data("tests/testdata/service/integrations/create.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"dest_endpoint_id": "5bdeef77-a901-4487-a0db-54eba23539f8",
			"dest_service": "service2",
			"integration_type": "dashboard",
			"source_endpoint_id": "543e420d-aa63-43e8-b8e8-294a78c600e7",
			"source_service": "service1",
			"user_config": {}
		});

		let client = client.service_integrations();
		match client.create_integration("myproject", &json_body).await {
			Ok(resp) => assert!(resp.service_integration.active),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_delete_integration_endpoint() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration_endpoint/endpointid";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		let client = client.service_integrations();
		match client
			.delete_integration_endpoint("myproject", "endpointid")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_delete_integration() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration/integrationid";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		let client = client.service_integrations();
		match client
			.delete_integration("myproject", "integrationid")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_get() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration/integrationid";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/integrations/get_service_integration.json",
		);

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let client = client.service_integrations();
		match client.get("myproject", "integrationid").await {
			Ok(_) => assert!(true),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_list_endpoints_by_project() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration_endpoint";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/integrations/list_service_integration_eps.json",
		);

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let client = client.service_integrations();
		match client.list_endpoints_by_project("myproject").await {
			Ok(resp) => assert!(resp.service_integration_endpoints.len() > 0),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_list_integrations_by_service() {
		let client = testutil::client();
		let query_url = "/project/myproject/service/myservice/integration";

		let test_data = testutil::get_test_data(
			"tests/testdata/service/integrations/list_integration_for_service.json",
		);

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let client = client.service_integrations();
		match client
			.list_integrations_for_service("myproject", "myservice")
			.await
		{
			Ok(resp) => assert!(resp.service_integrations.len() > 0),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_list_integration_endpoint_types() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration_endpoint_types";

		let test_data = testutil::get_test_data(
			"tests/testdata/service/integrations/list_integration_endpoint_types.json",
		);

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let client = client.service_integrations();
		match client.list_integration_endpoint_types("myproject").await {
			Ok(resp) => assert!(resp.endpoint_types.len() > 0),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_list_available_integration_types() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration_endpoint_types";

		let test_data = testutil::get_test_data(
			"tests/testdata/service/integrations/list_service_integration_types.json",
		);

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let client = client.service_integrations();
		match client.list_available_integration_types("myproject").await {
			Ok(resp) => assert!(resp.integration_types.len() > 0),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_update_integration() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration/integrationid";

		let test_data = testutil::get_test_data(
			"tests/testdata/service/integrations/update_service_integration.json",
		);

		let body = json!({
			"user_config": {}
		});
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let client = client.service_integrations();
		match client
			.update_integration("myproject", "integrationid", &body)
			.await
		{
			Ok(resp) => assert!(resp.service_integration.active),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_service_integrations_update_integration_endpoint() {
		let client = testutil::client();
		let query_url = "/project/myproject/integration_endpoint/endpointid";

		let test_data = testutil::get_test_data(
			"tests/testdata/service/integrations/update_service_integration_endpoint.json",
		);

		let body = json!({
			"user_config": {}
		});
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let client = client.service_integrations();
		match client
			.update_integration_endpoint("myproject", "endpointid", &body)
			.await
		{
			Ok(resp) => assert!(resp.service_integration_endpoint.endpoint_type == "datadog"),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}
}
