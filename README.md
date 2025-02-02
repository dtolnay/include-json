`include_json!`
===============

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/include--json-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/include-json)
[<img alt="crates.io" src="https://img.shields.io/crates/v/include_json.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/include_json)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-include_json-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/include_json)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/include-json/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/include-json/actions?query=branch%3Amaster)

Rust macro to parse a JSON file at compile time and compile it into the program
as a `serde_json::Value`.

Example &mdash; supplying a JSON file as context inside a [MiniJinja] template:

[MiniJinja]: https://github.com/mitsuhiko/minijinja

```rust
use include_json::include_json;

fn main() {
    let pkg = include_json!(concat!(env!("CARGO_MANIFEST_DIR"), "/package.json"));

    let mut env = minijinja::Environment::new();
    env.add_template("example", include_str!("example.jinja")).unwrap();
    let tmpl = env.get_template("example").unwrap();
    println!("{}", tmpl.render(minijinja::context!(pkg)).unwrap());
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
