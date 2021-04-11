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

use crate::user::types::*;
use crate::{
	client::{encode_param, HTTPClient},
	errors::AivenError,
	make_json_request, make_request,
};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
pub struct UserApi {
	http_client: HTTPClient,
}

impl UserApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}

	/// Authenticate user and return token for following authorizations
	///
	/// https://api.aiven.io/doc/#operation/UserAuth
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let mut json_body: HashMap<&str, String> = HashMap::new();
	/// json_body.insert("email", "some-user-email".to_owned());
	/// json_body.insert("password", "my_pass".to_owned());
	/// // Optionally if 2FA is enabled
	/// // json_body.insert("otp", o.into());
	/// let output = client.user().authenticate(&json_body).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn authenticate<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<UserAuth, AivenError> {
		let url: &str = "userauth";
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Sets a new password for the user. Immediately expires all existing
	/// authentication tokens.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let mut json_body: HashMap<&str, String> = HashMap::new();
	/// json_body.insert("new_password", "newpassword".to_owned());
	/// json_body.insert("password", "current_password".to_owned());
	/// let output = client.user().password_change(&json_body).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn password_change<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<ResUserPasswordChange, AivenError> {
		let url = "me/password";
		let response = make_json_request!(self, reqwest::Method::PUT, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Complete one-time password configuration.
	///
	/// https://api.aiven.io/doc/#operation/TwoFactorAuthConfigureOTP
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let mut json_body: HashMap<&str, String> = HashMap::new();
	/// json_body.insert("otp", "987654".into());
	/// json_body.insert("password", "abc123".into());
	/// json_body.insert("uri", "otpauth://.....".into());
	/// let output = client.user().complete_otp_config(&json_body).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn complete_otp_config<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<ResCompleteOTPConfig, AivenError> {
		let url = "me/2fa/otp";
		let response = make_json_request!(self, reqwest::Method::PUT, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Configure two-factor authentication.
	///
	/// https://api.aiven.io/doc/#operation/TwoFactorAuthConfigure.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let mut json_body: HashMap<&str, String> = HashMap::new();
	/// json_body.insert("method", "otp".into());
	/// json_body.insert("otp", "987654".into());
	/// let output = client.user().configure_2fa(&json_body).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn configure_2fa<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<ResConfigure2fa, AivenError> {
		let url = "me/2fa";
		let response = make_json_request!(self, reqwest::Method::PUT, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Confirm user email address.
	///
	/// https://api.aiven.io/doc/#operation/UserVerifyEmail.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.user().confirm_email_address("some-verification-code").await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn confirm_email_address(
		&self,
		verification_code: &str,
	) -> Result<ResConfirmUseremailAddress, AivenError> {
		let url = &format!("user/verify_email/{}", encode_param(verification_code));
		let response = make_request!(self, reqwest::Method::POST, url)?;
		Ok(response.json().await?)
	}

	/// Confirm user password reset.
	///
	/// https://api.aiven.io/doc/#operation/UserPasswordReset
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.user()
	/// .confirm_password_reset("newpass","verificationcode")
	/// .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn confirm_password_reset(
		&self,
		new_password: &str,
		verification_code: &str,
	) -> Result<(), AivenError> {
		let url = &format!("user/password_reset/{}", encode_param(verification_code));

		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("new_password", new_password.into());
		let body = &json_body;
		let _response = make_json_request!(self, reqwest::Method::POST, url, body)?;
		Ok(())
	}

	/// Create a user
	///
	/// https://api.aiven.io/doc/#operation/UserCreate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// use aiven_rs::user::types::UserCreateConfig;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let user_config = UserCreateConfig::default();
	/// let output = client.user().create(&user_config).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create(
		&self,
		user_config: &UserCreateConfig,
	) -> Result<ResUserCreate, AivenError> {
		let url = "user";
		let response = make_json_request!(self, reqwest::Method::POST, url, user_config)?;
		Ok(response.json().await?)
	}

	/// Create new access token
	///
	/// https://api.aiven.io/doc/#operation/AccessTokenCreate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let mut json_body: HashMap<&str, String> = HashMap::new();
	/// json_body.insert("description", "Integration client Alpha".into());
	/// json_body.insert("extend_when_used", "true".into());
	/// json_body.insert("max_age_seconds", "86400".into());
	/// let output = client.user().create_access_token(&json_body).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create_access_token<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<AccessToken, AivenError> {
		let url = "access_token";
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Delete linked authentication method, and revoke all associated access
	/// tokens
	///
	/// https://api.aiven.io/doc/#operation/UserAuthenticationMethodDelete
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let user_authentication_method_id = "somemethod-id";
	/// let output = client.user().delete_auth_method(user_authentication_method_id).await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_auth_method(&self, auth_method: &str) -> Result<(), AivenError> {
		let url = &format!("me/authentication_methods/{}", encode_param(auth_method));
		let _response = make_request!(self, reqwest::Method::DELETE, url)?;
		Ok(())
	}

	/// Expire all authorization tokens.
	///
	/// https://api.aiven.io/doc/#operation/UserExpireTokens
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.user().expire_auth_tokens().await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn expire_auth_tokens(&self) -> Result<(), AivenError> {
		let url = "me/expire_tokens";
		let _response = make_request!(self, reqwest::Method::POST, url)?;
		Ok(())
	}

	/// Get information for the current session's user
	///
	/// https://api.aiven.io/doc/#operation/UserInfo
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.user().info().await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn info(&self) -> Result<UserInfo, AivenError> {
		let url = "me";
		let json_body: &HashMap<&str, String> = &HashMap::new();
		let response = make_json_request!(self, reqwest::Method::GET, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Get available login options
	///
	/// https://api.aiven.io/doc/#operation/UserAuthLoginOptions
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.user().expire_auth_tokens().await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn auth_login_options(
		&self,
		json_body: &HashMap<&str, String>,
	) -> Result<Vec<UserAuthLoginOptions>, AivenError> {
		let url = "userauth/login_options";
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// List all valid access tokens
	///
	/// https://api.aiven.io/doc/#operation/AccessTokenList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let output = client.user().list_access_tokens()
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn list_access_tokens(&self) -> Result<AccessTokens, AivenError> {
		let url = "access_token";
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// List linked authentication methods
	///
	/// https://api.aiven.io/doc/#operation/UserAuthenticationMethodsList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .list_linked_auth_methods()
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn list_linked_auth_methods(&self) -> Result<AuthenticationMethods, AivenError> {
		let url = "me/authentication_methods";
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Logout user, removing current authentication token.
	///
	/// https://api.aiven.io/doc/#operation/UserLogout
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .logout()
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn logout(&self) -> Result<(), AivenError> {
		let url = "me/logout";
		let _response = make_request!(self, reqwest::Method::POST, url)?;
		Ok(())
	}

	/// Request user password reset.
	///
	/// https://api.aiven.io/doc/#operation/UserPasswordResetRequest
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .password_reset("myemail.com")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn password_reset(&self, email: &str) -> Result<(), AivenError> {
		let url = "user/password_reset_request";
		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("email", email.into());
		let body = &json_body;
		let _response = make_json_request!(self, reqwest::Method::POST, url, body)?;
		Ok(())
	}

	/// Revoke an access token
	///
	/// https://api.aiven.io/doc/#operation/AccessTokenRevoke
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .revoke_access_token("my-token-prefix")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn revoke_access_token(&self, token_prefix: &str) -> Result<(), AivenError> {
		let url = format!(
			"access_token/{token_prefix}",
			token_prefix = encode_param(token_prefix)
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Update an existing access token.
	///
	/// https://api.aiven.io/doc/#operation/AccessTokenUpdate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .update_access_token("token-prefix", "some-description")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn update_access_token(
		&self,
		token_prefix: &str,
		description: &str,
	) -> Result<AccessToken, AivenError> {
		let url = &format!(
			"access_token/{token_prefix}",
			token_prefix = encode_param(token_prefix)
		);
		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("description", description.into());
		let body = &json_body;

		let response = make_json_request!(self, reqwest::Method::PUT, url, body)?;
		Ok(response.json().await?)
	}

	/// Accept all invites for a single account.
	///
	/// https://api.aiven.io/doc/#operation/UserAccountInvitesAccept
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .accept_all_invites_for_account("account_id", "team_id")
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn accept_all_invites_for_account(
		&self,
		account_id: &str,
		team_id: &str,
	) -> Result<ResAccountInvites, AivenError> {
		let url = "/me/account/invites/accept";
		let body = &json!({
			"account_id": account_id,
			"team_id": team_id,
		});

		let response = make_json_request!(self, reqwest::Method::POST, url, body)?;
		Ok(response.json().await?)
	}

	/// List pending account invites.
	///
	/// https://api.aiven.io/doc/#operation/UserAccountInvitesList
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .list_pending_account_invites()
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn list_pending_account_invites(&self) -> Result<ResAccountInvites, AivenError> {
		let url = "/me/account/invites";
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// Reject invite to a team.
	///
	/// https://api.aiven.io/doc/#operation/UserAccountInvitesReject
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::new("https://api.aiven.io", "v1");
	/// let response = client
	///         .user()
	///         .reject_invite_to_team()
	///         .await
	///         .unwrap();
	/// Ok(())
	/// }
	/// ```
	pub async fn reject_invite_to_team(&self) -> Result<ResAccountInvites, AivenError> {
		let url = "/me/account/invites/reject";
		let response = make_request!(self, reqwest::Method::POST, url)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testutil;

	#[tokio::test]
	async fn test_user_authenticate() {
		let client = testutil::prepare_test_client();
		let query_url = "/userauth";
		let test_data = testutil::get_test_data("tests/testdata/user/authenticate.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let mut json_body = HashMap::new();
		json_body.insert("email", "jane@example.com".to_owned());
		json_body.insert("password", "my_pass".to_owned());
		match client.user().authenticate(&json_body).await {
			Ok(response) => {
				assert!(
					response.user_email == "jane@example.com",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_user_password_change() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/password";
		let test_data = testutil::get_test_data("tests/testdata/user/password_change.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let mut json_body = HashMap::new();
		json_body.insert("new_password", "xyz567".to_owned());
		json_body.insert("password", "abc123".to_owned());
		match client.user().password_change(&json_body).await {
			Ok(response) => {
				assert!(response.token == "some-token", format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_user_complete_otp_config() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/2fa/otp";
		let test_data = testutil::get_test_data("tests/testdata/user/complete_otp_config.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("otp", "987654".into());
		json_body.insert("password", "abc123".into());
		json_body.insert(
			"uri",
			"otpauth://totp/Aiven:foo%40example.com?secret=NF4E6L2JPISKV3AI&issuer=Aiven".into(),
		);
		match client.user().complete_otp_config(&json_body).await {
			Ok(response) => {
				assert!(response.token == "some-token", format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_user_configure_2fa() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/2fa";
		let test_data = testutil::get_test_data("tests/testdata/user/configure_2fa.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("method", "otp".into());
		json_body.insert("password", "abc123".into());

		match client.user().configure_2fa(&json_body).await {
			Ok(response) => {
				assert!(response.method == "otp", format!("{:?}", response));
				assert!(response.uri == "some-url", format!("{:?}", response));
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_user_confirm_email_address() {
		let client = testutil::prepare_test_client();
		let query_url = "/user/verify_email/verificationcode";
		let test_data = testutil::get_test_data("tests/testdata/user/confirm_user_email.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.user()
			.confirm_email_address("verificationcode")
			.await
		{
			Ok(response) => {
				assert!(
					response.invite_details.user_email == "jane@example.com",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_user_confirm_password_reset() {
		let client = testutil::prepare_test_client();
		let query_url = "/user/password_reset/verificationcode";
		let _m = testutil::create_mock_server(query_url, "", "POST");

		match client
			.user()
			.confirm_password_reset("newpassword", "verificationcode")
			.await
		{
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_user_create() {
		let client = testutil::prepare_test_client();
		let query_url = "/user";
		let test_data = testutil::get_test_data("tests/testdata/user/create_user.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let conf = UserCreateConfig {
			credit_code: "AVN2015".into(),
			email: "jane@example.com".into(),
			email_communication_categories: vec!["aiven_newsletter".into()],
			origin: "google search".into(),
			password: "abc123".into(),
			real_name: "Jane Smith".into(),
			token: "ZWJyOGZmcm1icDczYW83cQ==".into(),
		};
		match client.user().create(&conf).await {
			Ok(response) => {
				assert!(response.user_email == "jane@example.com");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_user_create_access_token() {
		let client = testutil::prepare_test_client();
		let query_url = "/access_token";
		let test_data = testutil::get_test_data("tests/testdata/user/create_access_token.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("description", "test-token-creation".to_owned());
		json_body.insert("extend_when_used ", "true".to_owned());
		json_body.insert("max_age_seconds ", "3600".to_owned());
		match client.user().create_access_token(&json_body).await {
			Ok(o) => assert!(true, "Output was {:?}", o),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_delete_auth_method() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/authentication_methods/someuniqueauthmethod";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		let user_authentication_method_id = "someuniqueauthmethod";
		match client
			.user()
			.delete_auth_method(user_authentication_method_id)
			.await
		{
			Ok(o) => assert!(true, "Output was {:?}", o),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_expire_auth_tokens() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/expire_tokens";
		let _m = testutil::create_mock_server(query_url, "", "POST");

		match client.user().expire_auth_tokens().await {
			Ok(o) => assert!(true, "Output was {:?}", o),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_auth_login_options() {
		let client = testutil::prepare_test_client();
		let query_url = "/userauth/login_options";
		let test_data = testutil::get_test_data("tests/testdata/user/auth_login_options.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let mut json_body: HashMap<&str, String> = HashMap::new();
		json_body.insert("email", "jane@example.com".into());
		let output: Vec<UserAuthLoginOptions> = client
			.user()
			.auth_login_options(&json_body)
			.await
			.expect("Failed to authenticate");
		assert!(
			output.len() > 0,
			"Should have received atleast one user-login option"
		);
	}

	#[tokio::test]
	async fn test_user_info() {
		let client = testutil::prepare_test_client();
		let query_url = "/me";
		let test_data = testutil::get_test_data("tests/testdata/user/current_user_info.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let output = client.user().info().await.expect("Failed to authenticate");
		assert!(
			output.user.user == "jane@example.com",
			"Should successfully receive the user-information."
		);
	}

	#[tokio::test]
	async fn test_user_list_access_tokens() {
		let client = testutil::prepare_test_client();
		let query_url = "/access_token";
		let test_data = testutil::get_test_data("tests/testdata/user/list_all_access_tokens.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let user_client = client.user();
		match user_client.list_access_tokens().await {
			Ok(response) => {
				// for token in response.tokens.iter() {
				// 	if let Some(desc) = &token.description {
				// 		if desc.contains("test-token") {
				// 			let to_delete_token = token.token_prefix.clone().unwrap();
				// 			user_client
				// 				.revoke_access_token(&to_delete_token)
				// 				.await
				// 				.expect("Failed to remove the test-token");
				// 		}
				// 	}
				// }
				assert!(response.tokens.len() > 0, "Output was {:?}", response)
			}
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_list_linked_auth_methods() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/authentication_methods";
		let test_data =
			testutil::get_test_data("tests/testdata/user/list_linked_auth_methods.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let user_client = client.user();
		match user_client.list_linked_auth_methods().await {
			Ok(response) => assert!(
				response.authentication_methods.len() > 0,
				"Output was {:?}",
				response
			),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_logout() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/logout";
		let _m = testutil::create_mock_server(query_url, "", "POST");

		let user_client = client.user();
		match user_client.logout().await {
			Ok(_) => assert!(true),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_password_reset() {
		let client = testutil::prepare_test_client();
		let query_url = "/user/password_reset_request";
		let _m = testutil::create_mock_server(query_url, "", "POST");

		let user_client = client.user();
		match user_client.password_reset("myemail@de").await {
			Ok(_) => assert!(true),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_revoke_access_token() {
		let client = testutil::prepare_test_client();
		let query_url = "/access_token/mytokenprefix";
		let _m = testutil::create_mock_server(query_url, "", "DELETE");

		let user_client = client.user();
		match user_client.revoke_access_token("mytokenprefix").await {
			Ok(_) => assert!(true),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_update_access_token() {
		let client = testutil::prepare_test_client();
		let query_url = "/access_token/mytokenprefix";
		let test_data = testutil::get_test_data("tests/testdata/user/update_token.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let user_client = client.user();
		match user_client
			.update_access_token("mytokenprefix", "some-description")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_accept_all_invites_for_account() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/account/invites/accept";
		let test_data =
			testutil::get_test_data("tests/testdata/user/accept_all_invites_for_account.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let user_client = client.user();
		match user_client
			.accept_all_invites_for_account("account_id", "team_id")
			.await
		{
			Ok(rep) => {
				assert!(rep.account_invites.len() > 0);
				assert!(true)
			}
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_list_pending_account_invites() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/account/invites";
		let test_data =
			testutil::get_test_data("tests/testdata/user/list_pending_account_invites.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		let user_client = client.user();
		match user_client.list_pending_account_invites().await {
			Ok(rep) => {
				assert!(rep.account_invites.len() > 0);
				assert!(true)
			}
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_user_reject_invite_to_team() {
		let client = testutil::prepare_test_client();
		let query_url = "/me/account/invites/reject";
		let test_data = testutil::get_test_data("tests/testdata/user/reject_invite_to_team.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let user_client = client.user();
		match user_client.reject_invite_to_team().await {
			Ok(rep) => {
				assert!(rep.account_invites.len() > 0);
				assert!(true)
			}
			Err(e) => {
				assert!(false, "Error was {:?}", e);
			}
		}
	}
}
