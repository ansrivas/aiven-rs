// MIT License
//
// Copyright (c) 2019 Ankur Srivastava
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

use crate::errors::AivenError;
use log::debug;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

#[derive(Debug, Clone)]
pub struct HTTPClient {
	client: reqwest::Client,
	base_url: reqwest::Url,
	version: String,
}

/// Percent encode an incoming parameter
pub(crate) fn encode_param(param: &str) -> String {
	percent_encode(param.as_bytes(), NON_ALPHANUMERIC).to_string()
}

/// Make a http request by providing a json-body
#[macro_export]
macro_rules! make_json_request {
	($sel:ident, $method:path, $url:expr, $body:ident) => {{
		use crate::errors::AivenError;
		use reqwest;
		use log::debug;

		let response: reqwest::Response = $sel
			.http_client
			.inner($method, $url)?
			.json($body)
			.send()
			.await?;
		let status_code = &response.status().as_u16();
		
		if !(*status_code >= 200 && *status_code <= 300) {
			debug!("status_code = {}",status_code);
			debug!("url queried = {}", $url);
			let api_response: APIResponse = response.json().await?;
			return Err(AivenError::APIResponseError {
				errors: api_response.errors.unwrap(),
				message: api_response.message.unwrap(),
				status: api_response.status,
				more_info: api_response.more_info,
			});
		}
		let ret: Result<reqwest::Response, AivenError> = Ok(response);
		ret
		}};
}

/// Make a http request without json body.
#[macro_export]
macro_rules! make_request {
	($sel:ident, $method:path, $url:expr) => {{
		use log::debug;
		use reqwest;
		let response: reqwest::Response = $sel.http_client.inner($method, $url)?.send().await?;

		let status_code = &response.status().as_u16();
		debug!("Received status code: {}", status_code);

		if !(*status_code >= 200 && *status_code <= 300) {
			let api_response: APIResponse = response.json().await?;
			return Err(AivenError::APIResponseError {
				errors: api_response.errors.unwrap(),
				message: api_response.message.unwrap(),
				status: api_response.status,
				more_info: api_response.more_info,
			});
			}
		let ret: Result<reqwest::Response, AivenError> = Ok(response);
		ret
		}};
}

impl HTTPClient {
	pub fn new<S>(base_url: S, client: reqwest::Client, version: String) -> HTTPClient
	where
		S: Into<String>,
	{
		let parsed_url =
			reqwest::Url::parse(&base_url.into()).expect("Failed to parse the base_url");

		let ver = format!("{}/", version.replace("/", ""));
		debug!("Version is {}", &ver);
		HTTPClient {
			base_url: parsed_url,
			client,
			version: ver,
		}
	}

	pub(crate) fn inner(
		&self,
		method: reqwest::Method,
		query_url: &str,
	) -> Result<reqwest::RequestBuilder, AivenError> {
		let qurl = query_url.trim_start_matches('/');
		let url = self.base_url.join(&self.version)?.join(qurl)?;
		debug!("URL is {:?}", &url);

		// dbg!(&url);
		let request_with_url_and_header: Result<reqwest::RequestBuilder, AivenError> = match method
		{
			reqwest::Method::GET => Ok(self.client.get(url)),
			reqwest::Method::PUT => Ok(self.client.put(url)),
			reqwest::Method::POST => Ok(self.client.post(url)),
			reqwest::Method::DELETE => Ok(self.client.delete(url)),
			_ => return Err(AivenError::UnsupportedMethod),
		};
		request_with_url_and_header
	}
}
