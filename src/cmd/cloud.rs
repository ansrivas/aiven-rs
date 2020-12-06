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

use crate::print_table;
use aiven_rs::cloud::types::ResClouds;
use anyhow::Result;
use comfy_table::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
/// Cloud commands.
pub struct Cloud {
	#[structopt(subcommand)]
	commands: SubCommandsCloud,
}

#[derive(StructOpt, Debug, PartialEq)]
#[structopt()]
pub enum SubCommandsCloud {
	/// List cloud types
	#[structopt(name = "list")]
	List {
		#[structopt(long)]
		project: Option<String>,

		/// raw json output
		#[structopt(long)]
		json: bool,
	},
}

pub async fn handle_cloud_list(
	client: &aiven_rs::AivenClient,
	cloud: Cloud,
) -> Result<(), aiven_rs::errors::AivenError> {
	let cloud_api = &client.cloud();

	match &cloud.commands {
		SubCommandsCloud::List { project: _, json } => {
			let output: ResClouds = cloud_api.list_all().await?;
			if *json {
				println!("{}", serde_json::to_string_pretty(&output.clouds)?);
				return Ok(());
			}

			let header = vec![
				"CLOUD_DESCRIPTION",
				"CLOUD_NAME",
				"GEO_LATITUDE",
				"GEO_LONGITUDE",
				"GEO_REGION",
			];
			let rows: Vec<Row> = output
				.clouds
				.into_iter()
				.map(|cloud| {
					Row::from(vec![
						cloud.cloud_description.clone(),
						cloud.cloud_name.clone(),
						cloud.geo_latitude.to_string(),
						cloud.geo_longitude.to_string(),
						cloud.geo_region.clone(),
					])
				})
				.collect();

			print_table(header, rows);
		}
	}

	Ok(())
}
