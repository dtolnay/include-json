use include_json::include_json;
use serde_json::json;

#[test]
fn test() {
    let actual = include_json!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.json"));
    let expected = json!({
        "null": null,
        "true": true,
        "false": false,
        "posint": 10,
        "negint": -10,
        "float": 10.0,
        "string": "...",
        "emptyarray": [],
        "array": [1, 2, 3],
        "emptyobject": {}
    });
    assert_eq!(actual, expected);
}
