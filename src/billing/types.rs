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
pub struct ResCredit {
	pub credit: Credit,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Credit {
	pub code: String,
	pub remaining_value: String,

	/// Allowed values: "discount", "employee", "evaluation", "internal",
	/// "other", "outage", "purchase", "sponsorship", "trial", "trial_over"
	#[serde(rename = "type")]
	pub credit_type: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Invoice {
	pub currency: String,
	pub download_cookie: String,
	pub invoice_number: String,
	pub period_begin: String,
	pub period_end: String,
	pub state: String,
	pub total_inc_vat: String,
	pub total_vat_zero: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Invoices {
	pub invoices: Vec<Invoice>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Credits {
	pub credits: Vec<Credit>,
}
