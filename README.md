<div align="center">
 <p><h1>aiven-rs</h1> </p>
  <p><strong>An async rust-sdk for Aiven</strong> </p>
<p>

<div style="display: flex; justify-content: center;">
  <a href="https://www.rust-lang.org/"><img style="width:40%;" src="assets/rust.png" alt="rust" border="0" /></a>
  <a href="https://aiven.io/"><img style="width:33%;" src="assets/aiven.png" alt="aiven" border="0" /></a>
</div>

![Linux](https://github.com/ansrivas/aiven-rs/workflows/Linux/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/aiven_rs)](https://crates.io/crates/aiven_rs)
[![Docs.rs](https://docs.rs/aiven_rs/badge.svg)](https://docs.rs/aiven_rs)
[![codecov](https://codecov.io/gh/ansrivas/aiven-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/ansrivas/aiven-rs)

</p>
</div>
</br>

## Resources

- Build Status on other [platforms](./BUILD_STATUS.md)
- Majority of the end points from https://api.aiven.io/doc/ have been implemented as of [07.01.2021]
- [Documentation](https://docs.rs/aiven-rs)

## Getting Started

```rust
// tokio = "1.0"
// aiven_rs = "0.4.0"

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
## Running the examples:
```sh
RUST_LOG=aiven_rs=debug cargo run --example clouds
```
## License

This project is licensed under
- MIT license ([LICENSE-MIT](LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
