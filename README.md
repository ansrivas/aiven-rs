<div align="center">
 <p><h1>aiven-rs</h1> </p>
  <p><strong>An async rust-sdk for Aiven</strong> </p>
<p>

<div style="display: flex; justify-content: center;">
  <a href="https://www.rust-lang.org/"><img style="width:40%;" src="assets/rust.png" alt="rust" border="0" /></a>
  <a href="https://aiven.io/"><img style="width:75%;" src="assets/aiven.png" alt="aiven" border="0" /></a>
</div>


![Linters](https://github.com/ansrivas/aiven-rs/workflows/Linters/badge.svg)
![Linux](https://github.com/ansrivas/aiven-rs/workflows/Linux/badge.svg)
![MacOS](https://github.com/ansrivas/aiven-rs/workflows/MacOS/badge.svg)
![Windows](https://github.com/ansrivas/aiven-rs/workflows/Windows/badge.svg)
[![Version](https://meritbadge.herokuapp.com/aiven_rs)](https://crates.io/crates/aiven_rs)
[![Docs.rs](https://docs.rs/aiven_rs/badge.svg)](https://docs.rs/aiven_rs)

</p>
</div>
</br>

## Resources

* Majority of the end points from https://api.aiven.io/doc/ have been implemented as of [17.04.2020]
* [Documentation](https://docs.rs/aiven-rs)
* Minimum supported Rust version: 1.42 or later

## Getting Started
```rust
use aiven_rs::{cloud::types::ResClouds, AivenClient};

#[tokio::main]
async fn main() {
  env_logger::init();
  // use std::env;
  // 
  // let token = env::var("AIVEN_TOKEN").expect("Please set env variable to read AIVEN_TOKEN");
  // let client = AivenClient::from_token("https://api.aiven.io", "v1", &token);

  let client = AivenClient::new("https://api.aiven.io", "v1");
  let cloud_api = client.cloud();
  let output: ResClouds = cloud_api.list_all().await.unwrap();
  for cloud in &output.clouds {
    println!("{:?}", cloud.cloud_name);
  }
}
```

## License

This project is licensed under

* MIT license ([LICENSE-MIT](LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

