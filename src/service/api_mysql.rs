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
	make_json_request,
	service::types_mysql::*,
};
use serde::Serialize;

pub struct ServiceMysqlApi {
	http_client: HTTPClient,
}

impl ServiceMysqlApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Fetch MySQL service query statistics
	///
	/// https://api.aiven.io/doc/#operation/MySQLServiceQueryStatistics
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
	///         .service_mysql()
	///         .fetch_query_stats("token-prefix", "some-description", &json_body)
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn fetch_query_stats<T: Serialize + ?Sized>(
		&self,
		project: &str,
		service_name: &str,
		json_body: &T,
	) -> Result<ResMySqlQueriesStats, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/mysql/query/stats",
			project = encode_param(project),
			service_name = encode_param(service_name)
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::testutil;
	use std::collections::HashMap;
	#[tokio::test]
	async fn test_mysql_fetch_query_stats() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/mysql/query/stats";

		let data = r#"{"queries": [{}]}"#;
		let _m = testutil::create_mock_server(query_url, data, "POST");

		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("limit", "100".into());
		json_body.insert("offset", "100".into());
		json_body.insert("order_by", "calls:desc,total_time:asc".into());

		match client
			.service_mysql()
			.fetch_query_stats("myproject", "myservice", &json_body)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
