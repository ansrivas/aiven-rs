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

use crate::client::APIError;
use std::io;
use thiserror::Error;
use url::ParseError;

/// Default Error enum which provides translation between std error to different
/// error types
#[derive(Error, Debug)]
pub enum AivenError {
	// #[error("Failed to get a db-connection from database pool")]
	// PoolConnError(#[from] PoolError),
	#[error("HTTP Request error")]
	ReqwestError(#[from] reqwest::Error),

	#[error("Failed during URL parsing")]
	URLParseError(#[from] ParseError),

	#[error("Failed during IO operation")]
	IOError(#[from] io::Error),
	//     backtrace: BackTrace
	// },
	#[error("Unsupported method ")]
	UnsupportedMethod,

	#[error("Failed during http request with status_code `{status_code:?}` and text `{text:?}`")]
	ReqwestErrorWithStatus { status_code: String, text: String },

	#[error("Failed during Serde operation")]
	SerdeError(#[from] serde_json::Error),

	#[error("Failed during parsing APIResponse")]
	APIResponseError {
		errors: Vec<APIError>,
		message: String,
	},
}
