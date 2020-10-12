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

use crate::{client::HTTPClient, errors::AivenError, make_request};

use crate::cloud::types;
pub struct CloudApi {
	http_client: HTTPClient,
}

impl CloudApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}
	/// List available cloud platforms by project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .cloud()
	///         .list_by_project("my-project").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_by_project(&self, project: &str) -> Result<types::ResClouds, AivenError> {
		let url = &format!("project/{}/clouds", project);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List all available cloud platforms
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .cloud()
	///         .list_all().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_all(&self) -> Result<types::ResClouds, AivenError> {
		let url = "clouds";
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::testutil;

	#[tokio::test]
	async fn test_cloud_list_all() {
		let client = testutil::prepare_test_client();
		let query_url = "/clouds";
		let test_data = testutil::get_test_data("tests/testdata/cloud/list.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.cloud().list_all().await {
			Ok(response) => {
				assert!(response.clouds.len() > 0, format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_cloud_list_by_project() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/my-project/clouds";
		let test_data = testutil::get_test_data("tests/testdata/cloud/list_by_project.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.cloud().list_by_project("my-project").await {
			Ok(response) => {
				assert!(response.clouds.len() > 0, format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
