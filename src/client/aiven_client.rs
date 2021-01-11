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
	account::AccountApi,
	billing::ProjectBillingApi,
	billing_group::BillingGroupApi,
	client::HTTPClient,
	cloud::CloudApi,
	key_mgmt::ProjectKeyManagementApi,
	payment::PaymentApi,
	project::ProjectApi,
	service::{
		ServiceApi, ServiceElastiSearchApi, ServiceIntegrationsApi, ServiceKafkaApi,
		ServiceKafkaMirrorMaker, ServiceMysqlApi, ServicePostgresApi,
	},
	ticket::TicketApi,
	user::UserApi,
};

use reqwest::header::HeaderValue;

// Little macro rule to simply create an instance of a class
// create!(self, MyClass) expands to :
// MyClass::new(self.client.clone())
macro_rules! create {
	($sel:ident,$instance:ident) => {{
		$instance::new($sel.client.clone())
		}};
}

#[derive(Debug)]
pub struct AivenClient {
	client: HTTPClient,
}

impl AivenClient {
	fn inner_client<T>(base_url: T, token: Option<T>, version: T) -> AivenClient
	where
		T: Into<String>,
	{
		let mut headers = reqwest::header::HeaderMap::new();
		headers.insert(
			"content-type",
			HeaderValue::from_str("application/json").unwrap(),
		);
		if let Some(t) = token {
			headers.insert(
				"authorization",
				HeaderValue::from_str(&format!("aivenv1 {}", &t.into())).unwrap(),
			);
		}

		// We are unwrapping here only because we want it to fail early
		let client = reqwest::ClientBuilder::new()
			.default_headers(headers)
			.build()
			.unwrap();

		AivenClient {
			client: HTTPClient::new(base_url.into(), client, version.into()),
		}
	}

	/// Create a new basic client with just url and version.
	///
	/// Use this client in situations where you don't need to use the
	/// authentication token.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// Ok(())
	/// }
	/// ```
	pub fn new<T>(base_url: T, version: T) -> AivenClient
	where
		T: Into<String>,
	{
		AivenClient::inner_client(base_url, None, version)
	}

	/// Create a new basic client with url, version and token.
	///
	/// Use this client in situations where you need to use the authentication
	/// token.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// Ok(())
	/// }
	/// ```
	pub fn from_token<T>(base_url: T, version: T, token: T) -> AivenClient
	where
		T: Into<String>,
	{
		AivenClient::inner_client(base_url, Some(token), version)
	}

	/// Access all the cloud APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let cloud_api = client.cloud();
	/// // use cloud_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn cloud(&self) -> CloudApi {
		create!(self, CloudApi)
	}

	/// Access all the user APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let user_api = client.user();
	/// // use user_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn user(&self) -> UserApi {
		create!(self, UserApi)
	}

	/// Access all the user APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let project_api = client.project();
	/// // use project_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn project(&self) -> ProjectApi {
		create!(self, ProjectApi)
	}

	/// Access all the user APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let service_api = client.service();
	/// // use service_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn service(&self) -> ServiceApi {
		create!(self, ServiceApi)
	}

	/// Access all the service integratins APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let service_integrations_api = client.service_integrations();
	/// // use service_integrations_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn service_integrations(&self) -> ServiceIntegrationsApi {
		create!(self, ServiceIntegrationsApi)
	}
	/// Access all the payment APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let payment_api = client.payment();
	/// // use payment_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn payment(&self) -> PaymentApi {
		create!(self, PaymentApi)
	}

	/// Access all the project billing APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let project_billing_api = client.project_billing();
	/// // use project_billing_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn project_billing(&self) -> ProjectBillingApi {
		create!(self, ProjectBillingApi)
	}

	/// Access all the project key management APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let project_key_management_api = client.project_key_management();
	/// // use project_key_management_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn project_key_management(&self) -> ProjectKeyManagementApi {
		create!(self, ProjectKeyManagementApi)
	}

	/// Access all the elasticsearch service APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let service_elasticsearch_api = client.service_elasticsearch();
	/// // use service_elasticsearch_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn service_elasticsearch(&self) -> ServiceElastiSearchApi {
		create!(self, ServiceElastiSearchApi)
	}

	/// Access all the kafka service APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let service_kafka_api = client.service_kafka();
	/// // use service_kafka_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn service_kafka(&self) -> ServiceKafkaApi {
		create!(self, ServiceKafkaApi)
	}

	/// Access all the kafka service APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let service_kafka_api = client.service_kafka_mirrormaker();
	/// // use service_mirrormaker_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn service_kafka_mirrormaker(&self) -> ServiceKafkaMirrorMaker {
		create!(self, ServiceKafkaMirrorMaker)
	}

	/// Access all the mysql service APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let service_mysql_api = client.service_mysql();
	/// // use service_mysql_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn service_mysql(&self) -> ServiceMysqlApi {
		create!(self, ServiceMysqlApi)
	}

	/// Access all the postgres service APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let service_postgres_api = client.service_postgres();
	/// // use service_postgres_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn service_postgres(&self) -> ServicePostgresApi {
		create!(self, ServicePostgresApi)
	}

	/// Access all customer support ticket related APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let ticket_api = client.ticket();
	/// // use ticket_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn ticket(&self) -> TicketApi {
		create!(self, TicketApi)
	}

	/// Access all account related APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let account_api = client.account();
	/// // use account_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn account(&self) -> AccountApi {
		create!(self, AccountApi)
	}

	/// Access all billing-group related APIs
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let billing_group_api = client.billing_group();
	/// // use billing_group_api from here on
	/// Ok(())
	/// }
	/// ```
	pub fn billing_group(&self) -> BillingGroupApi {
		create!(self, BillingGroupApi)
	}
}
