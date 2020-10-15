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
	account::types,
	client::{encode_param, HTTPClient},
	errors::AivenError,
	make_json_request, make_request,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AccountApi {
	http_client: HTTPClient,
}

impl AccountApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Create a new authentication method
	///
	/// https://api.aiven.io/doc/#operation/AccountAuthenticationMethodCreate
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
	/// let mut json = HashMap::new();
	/// json.insert("account_id", "some-account-id");
	/// json.insert("authentication_method_enabled", "true");
	/// // check rest of the json body from the API doc above
	/// let response = client
	///         .account()
	///         .create_new_auth_method("my-account-id", &json).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn create_new_auth_method<T: Serialize + ?Sized>(
		&self,
		account_id: &str,
		json_body: &T,
	) -> Result<types::AuthenticationMethodResponse, AivenError> {
		let url = &format!(
			"account/{account_id}/authentication",
			account_id = encode_param(account_id)
		);
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// List authentication methods
	///
	/// https://api.aiven.io/doc/#operation/AccountAuthenticationMethodsList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// // check rest of the json body from the API doc above
	/// let response = client
	///         .account()
	///         .list_auth_methods("my-account-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_auth_methods(
		&self,
		account_id: &str,
	) -> Result<types::AuthenticationMethodsResponse, AivenError> {
		let url = &format!(
			"account/{account_id}/authentication",
			account_id = encode_param(account_id)
		);
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Delete authentication method
	///
	/// https://api.aiven.io/doc/#operation/AccountAuthenticationMethodDelete
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// // check rest of the json body from the API doc above
	/// let response = client
	///         .account()
	///         .delete_auth_method("my-account-id", "account-method-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn delete_auth_method(
		&self,
		account_id: &str,
		account_auth_method_id: &str,
	) -> Result<(), AivenError> {
		let url = &format!(
			"account/{account_id}/authentication/{account_auth_method_id}",
			account_id = encode_param(account_id),
			account_auth_method_id = encode_param(account_auth_method_id),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, url)?;
		Ok(())
	}

	/// Get details of a single authentication method
	///
	/// https://api.aiven.io/doc/#operation/AccountAuthenticationMethodGet
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// // check rest of the json body from the API doc above
	/// let response = client
	///         .account()
	///         .get_auth_method_details("my-account-id", "account-method-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn get_auth_method_details(
		&self,
		account_id: &str,
		account_auth_method_id: &str,
	) -> Result<types::AuthenticationMethodResponse, AivenError> {
		let url = &format!(
			"account/{account_id}/authentication/{account_auth_method_id}",
			account_id = encode_param(account_id),
			account_auth_method_id = encode_param(account_auth_method_id),
		);
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}


	/// Update authentication method
	///
	/// https://api.aiven.io/doc/#operation/AccountAuthenticationMethodUpdate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// // check rest of the json body from the API doc above
	/// let response = client
	///         .account()
	///         .update_auth_method("my-account-id", "account-method-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn update_auth_method(
		&self,
		account_id: &str,
		account_auth_method_id: &str,
	) -> Result<types::AuthenticationMethodResponse, AivenError> {
		let url = &format!(
			"account/{account_id}/authentication/{account_auth_method_id}",
			account_id = encode_param(account_id),
			account_auth_method_id = encode_param(account_auth_method_id),
		);
		let response = make_request!(self, reqwest::Method::PUT, url)?;
		Ok(response.json().await?)
	}
	

	/// Create a new account
	///
	/// https://api.aiven.io/doc/#operation/AccountCreate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// # #[tokio::main]
	/// # async fn main()-> Result<(), Box<dyn std::error::Error>>{
	///
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// // check rest of the json body from the API doc above
	/// let response = client
	///         .account()
	///         .create_new("my-account-name").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn create_new(
		&self,
		account_name: &str,
	) -> Result<types::AccountCreateResponse, AivenError> {
		let url = "account";
		let json_data = &[("account_name", account_name)];
		let response = make_json_request!(self, reqwest::Method::POST, url, json_data)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::client::{encode_param, HTTPClient};
	use crate::testutil;

	#[tokio::test]
	async fn test_account_create_new_auth_method() {
		let client = testutil::prepare_test_client();
		let query_url = "/account/someaccountid/authentication";

		let test_data = testutil::get_test_data("tests/testdata/account/new_auth_method.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let data = &[("authentication_method_name", "some-method")];
		match client
			.account()
			.create_new_auth_method("someaccountid", data)
			.await
		{
			Ok(resp) => assert!(resp.authentication_method.account_id == "string"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_auth_methods() {
		let client = testutil::prepare_test_client();
		let query_url = "/account/someaccountid/authentication";

		let test_data = testutil::get_test_data("tests/testdata/account/auth_methods.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.account().list_auth_methods("someaccountid").await {
			Ok(resp) => assert!(resp.authentication_methods[0].account_id == "string"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_delete() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/authentication/{account_authentication_method_id}",
			account_id = encode_param("account_id"),
			account_authentication_method_id = encode_param("someid")
		);

		let _m = testutil::create_mock_server(&query_url, "", "DELETE");

		match client.account().delete_auth_method("account_id", "someid").await {
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_get_auth_method_details() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/authentication/{account_authentication_method_id}",
			account_id = encode_param("account_id"),
			account_authentication_method_id = encode_param("someid")
		);

		let test_data = testutil::get_test_data("tests/testdata/account/get_auth_details.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client.account().get_auth_method_details("account_id", "someid").await {
			Ok(response) => assert!(response.authentication_method.account_id == "unique-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_update_auth_method() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/authentication/{account_authentication_method_id}",
			account_id = encode_param("account_id"),
			account_authentication_method_id = encode_param("someid")
		);

		let test_data = testutil::get_test_data("tests/testdata/account/update_auth_method.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "PUT");

		match client.account().update_auth_method("account_id", "someid").await {
			Ok(response) => assert!(response.authentication_method.account_id == "updated-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_create_new() {
		let client = testutil::prepare_test_client();
		let query_url = "/account";

		let test_data = testutil::get_test_data("tests/testdata/account/create_new.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "POST");

		match client.account().create_new("account_name").await {
			Ok(response) => assert!(response.account.account_name == "new-account-name"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

}
