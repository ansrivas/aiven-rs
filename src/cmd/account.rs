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

use aiven_rs::errors;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
/// Account commands.
pub struct Account {
	#[structopt(subcommand)]
	pub commands: SubCommandsAccount,
}

#[derive(StructOpt, Debug, PartialEq)]
#[structopt()]
pub enum SubCommandsAccount {
	#[structopt(name = "authentication-method")]
	AuthMethod(CmdAccountAuthMethod),

	Create(CmdAccountCreate),
	Delete(CmdAccountCreate),
	List(CmdAccountCreate),
	Team(CmdAccountCreate),
	Update(CmdAccountCreate),
}

#[derive(StructOpt, Debug, PartialEq)]
/// Account Authentication-Method commands
// #[structopt(name = "authentication-method")]
pub struct CmdAccountAuthMethod {
	#[structopt(subcommand)]
	pub commands: SubCommandsAuthMethod,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum SubCommandsAuthMethod {
	/// List cloud types
	#[structopt(name = "list")]
	List {
		#[structopt(long)]
		project: Option<String>,

		/// raw json output
		#[structopt(long)]
		json: bool,
	},
}

#[derive(StructOpt, Debug, PartialEq)]
/// Create new account
#[structopt(name = "create")]
pub struct CmdAccountCreate {
	/// name of the account to create
	#[structopt(short, long)]
	name: String,

	/// raw json output
	#[structopt(long)]
	json: bool,
}

pub fn handle_accounts(account: Account) -> Result<(), errors::AivenError> {
	match account.commands {
		SubCommandsAccount::Create(cmd) => {
			println!("We will create a new project of name {}", cmd.name);
			if cmd.json {
				println!("We would also print json");
			}
		}
		_ => (),
	}
	Ok(())
}
