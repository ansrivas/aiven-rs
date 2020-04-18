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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserAuth {
	pub state: String,
	pub token: String,
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResUserPasswordChange {
	// pub message: Option<String>,
	pub token: String,
	// pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResCompleteOTPConfig {
	// pub message: Option<String>,
	pub token: String,
	pub method: String,
	// pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResConfigure2fa {
	// pub message: Option<String>,
	pub qrcode: String,
	pub uri: String,
	pub method: String,
	// pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResConfirmUseremailAddress {
	// pub message: Option<String>,
	pub invite_details: UserEmail,
	// pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResConfirmPasswordReset {
	// pub message: Option<String>,
	pub invite_details: UserEmail,
	// pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserEmail {
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserAuthLoginOptions {
	pub action: String,
	pub method: Option<String>,
	pub name: Option<String>,
	pub redirect_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserInfo {
	pub user: User,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Invitation {
	pub invite_code: String,
	pub invite_time: String,
	pub inviting_user_email: String,
	pub project_name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct User {
	pub auth: Vec<String>,
	pub create_time: String,
	pub features: Option<HashMap<String, String>>,
	pub intercom: Option<HashMap<String, String>>,
	pub invitations: Vec<Invitation>,
	pub project_membership: HashMap<String, String>,
	pub projects: Vec<String>,
	pub real_name: String,
	pub state: String,
	pub token_validity_begin: String,
	pub user: String,
	pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AccessToken {
	pub create_time: String,
	pub created_manually: bool,
	pub currently_active: bool,
	pub description: Option<String>,
	pub expiry_time: Option<String>,
	pub extend_when_used: bool,
	pub full_token: Option<String>,
	pub last_ip: Option<String>,
	pub last_used_time: Option<String>,
	pub last_user_agent: Option<String>,
	pub last_user_agent_human_readable: Option<String>,
	pub max_age_seconds: Option<i64>,
	pub token_prefix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AccessTokens {
	pub tokens: Vec<AccessToken>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AuthenticationMethod {
	pub authentication_method_account_id: String,
	pub create_time: String,
	pub currently_active: bool,
	pub delete_time: String,
	pub last_used_time: String,
	pub method_id: String,
	pub name: String,
	pub public_remote_identity: String,
	pub remote_provider_id: String,
	pub state: String,
	pub update_time: String,
	pub user_email: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AuthenticationMethods {
	pub authentication_methods: Vec<AuthenticationMethod>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResUserCreate {
	pub state: String,
	pub token: String,
	pub user: User,
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserCreateConfig {
	pub credit_code: String,
	pub email: String,
	pub email_communication_categories: Vec<String>,
	pub origin: String,
	pub password: String,
	pub real_name: String,
	pub token: String,
}
