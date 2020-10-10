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

use crate::ticket::types::*;
use serde::Serialize;
pub struct TicketApi {
	http_client: HTTPClient,
}

impl TicketApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Create a support ticket.
	///
	/// https://api.aiven.io/doc/#operation/ProjectTicketCreate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let mut json_body: HashMap<&str, String> = HashMap::new();
	/// json_body.insert("description", "some description about ticket".to_owned());
	/// json_body.insert("service_name", "service_name".to_owned());
	/// // severity values can be critical, high or low
	/// json_body.insert("severity", "critical".to_owned());
	/// json_body.insert("title", "some short description".to_owned());
	/// let output = client.ticket().create("project-name", &json_body).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create<T: Serialize + ?Sized>(
		&self,
		project: &str,
		json_body: &T,
	) -> Result<Ticket, AivenError> {
		let url: &str = &format!("project/{project}/tickets", project = encode_param(project));
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// List support tickets.
	///
	/// https://api.aiven.io/doc/#operation/ProjectTicketList.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.ticket().get("some-project-name").await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get(&self, project: &str) -> Result<Ticket, AivenError> {
		let url: &str = &format!("project/{project}/tickets", project = encode_param(project));
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Invite a user to a ticket.
	///
	/// https://api.aiven.io/doc/#operation/ProjectTicketInvite.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.ticket().invite_user("some-project-name",
	///         "some-ticket-id",
	///         "user-email")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn invite_user(
		&self,
		project: &str,
		ticket_id: &str,
		user_email: &str,
	) -> Result<(), AivenError> {
		let url: &str = &format!(
			"/project/{project}/tickets/{ticket_id}/invite",
			project = encode_param(project),
			ticket_id = encode_param(ticket_id)
		);

		let json_body = &[("user_email", user_email.to_string())];

		let _response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil;
	use std::collections::HashMap;

	#[tokio::test]
	async fn test_ticket_create() {
		let client = testutil::prepare_test_client();

		let project_name = "my-project-name";
		let query_url: &str = &format!(
			"project/{project}/tickets",
			project = encode_param(project_name)
		);

		let test_data = testutil::get_test_data("tests/testdata/ticket/create.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let mut json_body = HashMap::new();
		json_body.insert("description", "some description about ticket".to_owned());
		json_body.insert("service_name", "service_name".to_owned());
		json_body.insert("severity", "critical".to_owned());
		json_body.insert("title", "some short description".to_owned());

		match client.ticket().create(project_name, &json_body).await {
			Ok(response) => {
				assert!(
					response.user_email == "some-email.de",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("Error during creating ticket {:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_tickets_get() {
		let client = testutil::prepare_test_client();

		let project_name = "my-project-name";
		let query_url: &str = &format!(
			"project/{project}/tickets",
			project = encode_param(project_name)
		);

		let test_data = testutil::get_test_data("tests/testdata/ticket/all_tickets.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.ticket().get(project_name).await {
			Ok(response) => {
				assert!(
					response.user_email == "some-email.de",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("Error during creating ticket {:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_tickets_invite_user() {
		let client = testutil::prepare_test_client();

		let project_name = "my-project-name";
		let ticket_id = "some-unique-id";
		let query_url: &str = &format!(
			"/project/{project}/tickets/{ticket_id}/invite",
			project = encode_param(project_name),
			ticket_id = encode_param(ticket_id),
		);

		let _m = testutil::create_mock_server(query_url, "", "POST");

		match client
			.ticket()
			.invite_user(project_name, ticket_id, "some-user-email-to-invite")
			.await
		{
			Ok(_response) => {
				assert!(true);
			}
			Err(e) => assert!(
				false,
				format!("Error during inviting user to ticket {:?}", e)
			),
		}
	}
}
