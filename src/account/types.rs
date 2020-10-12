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
	pub authentication_method_enabled: String,
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

// {

//     "authentication_method":

// {

//     "account_id": "string",
//     "authentication_method_enabled": true,
//     "authentication_method_id": "string",
//     "authentication_method_name": "string",
//     "authentication_method_type": "string",
//     "auto_join_team_id": "string",
//     "create_time": "string",
//     "delete_time": "string",
//     "saml_acs_url": "string",
//     "saml_certificate": "string",
//     "saml_certificate_issuer": "string",
//     "saml_certificate_not_valid_after": "string",
//     "saml_certificate_not_valid_before": "string",
//     "saml_certificate_subject": "string",
//     "saml_digest_algorithm": "string",
//     "saml_entity_id": "string",
//     "saml_field_mapping":

//     {
//         "email": "string",
//         "first_name": "string",
//         "identity": "string",
//         "last_name": "string",
//         "real_name": "string"
//     },
//     "saml_idp_url": "string",
//     "saml_metadata_url": "string",
//     "saml_signature_algorithm": "string",
//     "saml_variant": "string",
//     "state": "string",
//     "update_time": "string"

// },
// "errors":
// [

//         {}
//     ],
//     "message": "string"

// }
