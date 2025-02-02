use include_json::include_json;

fn main() {
    let _ = include_json!(concat!(env!("CARGO_MANIFEST_DIR"), "/nonexistent.json"));
}
