`include_json!`
===============

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
