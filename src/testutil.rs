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
use crate::client::AivenClient;
use mockito::{self, mock};
use once_cell::sync::OnceCell;
use std::{env, fs::File, io::prelude::*, path::PathBuf};

/// Internal test client which reads the Aiven token from an environment-var
#[allow(dead_code)]
static TEST_AIVEN_CLIENT: OnceCell<AivenClient> = OnceCell::new();

#[allow(dead_code)]
pub(crate) fn prepare_test_client() -> &'static AivenClient {
	TEST_AIVEN_CLIENT.get_or_init(|| {
		let url = &mockito::server_url();
		println!("Test client not found, recreating {:?}", &url);
		let token = env::var("AIVEN_TOKEN").unwrap_or_else(|_| "abc".to_string());
		AivenClient::from_token(url.as_ref(), "", &token)
		// let token = env::var("AIVEN_TOKEN").expect("Please set env variable
		// to read AIVEN_TOKEN"); AivenClient::from_token("https://api.aiven.io", "v1", &token)
	})
}

#[allow(dead_code)]
pub(crate) fn client() -> &'static AivenClient {
	TEST_AIVEN_CLIENT.get_or_init(|| {
		let url = &mockito::server_url();
		println!("Test client not found, recreating {:?}", &url);
		let token = env::var("AIVEN_TOKEN").unwrap_or_else(|_| "abc".to_string());
		AivenClient::from_token(url.as_ref(), "", &token)
		// let token = env::var("AIVEN_TOKEN").expect("Please set env variable
		// to read AIVEN_TOKEN"); AivenClient::from_token("https://api.aiven.io", "v1", &token)
	})
}
#[allow(dead_code)]
pub(crate) fn get_test_data(path: &str) -> String {
	let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	d.push(path);

	let err_msg = format!("Failed to open input file for testing: {:?}", &path);
	let mut file = File::open(d.as_path()).expect(&err_msg);
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Failed to read data from input test files");
	contents
}

#[allow(dead_code)]
pub(crate) fn create_mock_server(query_url: &str, test_data: &str, method: &str) -> mockito::Mock {
	// dbg!(query_url);
	// dbg!(method);
	// dbg!(test_data);

	let query_url = format!("/{}", query_url.trim_start_matches('/'));
	mock(method, query_url.as_ref())
		.match_header("authorization", "aivenv1 abc")
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(test_data)
		.create()
}

#[allow(dead_code)]
pub(crate) fn create_mock_server_404(
	query_url: &str,
	test_data: &str,
	method: &str,
) -> mockito::Mock {
	mock(method, query_url)
		.match_header("aiven1", "abc")
		.with_status(404)
		.with_header("content-type", "application/json")
		.with_body(test_data)
		.create()
}
