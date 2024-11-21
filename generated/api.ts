/// this file is auto generated by its Rust definition, do not edit manually

export type Empty = Record<string, never>;

export type Endpoint<
  Method extends string,
  Path extends string,
  Params,
  Query,
  Payload,
  Output
> = {
  method: Method,
  path: Path,
  // this $ types are never constructed, only used as a template
  $params?: Params,
  $query?: Query,
  $payload?: Payload
  $output?: Output
};

export type ErrorPayload = { error: { status: number, message: string, } & ({ "kind": "INTERNAL" } | { "kind": "RESOURCE_NOT_FOUND" } | { "kind": "RECORD_NOT_FOUND" } | { "kind": "INVALID_PARAMS_PARSE" } | { "kind": "INVALID_PARAMS_VALIDATE" } | { "kind": "INVALID_QUERY_PARSE" } | { "kind": "INVALID_QUERY_VALIDATE" } | { "kind": "INVALID_PAYLOAD_PARSE" } | { "kind": "INVALID_PAYLOAD_VALIDATE" }), }

export type Api = {
  "/users": {
    "GET": Endpoint<"GET", "/users", Empty, { skip: bigint | null, limit: bigint | null, }, Empty, { skip: bigint, limit: bigint, total: bigint, items: Array<{ 
/**
 * The unique id of the user
 */
id: string, 
/**
 * The email address of the user
 */
email: string, }>, }>
  }
  "/users/:id": {
    "GET": Endpoint<"GET", "/users/:id", { id: string, }, Empty, Empty, { 
/**
 * The unique id of the user
 */
id: string, 
/**
 * The email address of the user
 */
email: string, }>
  }
}