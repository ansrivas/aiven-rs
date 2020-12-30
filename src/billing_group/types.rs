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
pub struct Email {
	pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BillingGroup {
	pub account_id: String,
	pub account_name: String,
	pub address_lines: Vec<String>,
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
	pub estimated_balance_local: String,
	pub estimated_balance_usd: String,
	pub payment_method: String,
	pub state: String,
	pub vat_id: String,
	pub zip_code: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResponseBillingGroup {
	pub billing_group: BillingGroup,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResponseBillingGroups {
	pub billing_groups: Vec<BillingGroup>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CardInfo {
	pub brand: String,
	pub card_id: String,
	pub country: String,
	pub country_code: String,
	pub exp_month: u32,
	pub exp_year: u32,
	pub last4: String,
	pub name: String,
	pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResponseClaimCredit {
	pub credit: Credit,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ResponseCredits {
	pub credits: Vec<Credit>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Credit {
	pub code: String,
	pub remaining_value: String,
	#[serde(rename = "type")]
	pub credit_type: String,
}
