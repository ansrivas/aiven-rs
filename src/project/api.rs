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
	client::{encode_param, HTTPClient},
	errors::AivenError,
	make_json_request, make_request,
	project::types,
	response::APIResponse,
};
use serde::Serialize;
use std::collections::HashMap;

pub struct ProjectApi {
	http_client: HTTPClient,
}

impl ProjectApi {
	pub(crate) fn new(client: HTTPClient) -> Self {
		Self {
			http_client: client,
		}
	}
	///  Confirm project invite
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectInviteAccept
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .confirm_project_invite("myproject", "mycode")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn confirm_project_invite(
		&self,
		project: &str,
		verification_code: &str,
	) -> Result<types::ResProjectInvite, AivenError> {
		let url = &format!(
			"project/{project}/invite/{verification_code}",
			project = project,
			verification_code = verification_code
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Create a project
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectCreate
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	///
	/// let json = json!({
	/// "account_id": "a22ba494e096",
	/// "billing_address": "ACME Corporation, Main Street 1, Monument Valley, UT",
	/// "billing_currency": "USD",
	/// "billing_emails": [
	/// {
	///  "email": "jane@example.com"
	/// }]
	/// // And many other options
	/// });
	/// let response = client
	///         .project()
	///         .create(&json)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn create<T: Serialize + ?Sized>(
		&self,
		json_body: &T,
	) -> Result<types::ResProject, AivenError> {
		let url = "project";
		let response = make_json_request!(self, reqwest::Method::POST, url, json_body)?;
		Ok(response.json().await?)
	}

	/// Delete a peering connection for a project VPC
	///
	/// https://api.aiven.io/doc/#api-Project-VpcPeeringConnectionDelete
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .delete_peering_connection("project", "vpc-id", "peer-cloud-account", "peer-vpc")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_peering_connection(
		&self,
		project: &str,
		project_vpc_id: &str,
		peer_cloud_account: &str,
		peer_vpc: &str,
	) -> Result<types::PeeringConnection, AivenError> {
		let url = format!(
			"project/{project}/vpcs/{project_vpc_id}/peering-connections/peer-accounts/\
			 {peer_cloud_account}/peer-vpcs/{peer_vpc}",
			project = encode_param(project),
			project_vpc_id = encode_param(project_vpc_id),
			peer_cloud_account = encode_param(peer_cloud_account),
			peer_vpc = encode_param(peer_vpc),
		);
		let response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(response.json().await?)
	}

	/// Delete a peering connection for a project VPC based on a region
	///
	/// # Arguments
	///
	/// * `Arg1` -
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .delete_peering_connection_with_region("project",
	///            "vpc-id",
	///            "peer-cloud-account",
	///            "peer-vpc",
	///            "region")
	///          .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_peering_connection_with_region(
		&self,
		project: &str,
		project_vpc_id: &str,
		peer_cloud_account: &str,
		peer_vpc: &str,
		peer_region: &str,
	) -> Result<types::PeeringConnection, AivenError> {
		let url = format!(
			"project/{project}/vpcs/{project_vpc_id}/peering-connections/peer-accounts/\
			 {peer_cloud_account}/peer-vpcs/{peer_vpc}/peer-regions/{peer_region}",
			project = encode_param(project),
			project_vpc_id = encode_param(project_vpc_id),
			peer_cloud_account = encode_param(peer_cloud_account),
			peer_vpc = encode_param(peer_vpc),
			peer_region = encode_param(peer_region),
		);
		let response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(response.json().await?)
	}

	/// Delete a project VPC
	///
	/// # Arguments
	///
	/// * `Arg1` -
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .delete_project_vpc("project", "vpc-id")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_project_vpc(
		&self,
		project: &str,
		project_vpc_id: &str,
	) -> Result<types::ProjectVPC, AivenError> {
		let url = format!(
			"project/{project}/vpcs/{project_vpc_id}",
			project = encode_param(project),
			project_vpc_id = encode_param(project_vpc_id),
		);
		let response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(response.json().await?)
	}

	/// Delete an invitation to a project
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .delete_project_invitation("project", "email-id")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_project_invitation(
		&self,
		project: &str,
		invited_email: &str,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/invite/{invited_email}",
			project = encode_param(project),
			invited_email = encode_param(invited_email),
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Delete project
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .delete_project("project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn delete_project(&self, project: &str) -> Result<(), AivenError> {
		let url = format!("project/{project}", project = encode_param(project),);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}

	/// Get VPC information
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `project_vpc_id` - Project VPC ID
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .get_vpc_info("project", "project-vpc-id")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_vpc_info(
		&self,
		project: &str,
		project_vpc_id: &str,
	) -> Result<types::ProjectVPC, AivenError> {
		let url = format!(
			"project/{project}/vpcs/{project_vpc_id}",
			project = encode_param(project),
			project_vpc_id = encode_param(project_vpc_id),
		);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get project details
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectGet
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .get_project_details("project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_project_details(
		&self,
		project: &str,
	) -> Result<types::ResProject, AivenError> {
		let url = format!("project/{project}", project = encode_param(project),);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Get project event log entries
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectGetEventLogs
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .get_event_log_entries("project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn get_event_log_entries(
		&self,
		project: &str,
	) -> Result<types::ResEvents, AivenError> {
		let url = format!("project/{project}/events", project = encode_param(project),);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List VPCs for a project
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .list_vpcs("project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_vpcs(&self, project: &str) -> Result<types::ProjectVPCs, AivenError> {
		let url = format!("project/{project}/vpcs", project = encode_param(project),);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List active alerts for all services in a project
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectAlertsList
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .list_active_alerts("project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_active_alerts(
		&self,
		project: &str,
	) -> Result<types::ActiveAlerts, AivenError> {
		let url = format!("project/{project}/alerts", project = encode_param(project),);
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// List projects
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectList
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .list_projects()
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_projects(&self) -> Result<types::ProjectList, AivenError> {
		let url = "project";
		let response = make_request!(self, reqwest::Method::GET, url)?;
		Ok(response.json().await?)
	}

	/// List users with access to the project. May contain same user multiple
	/// times if they belong to multiple teams associated to the project.
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectUserList
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .list_users_for_project("project")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn list_users_for_project(
		&self,
		project: &str,
	) -> Result<types::ProjectUserList, AivenError> {
		let url = format!("project/{project}/users", project = encode_param(project));
		let response = make_request!(self, reqwest::Method::GET, &url)?;
		Ok(response.json().await?)
	}

	/// Remove user from the project.
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectUserRemove
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `user_email` - Email address
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .remove_user("project", "useremail")
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn remove_user(&self, project: &str, email: &str) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/user/{email}",
			project = encode_param(project),
			email = encode_param(email)
		);
		let _response = make_request!(self, reqwest::Method::DELETE, &url)?;
		Ok(())
	}
	/// Request a VPC in a cloud for the project
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// // for e.g.
	/// let body = json!({
	/// "cloud_name": "aws-eu-central-1",
	/// "network_cidr": "192.168.6.0/24",
	/// "peering_connections": [
	///     {
	///         "peer_azure_app_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
	///         "peer_azure_tenant_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
	///         "peer_cloud_account": "123456789012",
	///         "peer_region": "us-east-1",
	///         "peer_resource_group": "my-peered-rg",
	///         "peer_vpc": "vpc-2f09a348",
	///         "user_peer_network_cidrs": [
	///             "192.168.6.0/24"
	///         ]
	///     }
	///   ]
	/// });
	/// let response = client
	///         .project()
	///         .request_vpc_for_project("project", &body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn request_vpc_for_project<T: Serialize + ?Sized>(
		&self,
		project: &str,
		json_body: &T,
	) -> Result<types::ProjectVPC, AivenError> {
		// ) -> Result<serde_json::Value, AivenError> {
		let url = format!("project/{project}/vpcs", project = encode_param(project),);

		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Request a peering connection for a project VPC
	///
	/// https://api.aiven.io/doc/#api-Project-VpcPeeringConnectionRequest
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let body = json!({
	/// "peer_azure_app_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
	/// "peer_azure_tenant_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
	/// "peer_cloud_account": "123456789012",
	/// "peer_region": "us-east-1",
	/// "peer_resource_group": "my-peered-rg",
	/// "peer_vpc": "vpc-2f09a348",
	/// "user_peer_network_cidrs": [
	///    "192.168.6.0/24"
	///    ]
	/// });
	/// let response = client
	///         .project()
	///         .request_peering_connection("project", "vpc-id", &body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn request_peering_connection<T: Serialize + ?Sized>(
		&self,
		project: &str,
		project_vpc_id: &str,
		json_body: &T,
	) -> Result<types::PeeringConnection, AivenError> {
		// ) -> Result<serde_json::Value, AivenError> {
		let url = format!(
			"project/{project}/vpcs/{project_vpc_id}/peering-connections",
			project = encode_param(project),
			project_vpc_id = encode_param(project_vpc_id),
		);

		let response = make_json_request!(self, reqwest::Method::POST, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Send project membership invitation
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectInvite
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .send_membership_invitation("project", "useremail", Some("developer"))
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn send_membership_invitation(
		&self,
		project: &str,
		user_email: &str,
		member_type: Option<&str>,
	) -> Result<(), AivenError> {
		let url = format!("project/{project}/invite", project = encode_param(project),);
		let mut json_body = HashMap::new();
		json_body.insert(
			"member_type",
			member_type.unwrap_or("developer").to_string(),
		);
		json_body.insert("user_email", user_email.to_string());
		let data = &json_body;
		let _response = make_json_request!(self, reqwest::Method::POST, &url, data)?;
		Ok(())
	}

	/// Update a project user.
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectUserUpdate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	/// * `user_email` - Email address
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// use aiven_rs::project::types::MemberType;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let response = client
	///         .project()
	///         .update_user("project", "useremail", MemberType::Admin)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_user(
		&self,
		project: &str,
		user_email: &str,
		member_type: types::MemberType,
	) -> Result<(), AivenError> {
		let url = format!(
			"project/{project}/user/{user_email}",
			project = encode_param(project),
			user_email = encode_param(user_email),
		);
		let mut json_body = HashMap::new();
		json_body.insert("member_type", member_type.to_string());
		let data = &json_body;
		let _response = make_json_request!(self, reqwest::Method::PUT, &url, data)?;
		Ok(())
	}

	/// Update project
	///
	/// https://api.aiven.io/doc/#api-Project-ProjectUpdate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// use std::collections::HashMap;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let mut json_body = HashMap::new();
	/// json_body.insert("account_id", "some_account_id".to_string());
	/// // .. and more json options
	/// let response = client
	///         .project()
	///         .update_project("some-project", &json_body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_project<T: Serialize + ?Sized>(
		&self,
		project: &str,
		json_body: &T,
	) -> Result<types::ResProject, AivenError> {
		let url = format!("project/{project}", project = encode_param(project),);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(response.json().await?)
	}

	/// Update user-defined peer network CIDRs for a project VPC
	///
	/// https://api.aiven.io/doc/#api-Project-VpcPeeringConnectionUpdate
	///
	/// # Arguments
	///
	/// * `project` - Project name
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust,no_run
	/// use serde_json::json;
	/// #[tokio::main]
	/// async fn main()-> Result<(), Box<dyn std::error::Error>>{
	/// let client = aiven_rs::AivenClient::from_token("https://api.aiven.io", "v1", "aiven-token");
	/// let json_body = json!({
	/// "add": [
	///     {
	///         "cidr": "192.168.6.0/24",
	///         "peer_cloud_account": "123456789012",
	///         "peer_resource_group": "my-peered-rg",
	///         "peer_vpc": "vpc-2f09a348"
	///     }
	/// ],
	/// "delete": [
	///     "192.168.6.0/24"
	/// ]
	/// });
	/// let response = client
	///         .project()
	///         .update_userdefined_network_cidrs("some-project", "project_vpc_id", &json_body)
	///         .await?;
	/// Ok(())
	/// }
	/// ```
	pub async fn update_userdefined_network_cidrs<T: Serialize + ?Sized>(
		&self,
		project: &str,
		project_vpc_id: &str,
		json_body: &T,
	) -> Result<types::ProjectVPC, AivenError> {
		let url = format!(
			"project/{project}/vpcs/{project_vpc_id}/user-peer-network-cidrs",
			project = encode_param(project),
			project_vpc_id = encode_param(project_vpc_id),
		);
		let response = make_json_request!(self, reqwest::Method::PUT, &url, json_body)?;
		Ok(response.json().await?)
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::testutil;
	use serde_json::json;

	#[tokio::test]
	async fn test_project_confirm_project_invite() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/some-project/invite/some-verif-code";
		let test_data =
			testutil::get_test_data("tests/testdata/project/confirm_project_invite.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.project()
			.confirm_project_invite("some-project", "some-verif-code")
			.await
		{
			Ok(response) => {
				assert!(
					response.invite_details.user_email != "",
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_create() {
		let client = testutil::prepare_test_client();
		let query_url = "/project";
		let test_data = testutil::get_test_data("tests/testdata/project/create_project.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		let json = json!({
			"account_id": "a22ba494e096",
			"billing_address": "ACME Corporation, Main Street 1, Monument Valley, UT",
			"billing_currency": "USD",
			"billing_emails": [
				{
					"email": "jane@example.com"
				}
			],
			"billing_extra_text": "Purchase order: PO100018",
			"card_id": "9330c086-8781-11e5-89ff-5404a64abfef",
			"cloud": "aws-eu-central-1",
			"copy_from_project": "myexistingproject",
			"country_code": "FI",
			"project": "myproject",
			"tech_emails": [
				{
					"email": "jane@example.com"
				}
			],
			"vat_id": "FI27957435"
		});
		match client.project().create(&json).await {
			Ok(response) => {
				assert!(
					response.project.account_id == Some("a22ba494e096".into()),
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
	#[tokio::test]
	async fn test_project_delete_peering_connection() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs/myvpc/peering-connections/peer-accounts/\
		                 mypeercloudaccount/peer-vpcs/mypeervpc";

		let test_data =
			testutil::get_test_data("tests/testdata/project/delete_peering_connection.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.project()
			.delete_peering_connection("myproject", "myvpc", "mypeercloudaccount", "mypeervpc")
			.await
		{
			Ok(response) => {
				assert!(
					response.peer_cloud_account == "123456789012".to_string(),
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_delete_peering_connection_with_region() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs/myvpc/peering-connections/peer-accounts/\
		                 mypeercloudaccount/peer-vpcs/mypeervpc/peer-regions/mypeerregion";

		let test_data = testutil::get_test_data(
			"tests/testdata/project/delete_peering_connection_with_region.json",
		);
		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.project()
			.delete_peering_connection_with_region(
				"myproject",
				"myvpc",
				"mypeercloudaccount",
				"mypeervpc",
				"mypeerregion",
			)
			.await
		{
			Ok(response) => {
				assert!(
					response.peer_cloud_account == "123456789012".to_string(),
					format!("{:?}", response)
				);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_delete_project_vpc() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs/myprojectvpcid";

		let test_data = testutil::get_test_data("tests/testdata/project/delete_project_vpc.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.project()
			.delete_project_vpc("myproject", "myprojectvpcid")
			.await
		{
			Ok(response) => {
				assert!(response.peering_connections.unwrap().len() > 0,);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_delete_project_invitation() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/invite/myinvitedemail";

		let test_data = "";
		let _m = testutil::create_mock_server(query_url, test_data, "DELETE");

		match client
			.project()
			.delete_project_invitation("myproject", "myinvitedemail")
			.await
		{
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_delete_project() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject";

		let test_data = "";
		let _m = testutil::create_mock_server(query_url, test_data, "DELETE");

		match client.project().delete_project("myproject").await {
			Ok(_) => {
				assert!(true);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_get_vpc_info() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs/myprojectvpcid";

		let test_data = testutil::get_test_data("tests/testdata/project/get_vpc_info.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client
			.project()
			.get_vpc_info("myproject", "myprojectvpcid")
			.await
		{
			Ok(response) => {
				assert!(response.cloud_name == "aws-eu-central-1".to_string());
				assert!(response.peering_connections.unwrap().len() > 0);
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_get_project_details() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject";

		let test_data = testutil::get_test_data("tests/testdata/project/get_project_details.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.project().get_project_details("myproject").await {
			Ok(response) => {
				assert!(response.project.billing_currency == "USD".to_string());
				assert!(response.project.tenant_id == "aiven".to_string());
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_get_event_log_entries() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/events";

		let test_data =
			testutil::get_test_data("tests/testdata/project/get_event_log_entries.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.project().get_event_log_entries("myproject").await {
			Ok(response) => {
				assert!(response.events.len() > 0);
				assert!(response.events[0].actor == "user@example.com".to_string());
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_list_project_vpc() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs";

		let test_data = testutil::get_test_data("tests/testdata/project/list_project_vpc.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.project().list_vpcs("myproject").await {
			Ok(response) => {
				assert!(response.vpcs.len() > 0);
				assert!(response.vpcs[0].cloud_name == "aws-eu-central-1".to_string());
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_list_active_alerts() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/alerts";

		let test_data = testutil::get_test_data("tests/testdata/project/list_active_alerts.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.project().list_active_alerts("myproject").await {
			Ok(response) => {
				assert!(response.alerts.len() > 0);
				assert!(response.alerts[0].event == "disk_usage".to_string());
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_list_users_for_project() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/users";

		let test_data =
			testutil::get_test_data("tests/testdata/project/list_users_for_project.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "GET");

		match client.project().list_users_for_project("myproject").await {
			Ok(response) => {
				assert!(response.invitations.len() > 0);
				assert!(response.users.len() > 0);
				assert!(response.users[0].auth[0] == "2fa-otp".to_string());
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_remove_user() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/user/myemailtoremove";

		let test_data = "".to_owned();

		let _m = testutil::create_mock_server(query_url, &test_data, "DELETE");

		match client
			.project()
			.remove_user("myproject", "myemailtoremove")
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_request_vpc_for_project() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs";

		let test_data =
			testutil::get_test_data("tests/testdata/project/request_vpc_for_project.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let json_body = json!({
			"cloud_name": "aws-eu-central-1",
			"network_cidr": "192.168.6.0/24",
			"peering_connections": [
				{
					"peer_azure_app_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
					"peer_azure_tenant_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
					"peer_cloud_account": "123456789012",
					"peer_region": "us-east-1",
					"peer_resource_group": "my-peered-rg",
					"peer_vpc": "vpc-2f09a348",
					"user_peer_network_cidrs": [
						"192.168.6.0/24"
					]
				}
			]
		});
		match client
			.project()
			.request_vpc_for_project("myproject", &json_body)
			.await
		{
			Ok(response) => {
				assert!(response.cloud_name == "aws-eu-central-1".to_string());
				assert!(response.state == "ACTIVE".to_string())
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_request_peering_connection() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs/myvpcid/peering-connections";

		let test_data =
			testutil::get_test_data("tests/testdata/project/request_peering_connection.json");

		let _m = testutil::create_mock_server(query_url, &test_data, "POST");
		let json_body = json!({
			"peer_azure_app_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
			"peer_azure_tenant_id": "adcf7194-d877-4505-a47a-91fefd96e3b8",
			"peer_cloud_account": "123456789012",
			"peer_region": "us-east-1",
			"peer_resource_group": "my-peered-rg",
			"peer_vpc": "vpc-2f09a348",
			"user_peer_network_cidrs": [
				"192.168.6.0/24"
			]
		});
		match client
			.project()
			.request_peering_connection("myproject", "myvpcid", &json_body)
			.await
		{
			Ok(response) => {
				assert!(response.peer_cloud_account == "123456789012".to_string());
				assert!(response.state == "ACTIVE".to_string())
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_send_membership_invitation() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/invite";

		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "POST");

		match client
			.project()
			.send_membership_invitation("myproject", "myuseremail", Some("admin"))
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_update_user() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/user/myuseremail";

		let test_data = "".to_string();
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		match client
			.project()
			.update_user("myproject", "myuseremail", types::MemberType::Developer)
			.await
		{
			Ok(_) => assert!(true),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_update_project() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject";

		let test_data = testutil::get_test_data("tests/testdata/project/update_project.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let json_body = json!({
			"account_id": "a22ba494e096",
			"billing_address": "ACME Corporation, Main Street 1, Monument Valley, UT",
			"billing_currency": "USD",
			"billing_emails": [
				{
					"email": "jane@example.com"
				}
			],
			"billing_extra_text": "Purchase order: PO100018",
			"card_id": "9330c086-8781-11e5-89ff-5404a64abfef",
			"cloud": "aws-eu-central-1",
			"country_code": "FI",
			"tech_emails": [
				{
					"email": "jane@example.com"
				}
			],
			"vat_id": "FI27957435"
		});
		match client
			.project()
			.update_project("myproject", &json_body)
			.await
		{
			Ok(response) => assert!(response.project.card_info.brand == "Visa"),
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}

	#[tokio::test]
	async fn test_project_update_userdefined_network_cidrs() {
		let client = testutil::prepare_test_client();
		let query_url = "/project/myproject/vpcs/myvpcid/user-peer-network-cidrs";

		let test_data =
			testutil::get_test_data("tests/testdata/project/update_userdefined_network_cidrs.json");
		let _m = testutil::create_mock_server(query_url, &test_data, "PUT");

		let json_body = json!({
			"add": [
				{
					"cidr": "192.168.6.0/24",
					"peer_cloud_account": "123456789012",
					"peer_resource_group": "my-peered-rg",
					"peer_vpc": "vpc-2f09a348"
				}
			],
			"delete": [
				"192.168.6.0/24"
			]
		});
		match client
			.project()
			.update_userdefined_network_cidrs("myproject", "myvpcid", &json_body)
			.await
		{
			Ok(response) => {
				assert!(response.cloud_name == "aws-eu-central-1");
				assert!(response.state == "ACTIVE");
			}
			Err(e) => assert!(false, format!("{:?}", e)),
		}
	}
}
