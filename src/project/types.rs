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
use std::{
	collections::HashMap,
	fmt::{Display, Formatter},
};

pub enum MemberType {
	Admin,
	Developer,
	Operator,
	ReadOnly,
}
impl Display for MemberType {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			MemberType::Admin => write!(f, "admin"),
			MemberType::Developer => write!(f, "developer"),
			MemberType::Operator => write!(f, "operator"),
			MemberType::ReadOnly => write!(f, "read_only"),
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResProjectInvite {
	pub invite_details: InviteDetails,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InviteDetails {
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Email {
	pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CardInfo {
	pub brand: String,
	pub card_id: Option<String>,
	pub country: Option<String>,
	pub country_code: String,
	pub exp_month: Option<i32>,
	pub exp_year: Option<i32>,
	pub last4: Option<String>,
	pub name: Option<String>,
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResProject {
	pub project: Project,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Project {
	pub account_id: Option<String>,
	pub available_credits: String,
	pub billing_address: String,
	pub billing_currency: String,

	pub billing_emails: Vec<Email>,
	pub billing_extra_text: Option<String>,

	pub card_info: CardInfo,
	pub country: String,
	pub country_code: String,
	pub default_cloud: String,
	pub estimated_balance: String,
	pub features: Option<serde_json::Value>,
	pub payment_method: String,
	pub project_name: String,
	pub tech_emails: Vec<Email>,
	pub tenant_id: String,
	pub trial_expiration_time: Option<String>,
	pub vat_id: String,
}

/// Project membership and type of membership
// #[derive(Deserialize, Serialize, Debug, Default)]
// pub struct ProjectMembership {
// 	#[serde(rename = "ANY")]
// 	pub any: String,
// }

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectList {
	pub project_membership: HashMap<String, String>,
	pub projects: Vec<Project>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct StateInfo {
	pub message: String,
	#[serde(rename = "type")]
	pub state_type: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PeeringConnection {
	pub create_time: String,

	pub peer_azure_app_id: String,
	pub peer_azure_tenant_id: String,
	pub peer_cloud_account: String,

	pub peer_region: String,
	pub peer_resource_group: String,

	pub peer_vpc: String,
	pub state: String,

	pub state_info: StateInfo,
	pub update_time: String,
	pub user_peer_network_cidrs: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Alert {
	pub create_time: String,
	pub event: String,
	pub project_name: String,
	pub service_name: String,
	pub service_type: String,
	pub severity: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ActiveAlerts {
	pub alerts: Vec<Alert>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectVPC {
	pub cloud_name: String,
	pub create_time: String,
	pub network_cidr: String,

	pub peering_connections: Option<Vec<PeeringConnection>>,
	pub project_vpc_id: String,
	pub state: String,
	pub update_time: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectVPCs {
	pub vpcs: Vec<ProjectVPC>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Invitation {
	pub invite_time: String,
	pub invited_user_email: String,
	pub inviting_user_email: String,
	pub member_type: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Event {
	pub actor: String,
	pub event_desc: String,
	pub event_type: String,
	pub service_name: String,
	pub time: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResEvents {
	pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct User {
	pub auth: Vec<String>,
	pub billing_contact: bool,
	pub create_time: String,
	pub member_type: String,
	pub real_name: String,
	pub team_id: Option<String>,
	pub team_name: Option<String>,
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectUserList {
	pub users: Vec<User>,
	pub invitations: Vec<Invitation>,
}
