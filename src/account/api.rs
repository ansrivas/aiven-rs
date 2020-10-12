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
	response::APIResponse,
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
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let response = client
	///         .account()
	///         .create_new_auth_method("my-project").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn create_new_auth_method<T: Serialize + ?Sized>(
		&self,
		account_id: &str,
		json_body: &T,
	) -> Result<serde_json::Value, AivenError> {
		let url = &format!(
			"account/{account_id}/authentication",
			account_id = account_id
		);
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}
}
