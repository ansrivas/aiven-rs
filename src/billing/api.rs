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
	billing::types,
	client::{encode_param, HTTPClient},
	errors::AivenError,
	make_json_request, make_request,
};
use bytes::Bytes;
use std::collections::HashMap;

pub struct ProjectBillingApi {
	http_client: HTTPClient,
}

impl ProjectBillingApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Claim a credit code
	///
	/// https://api.aiven.io/doc/#operation/ProjectCreditsClaim
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
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let project_billing = client.project_billing();
	/// let response = project_billing.claim_credit_code("project", "code").await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn claim_credit_code(
		&self,
		project: &str,
		code: &str,
	) -> Result<types::ResCredit, AivenError> {
		let url = format!("project/{project}/credits", project = encode_param(project));
		let mut json_body = HashMap::new();
		json_body.insert("code", code.to_string());
		let data = &json_body;
		let response = make_json_request!(self, reqwest::Method::POST, &url, data)?;
		Ok(response.json().await?)
	}

	/// Download PDF invoice
	///
	/// https://api.aiven.io/doc/#operation/ProjectInvoiceGet
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `invoice_number` - Credit code
	/// * `download_cookie` - Authentication cookie for invoice download
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::{fs::File, io::prelude::*};
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let project_billing = client.project_billing();
	/// let response = project_billing
	///   .download_pdf_invoice("project", "invoice", "download-cookie")
	///   .await?;
	/// let mut file = File::create("foo.pdf")?;
	/// file.flush()?;
	/// file.write_all(&response[..])?;
	/// Ok(())
	/// }
	/// ```
	pub async fn download_pdf_invoice(
		&self,
		project: &str,
		invoice_number: &str,
		download_cookie: &str,
	) -> Result<Bytes, AivenError> {
		let url = format!(
			"project/{project}/invoice/{invoice_number}/{download_cookie}",
			project = encode_param(project),
			invoice_number = encode_param(invoice_number),
			download_cookie = encode_param(download_cookie),
		);
		let response: reqwest::Response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.bytes().await?)
	}

	/// List project credits
	///
	/// https://api.aiven.io/doc/#operation/ProjectCreditsList
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let project_billing = client.project_billing();
	/// let response = project_billing.list_project_credits("project").await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_project_credits(&self, project: &str) -> Result<types::Credits, AivenError> {
		let url = format!("project/{project}/credits", project = encode_param(project),);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List project invoices
	///
	/// https://api.aiven.io/doc/#operation/ProjectInvoiceList
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let project_billing = client.project_billing();
	/// let response = project_billing.list_project_invoices("project").await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_project_invoices(
		&self,
		project: &str,
	) -> Result<types::Invoices, AivenError> {
		let url = format!("project/{project}/invoice", project = encode_param(project),);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::testutil;

	#[tokio::test]
	async fn test_project_billing_claim_credit_code() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/credits";

		let test_data =
			testutil::get_test_data("tests/testdata/project_billing/claim_credit_code.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.project_billing()
			.claim_credit_code("myproject", "credit-code")
			.await
		{
			Ok(resp) => assert!(resp.credit.code == "AVN2015"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_billing_list_project_credits() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/credits";

		let test_data =
			testutil::get_test_data("tests/testdata/project_billing/list_project_credits.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.project_billing()
			.list_project_credits("myproject")
			.await
		{
			Ok(resp) => assert!(resp.credits.len() > 0),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_billing_list_project_invoices() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/invoice";

		let test_data =
			testutil::get_test_data("tests/testdata/project_billing/list_project_invoices.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.project_billing()
			.list_project_invoices("myproject")
			.await
		{
			Ok(resp) => assert!(resp.invoices.len() > 0),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_billing_download_pdf_invoice() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/invoice/invoicenumber/f5ffd98ce948c517e1";

		let test_data =
			testutil::get_test_data("tests/testdata/project_billing/download_pdf_invoice.txt");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.project_billing()
			.download_pdf_invoice("myproject", "invoicenumber", "f5ffd98ce948c517e1")
			.await
		{
			Ok(response) => {
				assert!(&response[..] == b"somedata", format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
