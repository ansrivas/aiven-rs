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
use serde_json;
use std::collections::HashMap;
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Ticket {
	pub more_info: String,
	pub status: i32,
    pub ticket: HashMap<String, serde_json::Value>,
    pub create_time: String,
    pub description: String,
    pub followers: Vec<serde_json::Value>,
    pub real_name: String,
    pub user_email: String,
    pub user_id: String,
    pub following: bool,
    pub project_name: String,
    pub service_name: String,
    pub severity: String,
    pub state: String,
    pub submitter: serde_json::Value,
    pub ticket_id: String,
    pub title: String,
    pub update_time: String,
    pub user_real_name: String,

}