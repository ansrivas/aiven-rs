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

use aiven_rs::{cloud::types::ResClouds, AivenClient};
use anyhow::{Error, Result};
use async_compat::Compat;
use smol;

fn main() -> Result<()> {
	smol::block_on(Compat::new(async {
		tracing_subscriber::fmt::init();

		let client = AivenClient::new("https://api.aiven.io", "v1");
		let cloud_api = client.cloud();
		let output: ResClouds = cloud_api.list_all().await.map_err(Error::msg)?;
		for cloud in &output.clouds {
			println!("{:?}", cloud.cloud_name);
		}

		// use std::env;
		// let token = env::var("AIVEN_TOKEN").expect("Please set env variable to read
		// AIVEN_TOKEN");

		// let client = AivenClient::from_token("https://api.aiven.io", "v1", &token);
		// let output = client.cloud()
		// 	.list_by_project("some-arbitrary-project")
		// 	.await
		// 	.map_err(Error::msg)?
		// 	for cloud in &output.clouds {
		// 	println!("{:?}", cloud.cloud_name);
		// }
		Ok(())
	}))
}
