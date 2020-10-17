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

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SamlFieldMapping {
	pub email: String,
	pub first_name: String,
	pub identity: String,
	pub last_name: String,
	pub real_name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AuthenticationMethod {
	pub account_id: String,
	pub authentication_method_enabled: bool,
	pub authentication_method_id: String,
	pub authentication_method_name: String,
	pub authentication_method_type: String,
	pub auto_join_team_id: String,
	pub create_time: String,
	pub delete_time: String,
	pub saml_acs_url: String,
	pub saml_certificate: String,
	pub saml_certificate_issuer: String,
	pub saml_certificate_not_valid_after: String,
	pub saml_certificate_not_valid_before: String,
	pub saml_certificate_subject: String,
	pub saml_digest_algorithm: String,
	pub saml_entity_id: String,
	pub saml_field_mapping: SamlFieldMapping,
	pub saml_idp_url: String,
	pub saml_metadata_url: String,
	pub saml_signature_algorithm: String,
	pub saml_variant: String,
	pub state: String,
	pub update_time: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AuthenticationMethodResponse {
	pub authentication_method: AuthenticationMethod,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AuthenticationMethodsResponse {
	pub authentication_methods: Vec<AuthenticationMethod>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AccountResponse {
	pub account: Account,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Account {
	pub account_id: String,
	pub account_name: String,
	pub account_owner_team_id: String,
	pub create_time: String,
	pub is_account_owner: bool,
	pub update_time: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Accounts {
	pub accounts: Vec<Account>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Event {
	pub account_id: String,
	pub action_description: String,
	pub action_type: String,
	pub actor: String,
	pub actor_user_id: String,
	pub create_time: String,
	pub log_entry_id: i64,
	pub team_id: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Events {
	pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Email {
	pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CardInfo {
	pub brand: String,
	pub card_id: String,
	pub country: String,
	pub country_code: String,
	pub exp_month: i32,
	pub exp_year: i64,
	pub last4: String,
	pub name: String,
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Project {
	pub account_id: String,
	pub account_name: String,
	pub address_lines: serde_json::Value,
	pub available_credits: String,
	pub billing_address: String,
	pub billing_currency: String,
	pub billing_emails: Vec<Email>,
	pub billing_extra_text: String,
	pub billing_group_id: String,
	pub billing_group_name: String,
	pub card_info: CardInfo,
	pub city: String,
	pub company: String,
	pub country: String,
	pub country_code: String,
	pub default_cloud: String,
	pub estimated_balance: String,
	pub features: serde_json::Value,
	pub payment_method: String,
	pub project_name: String,
	pub state: String,
	pub tech_emails: Vec<Email>,

	pub tenant_id: String,
	pub trial_expiration_time: String,
	pub vat_id: String,
	pub zip_code: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Projects {
	pub projects: Vec<Project>,
	pub total_project_count: i64,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Team {
	pub account_id: String,
	pub create_time: String,
	pub team_id: String,
	pub team_name: String,
	pub team_type: String,
	pub update_time: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TeamResponse {
	pub team: Team,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Teams {
	pub teams: Vec<Team>,
}
