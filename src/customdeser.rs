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

use serde::{
	de::{self, Deserializer, Unexpected},
	Deserialize,
};
use std::{fmt::Display, str::FromStr};

/// Deserialize bool from String with custom value mapping
pub(crate) fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	match String::deserialize(deserializer)?.as_ref() {
		"true" => Ok(true),
		"false" => Ok(false),
		other => Err(de::Error::invalid_value(
			Unexpected::Str(other),
			&"true or false",
		)),
	}
}

pub(crate) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T: FromStr,
	T::Err: Display,
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	T::from_str(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[derive(Deserialize, Debug)]
	struct TestStructBool {
		#[serde(deserialize_with = "bool_from_string")]
		val_true: bool,
		#[serde(deserialize_with = "bool_from_string")]
		val_false: bool,
	}
	#[derive(Deserialize, Eq, PartialEq, Debug)]
	struct TestStructInt {
		#[serde(deserialize_with = "from_str")]
		val_i32: i32,
		#[serde(deserialize_with = "from_str")]
		val_u16: u16,
		#[serde(deserialize_with = "from_str")]
		val_i64: i64,
	}

	#[test]
	fn test_int_from_string() {
		let response: TestStructInt =
			serde_json::from_str("{\"val_i32\":\"1\", \"val_i64\":\"1\", \"val_u16\":\"1\" }")
				.unwrap();
		assert!(response.val_i32 == 1);
		assert!(response.val_u16 == 1);
		assert!(response.val_i64 == 1);
	}
	#[test]
	fn test_bool_from_string() {
		let response: TestStructBool =
			serde_json::from_str("{\"val_true\":\"true\",\"val_false\":\"false\" }").unwrap();
		assert!(response.val_true);
		assert!(!response.val_false);
	}

	#[test]
	#[should_panic]
	fn test_bool_from_string_panic() {
		let response: TestStructBool =
			serde_json::from_str("{\"val_true\":\"tru\",\"val_false\":\"false\" }").unwrap();
		assert!(response.val_true);
	}
}
