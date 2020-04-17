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

use crate::service::types_service::*;
use serde::Serialize;
pub struct ServiceApi {
	http_client: HTTPClient,
}

impl ServiceApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Cancel specified query from service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceCancelQuery
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let mut json_body = HashMap::new();
	/// json_body.insert("authentication", "caching_sha2_password");
	/// json_body.insert("username", "some-user");
	/// let response = client
	///     .service()
	///     .cancel_query("some-project", "some-service_name", &json_body)
	///     .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn cancel_query<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<ServiceCancelQuery, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/query/cancel",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Create a new (sub) user for service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceUserCreate
	///
	/// # Arguments
	///
	/// * `authentication` - Authentication details, Allowed values:
	///   "caching_sha2_password", "mysql_native_password"
	/// * `username`   - Service username, Size range: ..40
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let mut json_body = HashMap::new();
	/// json_body.insert("authentication", "caching_sha2_password");
	/// json_body.insert("username", "some-user");
	/// let response = client
	///     .service()
	///     .create_user("some-project", "some-service_name", &json_body)
	///     .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_user<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<ResServiceUser, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/user",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Create a new logical database for service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceDatabaseCreate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `service_name` - service_name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let mut json_body = HashMap::new();
	/// json_body.insert("database", "testdb");
	/// let response = client
	///     .service()
	///     .create_logical_database("some-project", "some-service_name", &json_body)
	///     .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_logical_database<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/db",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let _response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(())
	}

	/// Create a new task for service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceTaskCreate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `service_name` - service_name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let mut json_body = HashMap::new();
	/// json_body.insert("target_version", "10");
	/// json_body.insert("task_type", "upgrade_check");
	/// let response = client
	///    .service()
	///    .create_new_task("some-project", "some-service_name", &json_body)
	///    .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_new_task<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<ResTask, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/task",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Create a service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceCreate
	///
	/// # Arguments
	///
	/// * `project`   - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// use std::collections::HashMap;
	/// let mut json_body = HashMap::new();
	/// json_body.insert("authentication", "caching_sha2_password");
	/// json_body.insert("username", "some-user");
	/// let response = client
	///         .service()
	///         .create_service("project-name",&json_body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_service<T: Serialize + ?Sized>(
		&self,
		project: &str,
		json_body: &T,
	) -> Result<ResService, AivenError> {
		let url = format!("project/{project}/service", project = encode_param(project),);
		Ok(
			make_json_request!(self, reqwest::Method::POST, &url, json_body)?
				.json()
				.await?,
		)
	}

	/// Delete a logical database
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceDatabaseDelete
	///
	/// # Arguments
	///
	/// * `project`   - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .delete_logical_db("project-name","servicename", "dbname")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_logical_db(
		&self,
		project: &str,
		service_name: &str,
		db_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/db/{db_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			db_name = encode_param(db_name),
		);

		let _resp = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Delete a service user
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceUserDelete
	///
	/// # Arguments
	///
	/// * `project`   - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .delete_user("project-name","servicename", "service_username")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_user(
		&self,
		project: &str,
		service_name: &str,
		service_username: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/user/{service_username}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			service_username = encode_param(service_username),
		);

		let _resp = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Fetch current queries for the service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceQueryActivity
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let body = json!({
	/// "limit": 100,
	/// "offset": 100,
	/// "order_by": "client_id:desc"
	/// });
	/// let response = client
	///         .service()
	///         .fetch_current_queries("my-project", "my-service-name", &body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn fetch_current_queries<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<ResQueries, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/query/activity",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);

		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;

		Ok(response.json().await?)
	}

	/// Fetch service metrics
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceMetricsFetch
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let body = json!({   "period": "hour"});
	/// let response = client
	///         .service()
	///         .fetch_service_metrics("my-project", "my-service-name", &body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn fetch_service_metrics<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<serde_json::Value, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/metrics",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);

		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;

		Ok(response.json().await?)
	}

	/// Get details for a single user
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceUserGet
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///         .service()
	///         .get_user_details("project", "service_name", "myserviceuser")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_user_details(
		&self,
		project: &str,
		service_name: &str,
		service_username: &str,
	) -> Result<ResServiceUser, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/user/{service_username}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			service_username = encode_param(service_username),
		);
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// Get service information
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceGet
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///         .service()
	///         .get_service_info("project", "service_name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_service_info(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResService, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// Get service log entries
	///
	/// https://api.aiven.io/doc/#api-Service-ProjectGetServiceLogs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let json_body = json!({
	/// "limit": 100,
	/// "offset": "23425325",
	/// "sort_order": "asc"
	/// });
	/// let response = client
	///         .service()
	///         .get_log_entries("project", "service_name", &json_body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_log_entries<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<ResLogs, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/logs",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		Ok(
			make_json_request!(self, reqwest::Method::GET, &url, json_body)?
				.json()
				.await?,
		)
	}

	/// Get task result
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceTaskGet
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///         .service()
	///         .get_task_result("project", "service_name", "task_id")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_task_result(
		&self,
		project: &str,
		service_name: &str,
		task_id: &str,
	) -> Result<ResTask, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/task/{task_id}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			task_id = encode_param(task_id),
		);
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// List publicly available service types
	///
	/// https://api.aiven.io/doc/#api-Service-ListPublicServiceTypes
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///         .service()
	///         .list_public_service_types()
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_public_service_types(&self) -> Result<ResServiceTypes, AivenError> {
		let url = "service_types";
		Ok(make_request!(self, reqwest::Method::GET, url)?
			.json()
			.await?)
	}
	/// List service types for a project
	///
	/// https://api.aiven.io/doc/#api-Service-ListProjectServiceTypes
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///         .service()
	///         .list_service_types("project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_service_types(&self, project: &str) -> Result<ResServiceTypes, AivenError> {
		let url = format!(
			"project/{project}/service_types",
			project = encode_param(project)
		);
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}
	/// List service databases
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceDatabaseList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///   .service()
	///   .list_service_databases("my-project", "my-service-name")
	///   .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_service_databases(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResDatabaseNames, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/db",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// List active alerts for service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceAlertsList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///         .service()
	///         .list_active_alerts("project-name", "service-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_active_alerts(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResAlerts, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/alerts",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);

		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// List services
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "some-token");
	/// let response = client
	///         .service()
	///         .list_services("project-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_services(&self, project: &str) -> Result<ResServices, AivenError> {
		let url = format!("project/{project}/service", project = encode_param(project));
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// Modify service user credentials
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceUserCredentialsModify
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .modify_service_user_credential("my-project",
	///         "my-service-name",
	///         "service-user-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn modify_service_user_credential(
		&self,
		project: &str,
		service_name: &str,
		service_username: &str,
	) -> Result<ResService, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/user/{service_username}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			service_username = encode_param(service_username),
		);
		Ok(make_request!(self, reqwest::Method::PUT, &url)?
			.json()
			.await?)
	}

	/// Reset service user credentials
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceUserCredentialsReset
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .reset_service_user_credential("my-project", "my-service-name", "service-user-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn reset_service_user_credential(
		&self,
		project: &str,
		service_name: &str,
		service_username: &str,
	) -> Result<ResService, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/user/{service_username}/credentials/reset",
			project = encode_param(project),
			service_name = encode_param(service_name),
			service_username = encode_param(service_username),
		);
		Ok(make_request!(self, reqwest::Method::PUT, &url)?
			.json()
			.await?)
	}

	/// Reset service's query statistics
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceQueryStatisticsReset
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .reset_query_stats("my-project", "my-service-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn reset_query_stats(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResResetQueryStats, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/query/stats/reset",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		Ok(make_request!(self, reqwest::Method::PUT, &url)?
			.json()
			.await?)
	}

	/// Retrieve a service CA
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceKmsGetCA
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .get_service_ca("my-project", "my-service-name", "ca-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_service_ca(
		&self,
		project: &str,
		service_name: &str,
		ca_name: &str,
	) -> Result<ResServiceCA, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kms/ca/{ca_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			ca_name = encode_param(ca_name),
		);
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// Retrieve service keypair
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceKmsGetKeypair
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .get_service_keypair("my-project", "my-service-name", "keypairname")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_service_keypair(
		&self,
		project: &str,
		service_name: &str,
		keypair_name: &str,
	) -> Result<ResServiceKeyPair, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/kms/keypairs/{keypair_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			keypair_name = encode_param(keypair_name),
		);
		Ok(make_request!(self, reqwest::Method::GET, &url)?
			.json()
			.await?)
	}

	/// Start maintenance updates
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceMaintenanceStart
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .start_maintenance_updates("my-project", "my-service-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn start_maintenance_updates(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/maintenance/start",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);

		let _response = make_request!(self, reqwest::Method::PUT, &url)?;

		Ok(())
	}

	/// Temporarily enable writes for a service in read-only mode.
	/// Will only work if disk usage is lower than 99.0%
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceEnableWrites
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .enable_writes("my-project", "my-service-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn enable_writes(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ResEnableWrites, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/enable-writes",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);

		let response = make_request!(self, reqwest::Method::POST, &url)?;

		Ok(response.json().await?)
	}

	/// Update service configuration
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceUpdate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "token");
	/// let body = json!({
	/// "cloud": "aws-eu-central-1",
	/// "group_name": "mygroup",
	/// "maintenance": {
	/// "dow": "sunday",
	/// "time": "12:30:00"
	/// },
	/// "plan": "hobbyist",
	/// "powered": true,
	/// "project_vpc_id": "1007a317-aa2a-4fb4-9056-93924c5ee46f",
	/// "termination_protection": true,
	/// "user_config": {}
	/// });
	/// let response = client
	///         .service()
	///         .update_configuration("my-project", "my-service-name", &body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_configuration<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<ResService, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		Ok(
			make_json_request!(self, reqwest::Method::PUT, &url, json_body)?
				.json()
				.await?,
		)
	}

	/// Terminate a service
	///
	/// https://api.aiven.io/doc/#api-Service-ServiceDelete
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .service()
	///         .terminate("my-project", "my-service-name")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn terminate(&self, project: &str, service_name: &str) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);

		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::testutil;
	use serde_json::json;

	#[tokio::test]
	async fn test_service_create_user() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/user";
		let test_data = testutil::get_test_data("tests/testdata/service/service/create_user.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let body = json!({
			"authentication": "caching_sha2_password",
			"username": "testuser"
		});
		match client
			.service()
			.create_user("myproject", "myservice", &body)
			.await
		{
			Ok(response) => assert!(response.user.username == "testuser"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_create_logical_database() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/db";
		let test_data = "".to_string();
		// testutil::get_test_data("tests/testdata/service/service/
		// create_logical_database.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let body = json!({
			"database": "testdb",
			"lc_collate": "en_US.UTF-8",
			"lc_ctype": "en_US.UTF-8"
		});
		match client
			.service()
			.create_logical_database("myproject", "myservice", &body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_create_new_task() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/task";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/create_new_task.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let body = json!({
			"target_version": "10",
			"task_type": "upgrade_check"
		});
		match client
			.service()
			.create_new_task("myproject", "myservice", &body)
			.await
		{
			Ok(response) => assert!(response.task.success, format!("{:?}", response)),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_create_service() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/create_service.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let body = json!({
			"cloud": "aws-eu-central-1",
			"group_name": "mygroup",
			"maintenance": {
				"dow": "sunday",
				"time": "12:30:00"
			},
			"plan": "hobbyist",
			"project_vpc_id": "1007a317-aa2a-4fb4-9056-93924c5ee46f",
			"service_integrations": [
				[
					{
						"integration_type": "read_replica",
						"source_service": "myservice"
					}
				]
			],
			"service_name": "db123",
			"service_type": "pg",
			"termination_protection": true,
			"user_config": {}
		});
		match client.service().create_service("myproject", &body).await {
			Ok(response) => assert!(response.service.acl.unwrap().len() > 0),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_cancel_query() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/query/cancel";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/cancel_specified_query.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let body = json!({
			"pid": 84562,
			"terminate": "true"
		});
		match client
			.service()
			.cancel_query("myproject", "myservice", &body)
			.await
		{
			Ok(response) => {
				assert!(response.success, format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_delete_logical_db() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/db/mydb";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		match client
			.service()
			.delete_logical_db("myproject", "myservice", "mydb")
			.await
		{
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_delete_user() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/user/myserviceuser";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		match client
			.service()
			.delete_user("myproject", "myservice", "myserviceuser")
			.await
		{
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_fetch_current_queries() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/query/activity";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/fetch_current_queries.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let body = json!({
			"limit": 100,
			"offset": 100,
			"order_by": "client_id:desc"
		});
		match client
			.service()
			.fetch_current_queries("myproject", "myservice", &body)
			.await
		{
			Ok(response) => {
				assert!(response.queries.len() > 0);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_fetch_service_metrics() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/metrics";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/fetch_service_metrics.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		// let body = json!({
		// 	"period": "hour"
		// });

		let body = json!({});
		match client
			.service()
			.fetch_service_metrics("myproject", "myservice", &body)
			.await
		{
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_get_user_details() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/user/myserviceuser";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/get_user_details.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service()
			.get_user_details("myproject", "myservice", "myserviceuser")
			.await
		{
			Ok(response) => {
				assert!(response.user.account_type == "primary");
				assert!(response.user.username == "testuser");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_get_service_info() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/get_service_info.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service()
			.get_service_info("myproject", "myservice")
			.await
		{
			Ok(response) => {
				assert!(response.service.group_list.len() > 0);
				assert!(response.service.group_list[0] == "mygroup");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_get_log_entries() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/logs";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/get_log_entries.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let body = json!({
			"limit": 100,
			"offset": "23425325",
			"sort_order": "asc"
		});
		match client
			.service()
			.get_log_entries("myproject", "myservice", &body)
			.await
		{
			Ok(response) => {
				assert!(response.first_log_offset == "0");
				assert!(!response.logs.is_empty());
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_get_task_result() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/task/mytask";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/get_task_result.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service()
			.get_task_result("myproject", "myservice", "mytask")
			.await
		{
			Ok(response) => {
				assert!(response.task.success);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_list_active_alerts() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/alerts";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/list_active_alerts.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service()
			.list_active_alerts("myproject", "myservice")
			.await
		{
			Ok(response) => {
				assert!(response.alerts.len() > 0);
				assert!(response.alerts[0].event == "disk_usage");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_list_service_databases() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/db";
		let test_data = r#"
                        {
                            "databases": [
                                {
                                    "database_name": "defaultdb"
                                }
                            ]
                        }
                        "#;
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service()
			.list_service_databases("myproject", "myservice")
			.await
		{
			Ok(response) => {
				assert!(response.databases.len() > 0);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_list_service_types() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service_types";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/service/list_service_types_project.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.service().list_service_types("myproject").await {
			Ok(response) => {
				assert!(response.service_types.contains_key("ANY"));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_list_public_service_types() {
		let client = testutil::prepare_test_client();
		let query_url = "/service_types";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/service/list_public_service_types.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.service().list_public_service_types().await {
			Ok(response) => {
				assert!(response.service_types.contains_key("ANY"));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_list_services() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service";
		let test_data =
			testutil::get_test_data("tests/testdata/service/service/list_services.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.service().list_services("myproject").await {
			Ok(response) => {
				assert!(!response.services.is_empty());
				assert!(response.services[0].group_list[0] == "mygroup");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_modify_service_user_credential() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/user/myserviceusername";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/service/modify_service_user_credential.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		match client
			.service()
			.modify_service_user_credential("myproject", "myservice", "myserviceusername")
			.await
		{
			Ok(response) => {
				assert!(!response.service.group_list.is_empty());
				assert!(response.service.group_list[0] == "mygroup");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_reset_service_user_credential() {
		let client = testutil::prepare_test_client();
		let query_url =
			"/project/myproject/service/myservice/user/myserviceusername/credentials/reset";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/service/reset_service_user_credential.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		match client
			.service()
			.reset_service_user_credential("myproject", "myservice", "myserviceusername")
			.await
		{
			Ok(response) => {
				assert!(!response.service.group_list.is_empty());
				assert!(response.service.group_list[0] == "mygroup");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_reset_query_stats() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/query/stats/reset";
		let test_response = r#"{"queries": [ {}  ]}"#;
		let _m = testutil::create_mock_server(query_url, test_response, "PUT");

		match client
			.service()
			.reset_query_stats("myproject", "myservice")
			.await
		{
			Ok(response) => {
				assert!(!response.queries.is_empty());
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_get_service_ca() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/kms/ca/myca";

		let test_response = r#"{"certificate": "-----BEGIN CERTIFICATE-----"}"#;
		let _m = testutil::create_mock_server(query_url, test_response, "GET");

		match client
			.service()
			.get_service_ca("myproject", "myservice", "myca")
			.await
		{
			Ok(response) => {
				assert!(response.certificate.contains("CERTIFICATE"));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_get_service_keypair() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/kms/keypairs/mykeypair";

		let test_response = r#"{"certificate": "-----BEGIN CERTIFICATE-----",
                                        "key": "-----BEGIN PRIVATE KEY-----"}"#;
		let _m = testutil::create_mock_server(query_url, test_response, "GET");

		match client
			.service()
			.get_service_keypair("myproject", "myservice", "mykeypair")
			.await
		{
			Ok(response) => {
				assert!(response.certificate.contains("CERTIFICATE"));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_start_maintenance_updates() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/maintenance/start";
		let _m = testutil::create_mock_server(query_url, "", "PUT");

		match client
			.service()
			.start_maintenance_updates("myproject", "myservice")
			.await
		{
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_service_enable_writes() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/enable-writes";

		let test_response = r#"{"until": "2016-08-12T14:21:25.334013+00:00"}"#;
		let _m = testutil::create_mock_server(query_url, test_response, "POST");

		match client
			.service()
			.enable_writes("myproject", "myservice")
			.await
		{
			Ok(response) => {
				assert!(response.until == "2016-08-12T14:21:25.334013+00:00");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_terminate() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		match client.service().terminate("myproject", "myservice").await {
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_service_update_configuration() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice";
		let test_data = testutil::get_test_data(
			"tests/testdata/service/service/update_service_configuration.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");
		let body = json!({
			"cloud": "aws-eu-central-1",
			"group_name": "mygroup",
			"maintenance": {
				"dow": "sunday",
				"time": "12:30:00"
			},
			"plan": "hobbyist",
			"powered": true,
			"project_vpc_id": "1007a317-aa2a-4fb4-9056-93924c5ee46f",
			"termination_protection": true,
			"user_config": {}
		});
		match client
			.service()
			.update_configuration("myproject", "myservice", &body)
			.await
		{
			Ok(response) => {
				assert!(
					response.service.cloud_name == "aws-eu-central-1",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
