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

mod account;
mod cloud;
mod table;

use table::print_table;

use account::handle_accounts;
use account::Account;
use aiven_rs::errors;
use aiven_rs::{cloud::types::ResClouds, AivenClient};
use anyhow::{Error, Result};
use async_compat::Compat;
use cloud::{handle_cloud_list, Cloud};
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use smol;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
/// Top-level command.
pub struct CmdEntryPoint {
	#[structopt(subcommand)]
	pub commands: SubCommands,

	/// Server base url
	#[structopt(long, default_value = "https://api.aiven.io")]
	pub url: String,

	/// config file location, defaults to $HOME/.config/aiven/aiven-client.json
	#[structopt(long, parse(from_os_str))]
	pub config: Option<PathBuf>,

	/// CA certificate to use
	#[structopt(long, env = "AIVEN_CA_CERT")]
	pub auth_ca: Option<String>,

	/// Client auth token to use
	#[structopt(long, env = "AUTH_TOKEN")]
	pub auth_token: Option<String>,

	/// Show HTTP requests and responses
	#[structopt(long)]
	pub show_http: bool,
}

#[derive(StructOpt, Debug, PartialEq)]
#[structopt()]
pub enum SubCommands {
	#[structopt(name = "account")]
	One(Account),
	#[structopt(name = "cloud")]
	Two(Cloud),
}

fn main() {
	smol::block_on(Compat::new(async {
		let avn: CmdEntryPoint = CmdEntryPoint::from_args();

		let client = AivenClient::new(avn.url.as_ref(), "v1");

		let _ = match avn.commands {
			SubCommands::One(account) => handle_accounts(account),
			SubCommands::Two(cloud) => handle_cloud_list(&client, cloud).await,
			_ => Err(errors::AivenError::UnsupportedMethod),
		};
	}));
}
