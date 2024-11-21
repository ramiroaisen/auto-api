use std::path::Path;

#[tokio::main]
async fn main() {
  let registry = auto_api::api::registry();

  let openapi_spec = registry.openapi_spec();
  let openapi_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("generated/openapi.json");
  std::fs::write(&openapi_path, serde_json::to_string_pretty(&openapi_spec).unwrap()).expect("error writing openapi spec");
  println!("openapi spec written to {}", openapi_path.display());

  let ts_defs = registry.ts_definitions();
  let ts_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("generated/api.ts");
  std::fs::write(&ts_path, &ts_defs).expect("error writing ts definitions");
  println!("ts definitions written to {}", ts_path.display());
}