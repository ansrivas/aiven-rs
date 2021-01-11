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

use aiven_rs::AivenClient;
use std::{collections::HashMap, env};
use tracing::info;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	let token = env::var("AIVEN_TOKEN").expect("Please set env variable to read AIVEN_TOKEN");
	let client = AivenClient::from_token("https://api.aiven.io", "v1", &token);
	let cloud = client.project();
	let mut json_body = HashMap::new();
	json_body.insert("copy_from_project", "some-existing-project".to_owned());
	json_body.insert("project", "test".to_owned());

	// or using serde_json::json
	// use serde_json::json;
	// let json_body = json!({"copy_from_project": "some-existing-project",
	// "project": "new-name"});

	let output = cloud.create(&json_body).await.unwrap();
	info!("{:?}", output);
}
