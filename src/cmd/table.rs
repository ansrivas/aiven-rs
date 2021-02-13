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

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn print_table(header: Vec<&str>, rows: Vec<Row>) {
	// This part now can be extracted out
	let mut table = Table::new();
	table
		.set_style(TableComponent::HeaderLines, ' ')
		.set_style(TableComponent::HorizontalLines, ' ')
		.set_style(TableComponent::VerticalLines, ' ')
		.load_preset(UTF8_FULL)
		.set_content_arrangement(ContentArrangement::Dynamic)
		.set_header(header);

	for row in rows {
		table.add_row(row);
	}
	println!("{}", table);
}
