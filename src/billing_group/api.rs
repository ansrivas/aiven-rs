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
	billing_group::types,
	client::{encode_param, HTTPClient},
	errors::AivenError,
	make_json_request, make_request,
};
use bytes::Bytes;
use serde::Serialize;
use std::iter::IntoIterator;

pub struct BillingGroupApi {
	http_client: HTTPClient,
}

impl BillingGroupApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Create a billing group
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupCreate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let mut body = json!({
	///         "account_id": "John Doe",
	///         "address_lines": "43",
	///         "billing_emails": [
	///             {"email": "testuser"},
	///             {"email": "testuser2"}
	///         ]
	///     });
	/// // check rest of the json body from the API doc above
	/// let response = client
	///         .billing_group()
	///         .create(&body).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn create<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<types::ResponseBillingGroup, AivenError> {
		let url = "/billing-group";
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// List billing groups
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .billing_group()
	///         .list().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list(&self) -> Result<types::ResponseBillingGroups, AivenError> {
		let url = "/billing-group";
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Claim a credit code
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupCreditsClaim
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .billing_group()
	///         .claim_credit_code("billing-group-id", "credit-code").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn claim_credit_code(
		&self,
		billing_group_id: &str,
		credit_code: &str,
	) -> Result<types::ResponseClaimCredit, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/credits",
			billing_group = encode_param(billing_group_id)
		);

		let body = &serde_json::json!({
			"code": credit_code,
		});
		let response = make_json_request!(self, reqwest::Method::POST, url, body)?;
		Ok(response.json().await?)
	}

	/// List billing group credits
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupCreditsList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .billing_group()
	///         .list_billing_group_credits("billing-group-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_billing_group_credits(
		&self,
		billing_group_id: &str,
	) -> Result<types::ResponseCredits, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/credits",
			billing_group = encode_param(billing_group_id)
		);

		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Delete billing group
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupDelete
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .billing_group()
	///         .delete("billing-group-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn delete(&self, billing_group_id: &str) -> Result<(), AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param(billing_group_id)
		);

		let _response = make_request!(self, reqwest::Method::DELETE, url)?;
		Ok(())
	}

	/// Get billing group details
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupGet
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .billing_group()
	///         .details("billing-group-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn details(
		&self,
		billing_group_id: &str,
	) -> Result<types::ResponseBillingGroup, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param(billing_group_id)
		);

		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Update billing group
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupUpdate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// // Rest of the body can be checked from the link above.
	/// let body = json!({
	/// "account_id": "string",
	/// "address_lines": [
	/// "string"],
	/// });
	/// let response = client
	///         .billing_group()
	///         .update("billing-group-id", &body).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn update<T: Serialize + ?Sized>(
		&self,
		billing_group_id: &str,
		json_body: &T,
	) -> Result<types::ResponseBillingGroup, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param(billing_group_id)
		);

		let response = make_json_request!(self, reqwest::Method::PUT, url, json_body)?;
		Ok(response.json().await?)
	}

	/// List billing group events
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupEventList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .billing_group()
	///         .list_events("billing-group-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_events(
		&self,
		billing_group_id: &str,
	) -> Result<types::ResponseEvents, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param(billing_group_id)
		);

		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Download PDF invoice
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupInvoiceDownload
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use std::collections::HashMap;
	/// use tokio::fs::File;
	/// use tokio::io::AsyncWriteExt;
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .billing_group()
	///         .download_invoice("billing-group-id", "invoice-num", "download-cookie").await?;
	/// let mut file = File::create("foo.pdf").await?;
	/// file.write_all(&response[..]).await?;
	/// file.sync_all().await?;
	/// Ok(())
	/// # }
	/// ```
	pub async fn download_invoice(
		&self,
		billing_group_id: &str,
		invoice_number: &str,
		download_cookie: &str,
	) -> Result<Bytes, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/invoice/{invoice_number}/{download_cookie}",
			billing_group = encode_param(billing_group_id),
			invoice_number = encode_param(invoice_number),
			download_cookie = encode_param(download_cookie)
		);

		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.bytes().await?)
	}

	/// Get a single invoice
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupInvoiceGet
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use tokio::io::AsyncWriteExt;
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .billing_group()
	///         .get_invoice("billing-group-id", "invoice-num").await?;
	/// Ok(())
	/// # }
	/// ```
	pub async fn get_invoice(
		&self,
		billing_group_id: &str,
		invoice_number: &str,
	) -> Result<types::ResponseInvoice, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/invoice/{invoice_number}",
			billing_group = encode_param(billing_group_id),
			invoice_number = encode_param(invoice_number),
		);

		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Get invoice lines for a single invoice
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupInvoiceLinesList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// use tokio::io::AsyncWriteExt;
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .billing_group()
	///         .get_invoice_lines("billing-group-id", "invoice-num").await?;
	/// Ok(())
	/// # }
	/// ```
	pub async fn get_invoice_lines(
		&self,
		billing_group_id: &str,
		invoice_number: &str,
	) -> Result<types::ResponseInvoiceLines, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/invoice/{invoice_number}/lines",
			billing_group = encode_param(billing_group_id),
			invoice_number = encode_param(invoice_number),
		);

		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Get invoices generated for billing group
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupInvoiceList
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
	///         .billing_group()
	///         .get_invoices("billing-group-id").await?;
	/// Ok(())
	/// # }
	/// ```
	pub async fn get_invoices(
		&self,
		billing_group_id: &str,
	) -> Result<types::ResponseInvoiceBillingGroup, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/invoice",
			billing_group = encode_param(billing_group_id),
		);

		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Assign project to billing group
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupProjectAssign
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
	///         .billing_group()
	///         .assign_project("billing-group-id", "project-name").await?;
	/// Ok(())
	/// # }
	/// ```
	pub async fn assign_project(
		&self,
		billing_group_id: &str,
		project_name: &str,
	) -> Result<(), AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/project-assign/{project}",
			billing_group = encode_param(billing_group_id),
			project = encode_param(project_name),
		);

		let _response = make_request!(self, reqwest::Method::POST, url)?;
		Ok(())
	}

	/// Get projects assigned to billing group
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupProjectList
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
	///         .billing_group()
	///         .get_associated_projects("billing-group-id").await?;
	/// Ok(())
	/// # }
	/// ```
	pub async fn get_associated_projects(
		&self,
		billing_group_id: &str,
	) -> Result<types::ResponseAssociatedProjects, AivenError> {
		let url = &format!(
			"/billing-group/{billing_group}/projects",
			billing_group = encode_param(billing_group_id),
		);

		let response = make_request!(self, reqwest::Method::POST, url)?;
		Ok(response.json().await?)
	}

	/// Assign projects to billing group
	///
	/// https://api.aiven.io/doc/#operation/BillingGroupProjectsAssign
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
	/// let projects = vec!["project1", "project2"];
	/// let response = client
	///         .billing_group()
	///         .assign_projects("billing-group-id", projects ).await?;
	/// Ok(())
	/// # }
	/// ```
	pub async fn assign_projects<T, I>(
		&self,
		billing_group_id: &str,
		projects: I,
	) -> Result<(), AivenError>
	where
		I: IntoIterator<Item = T>,
		T: Into<String>,
	{
		let url = &format!(
			"/billing-group/{billing_group}/projects-assign",
			billing_group = encode_param(billing_group_id),
		);

		let projects: Vec<String> = projects.into_iter().map(Into::into).collect();
		let body = &serde_json::json!({
			"projects_names": projects,
		});
		let _response = make_json_request!(self, reqwest::Method::POST, url, body)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil;
	use serde_json::json;

	#[tokio::test]
	async fn test_billing_group_create() {
		let client = testutil::prepare_test_client();
		let query_url = "/billing-group";
		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/create_billing_group.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json_body = json!({
			"account_id": "string",
			"address_lines":[
				"string"
			]
		});
		match client.billing_group().create(&json_body).await {
			Ok(response) => {
				assert!(
					response.billing_group.account_id == "some-unique-accountid",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_list() {
		let client = testutil::prepare_test_client();
		let query_url = "/billing-group";
		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/list_billing_groups.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.billing_group().list().await {
			Ok(response) => {
				assert!(
					response.billing_groups[0].account_id == "some-unique-accountid",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_claim_credit_code() {
		let client = testutil::prepare_test_client();
		let query_url = &format!(
			"/billing-group/{billing_group}/credits",
			billing_group = encode_param("my-billing-group")
		);
		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/claim_credit_code.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.billing_group()
			.claim_credit_code("my-billing-group", "unique-credit-code")
			.await
		{
			Ok(response) => {
				assert!(
					response.credit.code == "unique-code",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_list_billing_group_credits() {
		let client = testutil::prepare_test_client();
		let query_url = &format!(
			"/billing-group/{billing_group}/credits",
			billing_group = encode_param("my-billing-group")
		);
		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/list_billing_group_credits.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.billing_group()
			.list_billing_group_credits("my-billing-group")
			.await
		{
			Ok(response) => {
				assert!(
					response.credits[0].code == "unique-code",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_delete() {
		let client = testutil::prepare_test_client();
		let query_url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param("my-billing-group")
		);

		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		match client.billing_group().delete("my-billing-group").await {
			Ok(_response) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_details() {
		let client = testutil::prepare_test_client();
		let query_url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param("my-billing-group")
		);

		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/billing_group_details.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.billing_group().details("my-billing-group").await {
			Ok(response) => {
				assert!(
					response.billing_group.account_id == "unique-account-id",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_update() {
		let client = testutil::prepare_test_client();
		let query_url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param("my-billing-group")
		);

		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/billing_group_update.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let body = json!({
			"account_id": "string",
			"address_lines": [
				"string"
			],
			"billing_currency": "AUD",
			"billing_emails": [
				{
					"email": "string"
				}
			],
			"billing_extra_text": "string",
			"billing_group_name": "string",
			"card_id": "string",
			"city": "string",
			"company": "string",
			"country_code": "st",
			"state": "string",
			"vat_id": "string",
			"zip_code": "string"

		});
		match client
			.billing_group()
			.update("my-billing-group", &body)
			.await
		{
			Ok(response) => {
				assert!(
					response.billing_group.account_id == "unique-updated-account",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_list_events() {
		let client = testutil::prepare_test_client();
		let query_url = &format!(
			"/billing-group/{billing_group}",
			billing_group = encode_param("my-billing-group")
		);

		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/list_billing_group_events.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.billing_group().list_events("my-billing-group").await {
			Ok(response) => {
				assert!(
					response.events[0].actor == "unique-actor",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_download_invoice() {
		let client = testutil::prepare_test_client();
		let url = &format!(
			"/billing-group/{billing_group}/invoice/{invoice_number}/{download_cookie}",
			billing_group = encode_param("my-billing-group"),
			invoice_number = encode_param("invoice-num"),
			download_cookie = encode_param("download-cookie")
		);

		let test_data = "fake-invoice-data";
		let _m = testutil::create_mock_server(url, &test_data, "GET");

		match client
			.billing_group()
			.download_invoice("my-billing-group", "invoice-num", "download-cookie")
			.await
		{
			Ok(response) => {
				assert!(
					&response[..] == b"fake-invoice-data",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_get_invoice() {
		let client = testutil::prepare_test_client();
		let url = &format!(
			"/billing-group/{billing_group}/invoice/{invoice_number}",
			billing_group = encode_param("my-billing-group"),
			invoice_number = encode_param("invoice-num"),
		);

		let test_data = testutil::get_test_data(
			"tests/testdata/billing_group/billing_group_get_single_invoice.json",
		);
		let _m = testutil::create_mock_server(url, &test_data, "GET");

		match client
			.billing_group()
			.get_invoice("my-billing-group", "invoice-num")
			.await
		{
			Ok(response) => {
				assert!(
					response.invoice.invoice_number == "12345",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_get_invoice_lines() {
		let client = testutil::prepare_test_client();
		let url = &format!(
			"/billing-group/{billing_group}/invoice/{invoice_number}/lines",
			billing_group = encode_param("my-billing-group"),
			invoice_number = encode_param("invoice-num"),
		);

		let test_data = testutil::get_test_data(
			"tests/testdata/billing_group/billing_group_get_invoice_lines.json",
		);
		let _m = testutil::create_mock_server(url, &test_data, "GET");

		match client
			.billing_group()
			.get_invoice_lines("my-billing-group", "invoice-num")
			.await
		{
			Ok(response) => {
				assert!(
					response.lines[0].cloud_name == "aws-east",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_get_invoices() {
		let client = testutil::prepare_test_client();
		let url = &format!(
			"/billing-group/{billing_group}/invoice",
			billing_group = encode_param("my-billing-group"),
		);

		let test_data =
			testutil::get_test_data("tests/testdata/billing_group/billing_group_get_invoices.json");
		let _m = testutil::create_mock_server(url, &test_data, "GET");

		match client
			.billing_group()
			.get_invoices("my-billing-group")
			.await
		{
			Ok(response) => {
				assert!(
					response.invoices[0].currency == "AUD",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_assign_project() {
		// env_logger::init();
		let client = testutil::prepare_test_client();
		let url = &format!(
			"/billing-group/{billing_group}/project-assign/{project}",
			billing_group = encode_param("my-billing-group"),
			project = encode_param("my-project-name"),
		);

		let test_data = "";
		let _m = testutil::create_mock_server(url, test_data, "POST");

		match client
			.billing_group()
			.assign_project("my-billing-group", "my-project-name")
			.await
		{
			Ok(_response) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_get_projects() {
		let client = testutil::prepare_test_client();
		let url = &format!(
			"/billing-group/{billing_group}/projects",
			billing_group = encode_param("my-billing-group"),
		);

		let test_data = testutil::get_test_data(
			"tests/testdata/billing_group/billing_group_associated_projects.json",
		);
		let _m = testutil::create_mock_server(url, &test_data, "POST");

		match client
			.billing_group()
			.get_associated_projects("my-billing-group")
			.await
		{
			Ok(response) => {
				assert!(
					response.projects[0].available_credits == "32.5",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_billing_group_assign_projects() {
		// env_logger::init();
		let client = testutil::prepare_test_client();
		let url = &format!(
			"/billing-group/{billing_group}/projects-assign",
			billing_group = encode_param("my-billing-group"),
		);

		let test_data = "";
		let _m = testutil::create_mock_server(url, test_data, "POST");

		let projects = vec!["project1", "projec2"];
		match client
			.billing_group()
			.assign_projects("my-billing-group", projects)
			.await
		{
			Ok(_response) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
