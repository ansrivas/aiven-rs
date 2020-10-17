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
use serde::Serialize;

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
	) -> Result<types::AccountResponse, AivenError> {
		let url = "account";
		let json_data = &[("account_name", account_name)];
		let response = make_json_request!(self, reqwest::Method::POST, url, json_data)?;
		Ok(response.json().await?)
	}

	/// Create a new account
	///
	/// https://api.aiven.io/doc/#operation/AccountList
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
	///         .list_accessible_accounts().await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_accessible_accounts(&self) -> Result<types::Accounts, AivenError> {
		let url = "account";
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Delete empty account
	///
	/// https://api.aiven.io/doc/#operation/AccountDelete
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
	///         .delete_account("my-account-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn delete_account(&self, account_id: &str) -> Result<(), AivenError> {
		let url = format!(
			"account/{account_id}",
			account_id = encode_param(account_id)
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Get account details
	///
	/// https://api.aiven.io/doc/#operation/AccountGet
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
	///         .get_details("account-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn get_details(
		&self,
		account_id: &str,
	) -> Result<types::AccountResponse, AivenError> {
		let url = format!(
			"account/{account_id}",
			account_id = encode_param(account_id)
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get account details
	///
	/// https://api.aiven.io/doc/#operation/AccountGet
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
	/// let json_body = json!({
	///    "account_name": "some-account-name"
	/// });
	/// let response = client
	///         .account()
	///         .update_account("my-account-id", &json_body).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn update_account<T: ?Sized + Serialize>(
		&self,
		account_id: &str,
		json_body: &T,
	) -> Result<types::AccountResponse, AivenError> {
		let url = format!(
			"account/{account_id}",
			account_id = encode_param(account_id)
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// List account events
	///
	/// https://api.aiven.io/doc/#operation/AccountEventList
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
	///         .list_events("my-account-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_events(&self, account_id: &str) -> Result<types::Events, AivenError> {
		let url = format!(
			"account/{account_id}/events",
			account_id = encode_param(account_id)
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List projects belonging to account
	///
	/// https://api.aiven.io/doc/#operation/AccountProjectsList
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
	///         .list_projects("my-account-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_projects(&self, account_id: &str) -> Result<types::Projects, AivenError> {
		let url = format!(
			"account/{account_id}/projects",
			account_id = encode_param(account_id)
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List account teams associated to a project
	///
	/// https://api.aiven.io/doc/#operation/AccountProjectsTeamsList
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
	///         .list_teams_for_project("my-account-id", "project-name").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_teams_for_project(
		&self,
		account_id: &str,
		project_name: &str,
	) -> Result<types::Teams, AivenError> {
		let url = format!(
			"account/{account_id}/project/{project_name}/teams",
			account_id = encode_param(account_id),
			project_name = encode_param(project_name)
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Create a new team
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamCreate
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
	/// let data = json!({"team_name": "my-new-team"});
	/// let response = client
	///         .account()
	///         .create_team("my-account-id", &data).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn create_team<T: ?Sized + Serialize>(
		&self,
		account_id: &str,
		json_data: &T,
	) -> Result<types::TeamResponse, AivenError> {
		let url = format!(
			"account/{account_id}/teams",
			account_id = encode_param(account_id)
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_data)?;
		Ok(response.json().await?)
	}

	/// List teams belonging to an account
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamList
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
	///         .list_teams("my-account-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_teams(&self, account_id: &str) -> Result<types::Teams, AivenError> {
		let url = format!(
			"account/{account_id}/teams",
			account_id = encode_param(account_id)
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Delete a team
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamDelete
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
	///         .delete_team("my-account-id", "my-team-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn delete_team(&self, account_id: &str, team_id: &str) -> Result<(), AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Get details for a single team
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamGet
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
	///         .get_team_details("my-account-id", "my-team-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn get_team_details(
		&self,
		account_id: &str,
		team_id: &str,
	) -> Result<types::TeamResponse, AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Update team details
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamUpdate
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
	/// let payload = json!({"team_name": "some-team"});
	/// let response = client
	///         .account()
	///         .update_team_details("my-account-id", "my-team-id", &payload).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn update_team_details<T: ?Sized + Serialize>(
		&self,
		account_id: &str,
		team_id: &str,
		json_body: &T,
	) -> Result<types::TeamResponse, AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// List pending invites
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamInvitesList
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
	///         .list_pending_invites("my-account-id", "my-team-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_pending_invites(
		&self,
		account_id: &str,
		team_id: &str,
	) -> Result<types::Invites, AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/invites",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Cancel pending user invite
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamMemberCancelInvite
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
	///         .cancel_pending_invite("my-account-id", "my-team-id", "email-to-cancel").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn cancel_pending_invite(
		&self,
		account_id: &str,
		team_id: &str,
		user_email: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/invites/{email}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
			email = encode_param(user_email),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Confirm account team invite
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamMemberVerifyInvite
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
	///         .confirm_team_invite("my-account-id", "verification-code").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn confirm_team_invite(
		&self,
		account_id: &str,
		invite_verification_code: &str,
	) -> Result<types::InviteDetails, AivenError> {
		let url = format!(
			"/account/{account_id}/invite/{invite_verification_code}",
			account_id = encode_param(account_id),
			invite_verification_code = encode_param(invite_verification_code),
		);
		let response = make_request!(self, reqwest::Method::POST, &url)?;
		Ok(response.json().await?)
	}

	/// Remove a member from the team
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamMemberVerifyInvite
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
	///         .remove_member("my-account-id", "team_id", "user-id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn remove_member(
		&self,
		account_id: &str,
		team_id: &str,
		user_id: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/member/{user_id}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
			user_id = encode_param(user_id),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Invite a new member to join the team
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamMembersInvite
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
	/// let json_payload = json!({"email": "my-email-address"});
	/// let response = client
	///         .account()
	///         .invite_member("my-account-id", "team_id", &json_payload).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn invite_member<T: ?Sized + Serialize>(
		&self,
		account_id: &str,
		team_id: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/members",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
		);
		let _response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(())
	}

	/// List members of a single team
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamMembersList
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
	///         .list_team_members("my-account-id", "team_id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_team_members(
		&self,
		account_id: &str,
		team_id: &str,
	) -> Result<types::Members, AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/members",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Associate team to a project
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamProjectAssociate
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
	/// let payload = json!({"team_type": "my-team-type"});
	/// let response = client
	///         .account()
	///         .associate_team_to_project("my-account-id", "team_id", "project", &payload).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn associate_team_to_project<T: ?Sized + Serialize>(
		&self,
		account_id: &str,
		team_id: &str,
		project: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/project/{project}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
			project = encode_param(project),
		);
		let _response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(())
	}

	/// Update team-project association
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamProjectAssociationUpdate
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
	/// let payload = json!({"team_type": "my-team-type"});
	/// let response = client
	///         .account()
	///         .update_team_project_association("my-account-id",
	/// 				"team_id",
	/// 				"project",
	/// 				 &payload).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn update_team_project_association<T: ?Sized + Serialize>(
		&self,
		account_id: &str,
		team_id: &str,
		project: &str,
		json_body: &T,
	) -> Result<(), AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/project/{project}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
			project = encode_param(project),
		);
		let _response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(())
	}

	/// Disassociate team from a project
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamProjectDisassociate
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
	///         .dissociate_team_from_project("my-account-id", "team_id", "project").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn dissociate_team_from_project(
		&self,
		account_id: &str,
		team_id: &str,
		project: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/project/{project}",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
			project = encode_param(project),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// List projects associated to a team
	///
	/// https://api.aiven.io/doc/#operation/AccountTeamProjectList
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
	///         .list_projects_by_team("my-account-id", "team_id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_projects_by_team(
		&self,
		account_id: &str,
		team_id: &str,
	) -> Result<types::TeamProjects, AivenError> {
		let url = format!(
			"/account/{account_id}/team/{team_id}/projects",
			account_id = encode_param(account_id),
			team_id = encode_param(team_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List projects associated with this account that user has access to
	///
	/// https://api.aiven.io/doc/#operation/AccountUserProjectsList
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
	///         .list_projects_by_user("my-account-id", "user_id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_projects_by_user(
		&self,
		account_id: &str,
		user_id: &str,
	) -> Result<types::UserProjects, AivenError> {
		let url = format!(
			"/account/{account_id}/team/{user_id}/projects",
			account_id = encode_param(account_id),
			user_id = encode_param(user_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List all teams for user
	///
	/// https://api.aiven.io/doc/#operation/AccountUserTeamsList
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
	///         .list_teams_for_user("my-account-id", "user_id").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_teams_for_user(
		&self,
		account_id: &str,
		user_id: &str,
	) -> Result<types::Teams, AivenError> {
		let url = format!(
			"/account/{account_id}/user/{user_id}/teams",
			account_id = encode_param(account_id),
			user_id = encode_param(user_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List/search users who are members of any team on this account
	///
	/// https://api.aiven.io/doc/#operation/AccountUsersSearch
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
	///
	/// let payload = json!({"limit": 1, "order_by": [{}], "query": "some-query"});
	/// let response = client
	///         .account()
	///         .search_users("my-account-id", &payload).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn search_users<T: ?Sized + Serialize>(
		&self,
		account_id: &str,
		json_body: &T,
	) -> Result<types::Users, AivenError> {
		let url = format!(
			"/account/{account_id}/users/search",
			account_id = encode_param(account_id),
		);
		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use crate::{client::encode_param, testutil};
	use serde_json::json;

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

		match client
			.account()
			.delete_auth_method("account_id", "someid")
			.await
		{
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

		match client
			.account()
			.get_auth_method_details("account_id", "someid")
			.await
		{
			Ok(response) => {
				assert!(response.authentication_method.account_id == "unique-account-id")
			}
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

		match client
			.account()
			.update_auth_method("account_id", "someid")
			.await
		{
			Ok(response) => {
				assert!(response.authentication_method.account_id == "updated-account-id")
			}
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

	#[tokio::test]
	async fn test_account_list_accessible_accounts() {
		let client = testutil::prepare_test_client();
		let query_url = "/account";

		let test_data =
			testutil::get_test_data("tests/testdata/account/list_accessible_accounts.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client.account().list_accessible_accounts().await {
			Ok(response) => assert!(response.accounts.len() > 0),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_delete_account() {
		let client = testutil::prepare_test_client();
		let query_url = format!("/account/{}", encode_param("some-account-to-delete"));

		let _m = testutil::create_mock_server(&query_url, "", "DELETE");

		match client
			.account()
			.delete_account("some-account-to-delete")
			.await
		{
			Ok(_response) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_get_details() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}",
			account_id = encode_param("some-account-details")
		);

		let test_data = testutil::get_test_data("tests/testdata/account/get_details.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client.account().get_details("some-account-details").await {
			Ok(response) => assert!(response.account.account_name == "some-account-name"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_update_account() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}",
			account_id = encode_param("some-account-id")
		);

		let test_data = testutil::get_test_data("tests/testdata/account/update_account.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "PUT");

		let body = json!({"account_name": "updated_name"});
		match client
			.account()
			.update_account("some-account-id", &body)
			.await
		{
			Ok(response) => assert!(response.account.account_name == "updated_name"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_events() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/events",
			account_id = encode_param("some-account-id")
		);

		let test_data = testutil::get_test_data("tests/testdata/account/list_events.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client.account().list_events("some-account-id").await {
			Ok(response) => assert!(response.events[0].account_id == "unique-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_projects() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/projects",
			account_id = encode_param("some-account-id")
		);

		let test_data = testutil::get_test_data("tests/testdata/account/list_projects.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client.account().list_projects("some-account-id").await {
			Ok(response) => assert!(response.projects[0].account_id == "unique-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_teams_for_project() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/project/{project_name}/teams",
			account_id = encode_param("unique-account-id"),
			project_name = encode_param("some-project-name")
		);

		let test_data =
			testutil::get_test_data("tests/testdata/account/list_teams_for_project.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client
			.account()
			.list_teams_for_project("unique-account-id", "some-project-name")
			.await
		{
			Ok(response) => assert!(response.teams[0].account_id == "unique-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_create_team() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/teams",
			account_id = encode_param("unique-account-id"),
		);

		let test_data = testutil::get_test_data("tests/testdata/account/create_team.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "POST");

		let json_body = json!({"team_name": "new-team"});
		match client
			.account()
			.create_team("unique-account-id", &json_body)
			.await
		{
			Ok(response) => assert!(response.team.account_id == "unique-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_teams() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/teams",
			account_id = encode_param("unique-account-id"),
		);

		let test_data = testutil::get_test_data("tests/testdata/account/list_teams.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client.account().list_teams("unique-account-id").await {
			Ok(response) => assert!(response.teams[0].account_id == "unique-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_delete_team() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team-id"),
		);

		let _m = testutil::create_mock_server(&query_url, "", "DELETE");

		match client
			.account()
			.delete_team("unique-account-id", "team-id")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_get_team_details() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team-id"),
		);

		let test_data = testutil::get_test_data("tests/testdata/account/get_team_details.json");
		let _m = testutil::create_mock_server(&query_url, &test_data, "GET");

		match client
			.account()
			.get_team_details("unique-account-id", "team-id")
			.await
		{
			Ok(response) => assert!(response.team.account_id == "unique-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_update_team_details() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team-id"),
		);

		let test_response =
			testutil::get_test_data("tests/testdata/account/update_team_details.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "PUT");

		let payload = json!({"team_name": "some-team-name"});

		match client
			.account()
			.update_team_details("unique-account-id", "team-id", &payload)
			.await
		{
			Ok(response) => assert!(response.team.account_id == "updated-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_pending_invites() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/invites",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team-id"),
		);

		let test_response =
			testutil::get_test_data("tests/testdata/account/list_pending_invites.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "GET");

		match client
			.account()
			.list_pending_invites("unique-account-id", "team-id")
			.await
		{
			Ok(response) => assert!(response.account_invites[0].account_id == "invited-account-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_cancel_pending_invite() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/invites/{email}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team-id"),
			email = encode_param("some-email-address"),
		);

		let _m = testutil::create_mock_server(&query_url, "", "DELETE");

		match client
			.account()
			.cancel_pending_invite("unique-account-id", "team-id", "some-email-address")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_confirm_team_invite() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/invite/{invite_verification_code}",
			account_id = encode_param("unique-account-id"),
			invite_verification_code = encode_param("unique-verification-code"),
		);

		let test_response =
			testutil::get_test_data("tests/testdata/account/confirm_account_team_invite.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "POST");

		match client
			.account()
			.confirm_team_invite("unique-account-id", "unique-verification-code")
			.await
		{
			Ok(response) => assert!(response.invite_details.user_email == "approved-email"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_remove_member() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/member/{user_id}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team_id"),
			user_id = encode_param("some-user-id"),
		);

		let _m = testutil::create_mock_server(&query_url, "", "DELETE");

		match client
			.account()
			.remove_member("unique-account-id", "team_id", "some-user-id")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_invite_member() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/members",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team_id"),
		);

		let _m = testutil::create_mock_server(&query_url, "", "POST");

		let payload = json!({"email": "email-to-invite"});
		match client
			.account()
			.invite_member("unique-account-id", "team_id", &payload)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_team_members() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/members",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team_id"),
		);

		let test_response =
			testutil::get_test_data("tests/testdata/account/list_team_members.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "GET");

		match client
			.account()
			.list_team_members("unique-account-id", "team_id")
			.await
		{
			Ok(response) => assert!(response.members[0].create_time == "timestamp"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_associate_team_to_project() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/project/{project}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team_id"),
			project = encode_param("project"),
		);

		// let test_response =
		// 	testutil::get_test_data("tests/testdata/account/associate_team_to_project.
		// json");
		let _m = testutil::create_mock_server(&query_url, "", "POST");

		let payload = json!({"team_type": "my-team-type"});
		match client
			.account()
			.associate_team_to_project("unique-account-id", "team_id", "project", &payload)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_update_team_project_association() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/project/{project}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team_id"),
			project = encode_param("project"),
		);

		// let test_response =
		// 	testutil::get_test_data("tests/testdata/account/
		// update_team_project_association.json");
		let _m = testutil::create_mock_server(&query_url, "", "PUT");

		let payload = json!({"team_type": "my-team-type"});
		match client
			.account()
			.update_team_project_association("unique-account-id", "team_id", "project", &payload)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_dissociate_team_from_project() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/project/{project}",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team_id"),
			project = encode_param("project"),
		);

		let _m = testutil::create_mock_server(&query_url, "", "DELETE");

		match client
			.account()
			.dissociate_team_from_project("unique-account-id", "team_id", "project")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_projects_by_team() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{team_id}/projects",
			account_id = encode_param("unique-account-id"),
			team_id = encode_param("team_id"),
		);

		let test_response =
			testutil::get_test_data("tests/testdata/account/list_projects_by_team.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "GET");

		match client
			.account()
			.list_projects_by_team("unique-account-id", "team_id")
			.await
		{
			Ok(response) => assert!(response.projects[0].project_name == "my-team-project-one"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_projects_by_user() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/team/{user_id}/projects",
			account_id = encode_param("unique-account-id"),
			user_id = encode_param("user_id"),
		);

		let test_response =
			testutil::get_test_data("tests/testdata/account/list_projects_by_user.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "GET");

		match client
			.account()
			.list_projects_by_user("unique-account-id", "user_id")
			.await
		{
			Ok(response) => {
				assert!(response.user_projects[0].project_name == "my-user-project-one")
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_list_teams_for_user() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/user/{user_id}/teams",
			account_id = encode_param("unique-account-id"),
			user_id = encode_param("user_id"),
		);

		let test_response =
			testutil::get_test_data("tests/testdata/account/list_teams_for_user.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "GET");

		match client
			.account()
			.list_teams_for_user("unique-account-id", "user_id")
			.await
		{
			Ok(response) => assert!(response.teams[0].team_id == "unique-team-id"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_account_search_users() {
		let client = testutil::prepare_test_client();
		let query_url = format!(
			"/account/{account_id}/users/search",
			account_id = encode_param("unique-account-id"),
		);

		let test_response = testutil::get_test_data("tests/testdata/account/search_users.json");
		let _m = testutil::create_mock_server(&query_url, &test_response, "POST");

		let payload = json!({"limit": 1, "order_by": [{}], "query": "some-query"});
		match client
			.account()
			.search_users("unique-account-id", &payload)
			.await
		{
			Ok(response) => assert!(response.users[0].real_name == "real_user"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
