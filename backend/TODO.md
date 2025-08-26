src/
├─ main.rs # Entrypoint, init server, spawn hot swap background
├─ state.rs # SharedState: versioned/granular client management, sharded locks
├─ grpc_clients/ # gRPC client wrappers per service
│ ├─ mod.rs # Re-export & generic client trait
│ ├─ user.rs
│ └─ product.rs
├─ handlers/ # Generic handler + per-service handler
│ ├─ mod.rs # register handlers, helper traits
│ ├─ generic.rs # Generic handler for repeated call/conversion logic
│ ├─ user.rs
│ └─ product.rs
├─ routes/ # Router mounting helpers
│ ├─ mod.rs # merge all routes
│ └─ service_router.rs # macro/helper to mount service router
├─ dto/ # Adapter layer: proto → internal DTO → JSON
│ ├─ mod.rs
│ ├─ generic.rs # shared conversion utils
│ ├─ user.rs
│ └─ product.rs
├─ proto/ # Generated protobuf
└─ utils/ # Logging, metrics, error handling, version tracing
├─ mod.rs
├─ logging.rs
├─ metrics.rs
└─ error.rs
