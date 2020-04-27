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
pub struct ServiceIntegrationEndPoint {
	pub endpoint_config: serde_json::Value,
	pub endpoint_id: String,
	pub endpoint_name: String,
	pub endpoint_type: String,
	pub user_config: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ServiceIntegration {
	pub active: bool,
	pub description: String,
	pub dest_endpoint: String,
	pub dest_endpoint_id: String,
	pub dest_project: String,
	pub dest_service: String,
	pub dest_service_type: String,
	pub enabled: bool,
	pub integration_status: serde_json::Value,
	pub integration_type: String,
	pub service_integration_id: String,
	pub source_endpoint: String,
	pub source_endpoint_id: String,
	pub source_project: String,
	pub source_service: String,
	pub source_service_type: String,
	pub user_config: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct IntegrationType {
	pub dest_description: String,
	pub dest_service_type: String,
	pub dest_service_types: Vec<String>,
	pub integration_type: String,
	pub source_description: String,
	pub source_service_types: Vec<String>,
	pub user_config_schema: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResIntegrationTypes {
	pub integration_types: Vec<IntegrationType>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceIntegration {
	pub service_integration: ServiceIntegration,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceIntegrations {
	pub service_integrations: Vec<ServiceIntegration>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceIntegrationEndPoint {
	pub service_integration_endpoint: ServiceIntegrationEndPoint,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResServiceIntegrationEndPoints {
	pub service_integration_endpoints: Vec<ServiceIntegrationEndPoint>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct EndpointType {
	pub endpoint_type: String,
	pub service_types: Vec<String>,
	pub title: String,
	pub user_config_schema: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResEndpointTypes {
	pub endpoint_types: Vec<EndpointType>,
}
