```
backend/
├── Cargo.toml                      # Cargo workspace
├── proto/                          # Proto definitions
│   ├── user/
│   │   ├── v1/
│   │   └── user.proto
│   ├── auth/
│   └── product/
├── core/                           # Core lib & shared utils
│   └── src/
│       ├── lib.rs
│       ├── config.rs
│       ├── constants.rs
│       ├── database.rs
│       ├── errors.rs
│       ├── models.rs
│       ├── utils.rs
│       └── macros.rs
├── services/                       # Each service as independent crate
│   ├── user/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── service_impl.rs
│   │       └── logic.rs
│   ├── auth/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── service_impl.rs
│   │       └── logic.rs
│   └── product/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── service_impl.rs
│           └── logic.rs
├── gateway/                        # API Gateway
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── grpc_client.rs
│       ├── handlers.rs
│       ├── middleware.rs
│       └── routes.rs
```
