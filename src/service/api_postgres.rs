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
	service::types_postgres::*,
};
use serde::Serialize;
use std::collections::HashMap;
pub struct ServicePostgresApi {
	http_client: HTTPClient,
}

impl ServicePostgresApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Update an existing access token.
	///
	/// https://api.aiven.io/doc/#api-User-AccessTokenUpdate
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
	/// let body = json!({
	///  "database": "testdb",
	///  "pool_mode": "session",
	///  "pool_name": "mypool-x-y-z",
	///  "pool_size": 50,
	///  "username": "testuser"
	/// });
	/// let response = client
	///         .service_postgres()
	///         .create_pool("token-prefix", "some-description", &body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_pool<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		body: &T,
	) -> Result<(), AivenError> {
		let url = &format!(
			"project/{project}/service/{service_name}/connection_pool",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let _response = make_json_request!(self, reqwest::Method::POST, url, body)?;
		Ok(())
	}

	/// Delete a connection pool
	///
	/// https://api.aiven.io/doc/#api-Service_-_PostgreSQL-ServicePGBouncerDelete
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `service_name` - Service name
	/// * `pool_name` - PgBouncer connection pool name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_postgres()
	///         .delete_pool("project","service", "poolname")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_pool(
		&self,
		project: &str,
		service_name: &str,
		pool_name: &str,
	) -> Result<(), AivenError> {
		let url = &format!(
			"project/{project}/service/{service_name}/connection_pool/{pool_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			pool_name = encode_param(pool_name),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, url)?;
		Ok(())
	}

	/// Fetch PostgreSQL service query statistics
	///
	/// https://api.aiven.io/doc/#api-Service_-_PostgreSQL-PGServiceQueryStatistics
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
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	///
	/// let mut json_body: HashMap<&str, String> = HashMap::new();
	/// json_body.insert("limit", "100".into());
	/// json_body.insert("offset", "100".into());
	/// json_body.insert("order_by", "calls:desc,total_time:asc".into());
	///
	/// let response = client
	///         .service_postgres()
	///         .fetch_query_stats("token-prefix", "some-description", &json_body)
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn fetch_query_stats(
		&self,
		project: &str,
		service_name: &str,
		json_body: &HashMap<&str, String>,
	) -> Result<ResPostgresQueriesStats, AivenError> {
		let url = &format!(
			"project/{project}/service/{service_name}/pg/query/stats",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Update a connection pool
	///
	/// https://api.aiven.io/doc/#api-Service_-_PostgreSQL-ServicePGBouncerUpdate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `service_name` - Service name
	/// * `pool_name` - PgBouncer connection pool name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	///
	/// let json_body = json!({
	/// "database": "testdb",
	/// "pool_mode": "session",
	/// "pool_size": 50,
	/// "username": "testuser"
	/// });
	/// let response = client
	///         .service_postgres()
	///         .update_pool("token-prefix", "some-description", "pool-name", &json_body)
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn update_pool<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		pool_name: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = &format!(
			"project/{project}/service/{service_name}/connection_pool/{pool_name}",
			project = encode_param(project),
			service_name = encode_param(service_name),
			pool_name = encode_param(pool_name),
		);
		let _response = make_json_request!(self, reqwest::Method::PUT, url, json_body)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil;
	use serde_json::json;

	#[tokio::test]
	async fn test_postgres_create_pool() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/connection_pool";
		let _m = testutil::create_mock_server(query_url, "", "POST");

		let body = json!({
			"database": "testdb",
			"pool_mode": "session",
			"pool_name": "mypool-x-y-z",
			"pool_size": 50,
			"username": "testuser"
		});
		match client
			.service_postgres()
			.create_pool("myproject", "myservice", &body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_postgres_delete_pool() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/connection_pool/mypoolname";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		match client
			.service_postgres()
			.delete_pool("myproject", "myservice", "mypoolname")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_postgres_fetch_query_stats() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/pg/query/stats";

		let data = r#"{"queries": [{}]}"#;
		let _m = testutil::create_mock_server(query_url, data, "POST");

		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("limit", "100".into());
		json_body.insert("offset", "100".into());
		json_body.insert("order_by", "calls:desc,total_time:asc".into());

		match client
			.service_postgres()
			.fetch_query_stats("myproject", "myservice", &json_body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_postgres_update_pool() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/connection_pool/mypool";

		let _m = testutil::create_mock_server(query_url, "", "PUT");

		let json_body = json!({
			"database": "testdb",
			"pool_mode": "session",
			"pool_size": 50,
			"username": "testuser"
		});
		match client
			.service_postgres()
			.update_pool("myproject", "myservice", "mypool", &json_body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
