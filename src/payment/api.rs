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
	payment::types,
};
use serde::Serialize;
pub struct PaymentApi {
	http_client: HTTPClient,
}

impl PaymentApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	#[allow(dead_code)]
	pub(crate) async fn get_stripe_key(&self) -> Result<String, AivenError> {
		let url = "config/stripe_key";
		let response: types::StripeKeyResponse = make_request!(self, reqwest::Method::GET, url)?
			.json()
			.await?;
		Ok(response.stripe_key)
	}

	/// Add credit card for user
	///
	/// https://api.aiven.io/doc/#operation/CreditCardAdd
	///
	/// # Arguments
	///
	/// * `stripe_token` - Credit card Stripe token
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// use std::collections::HashMap;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let mut json = HashMap::new();
	/// json.insert("stripe_token", "tok_17JIPf2eZvKY2CIkGwJXpO");
	/// let response = client
	///         .payment()
	///         .add_credit_card(&json)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn add_credit_card<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<types::ResCard, AivenError> {
		// TODO (ansrivas): Documentation of adding card
		let url = "card";
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Delete user's credit card
	///
	/// https://api.aiven.io/doc/#operation/CreditCardDelete
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .payment()
	///         .delete_credit_card("unique-card-id")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_credit_card(&self, card_id: &str) -> Result<(), AivenError> {
		let url = format!("card/{card_id}", card_id = encode_param(card_id));
		make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// List user's credit cards
	///
	/// https://api.aiven.io/doc/#operation/CreditCardsList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .payment()
	///         .list_credit_cards()
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_credit_cards(&self) -> Result<types::ResCards, AivenError> {
		let url = "card";
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Update user's credit card
	///
	/// https://api.aiven.io/doc/#operation/CreditCardUpdate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json = json!({
	/// "exp_month": 1,
	/// "exp_year": 2015,
	/// "name": "John Smith"
	/// });
	/// let response = client
	///         .payment()
	///         .update_credit_card("cardid",&json)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_credit_card<T: Serialize + ?Sized>(
		&self,
		card_id: &str,
		json_body: &T,
	) -> Result<types::ResCard, AivenError> {
		let url = format!("card/{card_id}", card_id = encode_param(card_id));
		let response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::{testutil, testutil::prepare_test_client};
	use serde_json::json;
	use std::collections::HashMap;

	#[tokio::test]
	async fn test_payment_list_credit_cards() {
		let client = testutil::prepare_test_client();
		let query_url = "/card";
		let test_data = testutil::get_test_data("tests/testdata/payment/list_cards.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.payment().list_credit_cards().await {
			Ok(response) => {
				let isok = response.cards.len() > 0;
				assert!(isok, format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[should_panic]
	#[tokio::test]
	async fn test_payment_get_stripe_key() {
		let client = prepare_test_client();

		match client.payment().get_stripe_key().await {
			Ok(response) => {
				assert!(response.len() > 0, format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_payment_add_credit_card() {
		let client = testutil::prepare_test_client();
		let query_url = "/card";
		let test_data = testutil::get_test_data("tests/testdata/payment/add_card.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let mut json_body = HashMap::new();
		json_body.insert("stripe_token", "some-token");
		match client.payment().add_credit_card(&json_body).await {
			Ok(response) => {
				assert!(response.card.last4 == "1234", format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_payment_update_credit_card() {
		let client = testutil::prepare_test_client();
		let query_url = "/card/cardid";
		let test_data = testutil::get_test_data("tests/testdata/payment/update_card.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let json = json!({
		"exp_month": 1,
		"exp_year": 2015,
		"name": "John Smith"
		});
		match client.payment().update_credit_card("cardid", &json).await {
			Ok(response) => {
				let isok = response.card.brand == "Visa";
				assert!(isok, format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_payment_delete_credit_card() {
		let client = testutil::prepare_test_client();
		let query_url = "/card/cardid";
		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client.payment().delete_credit_card("cardid").await {
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
