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
	key_mgmt::types,
	make_request,
	response::APIResponse,
};

pub struct ProjectKeyManagementApi {
	http_client: HTTPClient,
}
impl ProjectKeyManagementApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Claim a credit code
	///
	/// https://api.aiven.io/doc/#api-Project_Billing-ProjectCreditsClaim
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `code` - Credit code
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .project_key_management()
	///         .retrieve_ca_cert("my-project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn retrieve_ca_cert(
		&self,
		project: &str,
	) -> Result<types::ResCertificate, AivenError> {
		let url = format!("project/{project}/kms/ca", project = encode_param(project));
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}
}
#[cfg(test)]
mod tests {
	use crate::testutil;

	#[tokio::test]
	async fn test_key_mgmt_retrieve_ca_cert() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/kms/ca";
		let test_data = testutil::get_test_data("tests/testdata/key_mgmt/retrieve_ca_cert.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.project_key_management()
			.retrieve_ca_cert("myproject")
			.await
		{
			Ok(response) => {
				assert!(response.certificate.contains("CERTIFICATE"));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
