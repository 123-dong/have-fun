Gateway (minimal)

Purpose: HTTP API facade that forwards requests to service(s) via gRPC.

Suggested minimal layout inside `gateway/src/`

- `main.rs` - bootstraps HTTP server
- `routes.rs` - route definitions
- `handlers.rs` - request -> call grpc client (use case adapter)
- `grpc_client.rs`- thin gRPC client wrappers to call services
- `middleware/` - error mapping, auth, logging

Keep business logic inside service crates, not here.
