use core::panic;
use std::{convert::Infallible, sync::Arc};
use axum::response::IntoResponse;
use axum::routing::MethodRouter;
use axum::{async_trait, extract::Request, http::Method, response::Response, routing::MethodFilter};
use indexmap::IndexMap;
use schemars::{generate::SchemaSettings, Schema as SchemarsSchema};
use serde_json::json;
use shape::{Shape, ShapeOptions, ToTypescript};

use crate::response::into_json_response;
use crate::endpoint::Endpoint;
use crate::schema::Schema;

#[async_trait]
pub trait RegistryHandler: Send + Sync + 'static {
  async fn handle(&self, request: Request) -> Response;
}

pub struct RegistryHandlerItem<T>(pub T);

#[async_trait]
impl<
  Ctx: Send,
  Params: Schema + Send,
  Query: Schema + Send,
  Payload: Schema + Send,
  Output: Schema + Send,
  T: Endpoint<
    Ctx=Ctx,
    Params=Params,
    Query=Query,
    Payload=Payload,
    Output=Output
  >
> RegistryHandler for RegistryHandlerItem<T> {

  async fn handle(&self, req: Request) -> Response {
    match self.0.handle(req).await {
      Ok(out) => into_json_response(out),
      Err(err) => err.into_response(),
    }
  }
}


#[derive(Clone)]
pub struct RegistryItem {
  pub path: String,
  
  pub method: Method,

  pub params: Option<SchemarsSchema>,
  pub query: Option<SchemarsSchema>,
  pub payload: Option<SchemarsSchema>,
  pub output: SchemarsSchema,

  pub params_shape: Option<shape::Type>,
  pub query_shape: Option<shape::Type>,
  pub payload_shape: Option<shape::Type>,
  pub output_shape: shape::Type,

  pub handler: Arc<dyn RegistryHandler>,
}

fn shape_params_options() -> ShapeOptions {
  ShapeOptions {
    kind: shape::ShapeOptionsKind::Deserialize,
    option_add_null: false,
    option_add_undefined: false,
    option_is_optional: true,
  }
}

fn shape_query_options() -> ShapeOptions {
  ShapeOptions {
    kind: shape::ShapeOptionsKind::Deserialize,
    option_add_null: false,
    option_add_undefined: true,
    option_is_optional: true,
  }
}

fn shape_payload_options() -> ShapeOptions {
  ShapeOptions {
    kind: shape::ShapeOptionsKind::Deserialize,
    option_add_null: true,
    option_add_undefined: true,
    option_is_optional: true,
  }
}

fn shape_output_options() -> ShapeOptions {
  ShapeOptions {
    kind: shape::ShapeOptionsKind::Serialize,
    option_add_null: true,
    option_add_undefined: false,
    option_is_optional: false,
  }
}

#[derive(Clone)]
pub struct Registry {
  // { key: Path => { key: Method => Item }
  error_payload_schema: schemars::Schema,
  error_payload_shape: shape::Type,
  pub map: IndexMap<String, IndexMap<Method, RegistryItem>>,
}


impl Registry {
  pub fn new<ErrorPayload: Schema>() -> Self {
    let mut error_payload_settings = SchemaSettings::openapi3()
      .for_serialize();
    error_payload_settings.inline_subschemas = true;
    error_payload_settings.option_nullable = true;
    error_payload_settings.option_add_null_type = true;

    let error_payload_schema = ErrorPayload::json_schema(&mut error_payload_settings.into_generator());
    let error_payload_shape = ErrorPayload::shape(&shape_output_options());

    Self {
      error_payload_schema,
      error_payload_shape,
      map: IndexMap::new(),
    }
  }

  pub fn register<
    Ctx: Send,
    Params: Schema + Send,
    Query: Schema + Send,
    Payload: Schema + Send,
    Output: Schema + Send,
    T: Endpoint<
      Ctx=Ctx,
      Params=Params,
      Query=Query,
      Payload=Payload,
      Output=Output
    >
  >(&mut self, endpoint: T) {
    let path = endpoint.path();
    let method_map = self.map.entry(path.to_string()).or_default();
    let method = endpoint.method();
    match method_map.entry(method.clone()) {
      indexmap::map::Entry::Occupied(_) => {
        panic!("duplicate endpoint registered for path `{path}` and method `{method}`", );
      }

      indexmap::map::Entry::Vacant(entry) => {
        
        let params = if Params::is_void() {
          None
        } else {
          let mut params_settings = SchemaSettings::openapi3()
            .for_deserialize();
          params_settings.option_add_null_type = false;
          params_settings.option_nullable = false;
          params_settings.inline_subschemas = true;

          Some(Params::json_schema(&mut params_settings.into_generator()))
        };

        let query = if Query::is_void() {
          None
        } else {
          // query
          let mut query_settings = SchemaSettings::openapi3()
            .for_deserialize();
          query_settings.option_add_null_type = true;
          query_settings.option_nullable = true;
          query_settings.inline_subschemas = true;

          Some(Query::json_schema(&mut query_settings.into_generator()))
        };

        let payload = if Payload::is_void() {
          None
        } else {
          // payload
          let mut payload_settings = SchemaSettings::openapi3()
            .for_deserialize();
          payload_settings.option_add_null_type = true;
          payload_settings.option_nullable = true;
          payload_settings.inline_subschemas = true;

          Some(Payload::json_schema(&mut payload_settings.into_generator()))
        };

        // output
        let mut output_settings = SchemaSettings::openapi3()
          .for_serialize();
        output_settings.option_add_null_type = true;
        output_settings.option_nullable = true;
        output_settings.inline_subschemas = true;
        
        let output = Output::json_schema(&mut output_settings.into_generator());

        let params_shape = if Params::is_void() {
          None
        } else {
          Some(Params::shape(&shape_params_options()))
        };

        let query_shape = if Query::is_void() {
          None
        } else {
          Some(Query::shape(&shape_query_options()))
        };

        let payload_shape = if Payload::is_void() {
          None
        } else {
          Some(Payload::shape(&shape_payload_options()))
        };
        
        let output_shape = Output::shape(&shape_output_options());

        let item = RegistryItem {
          path: path.to_string(),
          method,
          params,
          query,
          payload,
          output,
          params_shape,
          query_shape,
          payload_shape,
          output_shape,
          handler: Arc::new(RegistryHandlerItem(endpoint)),
        };

        entry.insert(item);
      }
    }
  }

  pub fn ts_definitions(&self) -> String {
    let mut def = String::new();

    #[derive(Shape)]
    #[allow(clippy::upper_case_acronyms)]
    #[allow(unused)]
    enum Method {
      HEAD,
      GET,
      POST,
      PUT,
      PATCH,
      DELETE,
      OPTIONS,
      CONNECT,
      TRACE,
    }

    let ts_method = Method::shape(&shape::ShapeOptions::for_serialize()).to_typescript();

    let error_payload_src = self.error_payload_shape.to_typescript();

    def.push_str(&format!(
r#"/// this file is auto generated by its Rust definition, do not edit manually

export type Method = {ts_method};
type _Method = Method;

export type Empty = Record<string, never>;

export type Endpoint<
  Method extends _Method,
  Path extends string,
  Params,
  Query,
  Payload,
  Output
> = {{
  method: Method,
  path: Path,
  // this $ types are never constructed, only used as a template
  $params?: Params,
  $query?: Query,
  $payload?: Payload
  $output?: Output
}};

type ApiDefinition = Partial<Record<string, Partial<Record<Method, Endpoint<any, any, any, any, any, any>>>>>;

export type ErrorPayload = {error_payload_src};

export const Api = {{"#));
    for (path, methods_map) in &self.map {
      let quoted_path = serde_json::to_string(&json!(path)).unwrap();
      def.push_str(&format!("\n  {quoted_path}: {{"));
      for (method, item) in methods_map {
        let quoted_method = serde_json::to_string(&json!(method.as_str())).unwrap();
        def.push_str(
          &format!(
            "\n    {quoted_method}: {{ method: {method}, path: {path} }} as Endpoint<{quoted_method}, {quoted_path}, {params}, {query}, {payload}, {output}>,",
            method=quoted_method,
            path=quoted_path,
            params=item.params_shape.as_ref().map(|v| v.to_typescript()).unwrap_or_else(|| String::from("Empty")),
            query=item.query_shape.as_ref().map(|v| v.to_typescript()).unwrap_or_else(|| String::from("Empty")),
            payload=item.payload_shape.as_ref().map(|v| v.to_typescript()).unwrap_or_else(|| String::from("Empty")),
            output=item.output_shape.to_typescript(),
      ))
      }
      def.push_str("\n  },");
    }
    
    def.push_str("\n} satisfies ApiDefinition;");

    def

  }

  pub fn openapi_spec(&self) -> serde_json::Value {
   
    let schemas = json!({
      "ErrorPayload": self.error_payload_schema,
    });
    
    let mut paths = json!({});

    for (path, methods_map) in &self.map {

      let pattern = regex_static::static_regex!(":([a-zA-Z0-9_]+)");
      
      let path = pattern.replace_all(path, |caps: &regex::Captures<'_>| {
        let name = caps.get(1).unwrap().as_str();
        format!("{{{name}}}")
      }).to_string();

      let mut methods = json!({});

      for (method, item) in methods_map {
        let mut endpoint = json!({});
        
        let mut parameters = vec![];
        
        if let Some(schema) = &item.params {
          let value = schema.as_value();
          if !value["properties"].is_object() {
            panic!("params schema properties must be an object");
          }

          for (name, param) in value["properties"].as_object().unwrap() {
            let param = json!({
              "in": "path",
              "name": name,
              "required": true,
              "schema": param,
            });
            
            parameters.push(param);
          }
        }

        if let Some(schema) = &item.query {
          let value = schema.as_value();
          if !value["properties"].is_object() {
            panic!("query schema properties must be an object");
          }

          for (name, param) in value["properties"].as_object().unwrap() {
            let param = json!({
              "in": "query",
              "name": name,
              "style": "deepObject", 
              "schema": param,
            });

            parameters.push(param);
          }
        }

        if !parameters.is_empty() {
          endpoint["parameters"] = json!(parameters);  
        }

        if let Some(payload) = &item.payload {
          endpoint["requestBody"] = json!({
            "content": {
              "application/json": {
                "schema": payload,
              }
            }
          })
        }

        endpoint["responses"] = json!({
          "200": {
            "description": "A successful response",
            "content": {
              "application/json": {
                "schema": item.output
              }
            }
          },

          "4XX": {
            "description": "A client error",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/ErrorPayload",
                }
              }
            }
          },

          "5XX": {
            "description": "A server error",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/ErrorPayload",
                }
              }
            }
          }
        });

        methods[method.as_str().to_ascii_lowercase()] = endpoint;
      };

      paths[path] = methods;
    }

    let openapi = json!("3.0.3");

    let info = json!({
      "title": "Some API",
      "summary": "This is the Some API summary",
      "description": "This is the Some API description",
      "termsOfService": "http://example.test/terms/",
      "contact": {
        "name": "Some API Support",
        "url": "http://example.test",
        "email": "support@example.test",
      },
        // version of the API
      "version": "0.1.0",
      "license": {
        "name": "Apache 2.0",
        "identifier": "Apache-2.0",
        // url is mutually exclusive with identifier
        // "url": "https://www.apache.org/licenses/LICENSE-2.0.html",
      },
    });

    // can interpolate variables in `servers` see https://swagger.io/specification/

    let servers = json!([
      {
        "url": "/",
        "description": "This server",
      },

      // {
      //   "url": "https://{username}.gigantic-server.com:{port}/{basePath}",
      //   "description": "The production API server",
      //   "variables": {
      //     "username": {
      //       "default": "demo",
      //       "description": "this value is assigned by the service provider, in this example `gigantic-server.com`"
      //     },
      //     "port": {
      //       "enum": [
      //         "8443",
      //         "443"
      //       ],
      //       "default": "8443"
      //     },
      //     "basePath": {
      //       "default": "v2"
      //     }
      //   }
      // }
    ]);

    json!({
      "openapi": openapi,
      "info": info,
      "servers": servers,
      "paths": paths,
      "components": {
        "schemas": schemas, 
      }
    })
  }


  pub fn axum_router(&self) -> axum::Router {
    let mut router = axum::Router::<()>::new();
    for (path, methods_map) in &self.map {
      let mut method_router = MethodRouter::<(), Infallible>::new();
      for (method, item) in methods_map {
        let method_filter = match method  {
          &Method::HEAD => MethodFilter::HEAD,
          &Method::GET => MethodFilter::GET,
          &Method::POST => MethodFilter::POST,
          &Method::PUT => MethodFilter::PUT,
          &Method::PATCH => MethodFilter::PATCH,
          &Method::DELETE => MethodFilter::DELETE,
          &Method::OPTIONS => MethodFilter::OPTIONS,
          &Method::CONNECT => MethodFilter::CONNECT,
          &Method::TRACE => MethodFilter::TRACE,
          other => panic!("unsupported method {other}"),
        };

        let handler = item.handler.clone();
        method_router = method_router.on(method_filter, move |req: Request| async move {
          handler.handle(req).await
        });
      }
      router = router.route(path, method_router);
    }
    router
  }
}