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

//! # aiven-rs
//!
//! aiven-rs is a rust-sdk to interact with [aiven.io](https://aiven.io)
//!
//! # Getting started
//!
//! ```rust,no_run
//! // use aiven_rs;
//!
//! #[tokio::main]
//! async fn main()-> Result<(), Box<dyn std::error::Error>>{
//!
//!  let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
//!  // If creating an API to interact with resources which require authentication
//!  // let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
//!  let cloud_api = client.cloud();
//!  let resp = cloud_api.list_all().await?;
//!  for cloud in &resp.clouds{
//!   println!("{:?}", cloud.cloud_name);
//!  }
//!  Ok(())
//! }
//! ```
pub mod billing;
mod client;
mod customdeser;

pub mod account;
pub mod billing_group;
pub mod cloud;
pub mod key_mgmt;
pub mod payment;
pub mod project;
mod response;
pub mod service;
mod testutil;
pub mod ticket;
pub mod user;

pub mod errors;
pub use client::{APIError, AivenClient};

#[doc(hidden)]
pub use billing::ProjectBillingApi;
#[doc(hidden)]
pub use cloud::CloudApi;

#[doc(hidden)]
pub use key_mgmt::ProjectKeyManagementApi;

#[doc(hidden)]
pub use payment::PaymentApi;

#[doc(hidden)]
pub use project::ProjectApi;

#[doc(hidden)]
pub use service::{
	ServiceApi, ServiceElastiSearchApi, ServiceKafkaApi, ServiceMysqlApi, ServicePostgresApi,
};

#[doc(hidden)]
pub use user::UserApi;

#[doc(hidden)]
pub use ticket::TicketApi;

#[doc(hidden)]
pub use account::AccountApi;

#[doc(hidden)]
pub use billing_group::BillingGroupApi;
