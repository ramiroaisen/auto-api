use axum::{response::Html, routing::get, Json};

#[tokio::main]
async fn main() {
  let registry = auto_api::api::registry();

  let openapi = registry.openapi_spec();
  let api = registry.axum_router();

  let app = axum::Router::new()
    .route("/", get(Html(redoc())))
    .route("/swagger", get(Html(swagger())))
    .route("/openapi.json", get(Json(openapi)))
    .nest("/", api);

  let listener = tokio::net::TcpListener::bind("127.0.0.1:6985")
    .await
    .expect("tcp listener bind");

  println!("listening on http://127.0.0.1:6985");

  axum::serve(listener, app)
    .await
    .expect("axum serve");
}

fn redoc() -> &'static str {
r###"<!doctype html>
<html lang="en">
  <head>
    <title>Example API</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <redoc spec-url="./openapi.json"></redoc>
    <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
  </head>
  <body>
  </body>
</html>
"###
}

fn swagger() -> &'static str {
r###"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="//unpkg.com/swagger-ui-dist@3/swagger-ui.css">
    <title>Your App API v1</title>
  </head>
  <body>
    <div id="docs" />
    <script src="https://unpkg.com/swagger-ui-dist@3/swagger-ui-bundle.js"></script>
    <script>
      const ui = SwaggerUIBundle({
        url: "./openapi.json",
        dom_id: "#docs",
        deepLinking: true,
      })
    </script>
  </body>
</html>"###
}