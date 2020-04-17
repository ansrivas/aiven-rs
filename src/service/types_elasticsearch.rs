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
pub struct Index {
	pub create_time: String,
	pub docs: i64,
	pub health: String,
	pub index_name: String,
	pub number_of_replicas: i64,
	pub number_of_shards: i64,
	pub read_only_allow_delete: bool,
	pub size: u64,
	pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Indexes {
	pub indexes: Vec<Index>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Rule {
	pub index: String,
	pub permission: String,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Acl {
	pub rules: Vec<Rule>,
	pub username: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ElasticSearchConfig {
	pub acls: Vec<Acl>,
	pub enabled: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ElasticSearchACLConfig {
	pub elasticsearch_acl_config: ElasticSearchConfig,
}
