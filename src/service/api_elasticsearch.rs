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
	service::types_elasticsearch::*,
};
pub struct ServiceElastiSearchApi {
	http_client: HTTPClient,
}

impl ServiceElastiSearchApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Describe something here...
	///
	/// # Arguments
	///
	/// * `Arg1` -
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// // Your example here
	/// ```
	/// Delete an Elasticsearch index
	///
	/// https://api.aiven.io/doc/#api-Service_-_Elasticsearch
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .service_elasticsearch()
	///         .delete_index("project-name", "service-name", "index-to-delete")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_index(
		&self,
		project: &str,
		service_name: &str,
		index_name: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/index/{index_name}",
			project = encode_param(project),
			index_name = encode_param(index_name),
			service_name = encode_param(service_name),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	pub async fn list_indexes(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<Indexes, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/index",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	pub async fn set_acl_configuration(
		&self,
		project: &str,
		service_name: &str,
		acl_config: &ElasticSearchACLConfig,
	) -> Result<ElasticSearchACLConfig, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/elasticsearch/acl",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, acl_config)?;
		Ok(response.json().await?)
	}

	pub async fn show_acl_configuration(
		&self,
		project: &str,
		service_name: &str,
	) -> Result<ElasticSearchACLConfig, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/elasticsearch/acl",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	pub async fn update_acl_configuration(
		&self,
		project: &str,
		service_name: &str,
		acl_config: &ElasticSearchACLConfig,
	) -> Result<ElasticSearchACLConfig, AivenError> {
		let url = format!(
			"project/{project}/service/{service_name}/elasticsearch/acl",
			project = encode_param(project),
			service_name = encode_param(service_name),
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, acl_config)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil;

	#[tokio::test]
	async fn test_es_list_indexes() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/index";
		let test_data = testutil::get_test_data("tests/testdata/service/elasticsearch/list.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_elasticsearch()
			.list_indexes("myproject", "myservice")
			.await
		{
			Ok(response) => {
				assert!(response.indexes[0].docs == 5019, format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_es_set_acl_configuration() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/elasticsearch/acl";
		let test_data =
			testutil::get_test_data("tests/testdata/service/elasticsearch/set_acl_config.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let wanted_config = ElasticSearchACLConfig {
			elasticsearch_acl_config: ElasticSearchConfig {
				acls: vec![],
				enabled: Some(true),
			},
		};
		match client
			.service_elasticsearch()
			.set_acl_configuration("myproject", "myservice", &wanted_config)
			.await
		{
			Ok(response) => {
				assert!(
					response.elasticsearch_acl_config.enabled == Some(false),
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_es_show_acl_configuration() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/elasticsearch/acl";
		let test_data =
			testutil::get_test_data("tests/testdata/service/elasticsearch/show_acl_config.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.service_elasticsearch()
			.show_acl_configuration("myproject", "myservice")
			.await
		{
			Ok(response) => {
				assert!(
					response.elasticsearch_acl_config.enabled == Some(false),
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_es_update_acl_configuration() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/service/myservice/elasticsearch/acl";
		let test_data =
			testutil::get_test_data("tests/testdata/service/elasticsearch/update_acl_config.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let wanted_config = ElasticSearchACLConfig {
			elasticsearch_acl_config: ElasticSearchConfig {
				acls: vec![],
				enabled: Some(true),
			},
		};
		match client
			.service_elasticsearch()
			.update_acl_configuration("myproject", "myservice", &wanted_config)
			.await
		{
			Ok(response) => {
				assert!(
					response.elasticsearch_acl_config.enabled == Some(false),
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
