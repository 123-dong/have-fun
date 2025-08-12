```
backend/
├── Cargo.toml                      # Cargo workspace
├── proto
│   ├── Cargo.toml
│   ├── build.rs
│   ├── generated
│   │   ├── all_descriptor.bin
│   │   └── user.v1.rs
│   ├── src
│   │   └── lib.rs
│   └── user
│       └── v1
│           └── user.proto
├── shared/                           # Shared lib & shared utils
│   └── src/
│       ├── lib.rs
│       ├── models.rs
│       ├── errors.rs
│       ├── config.rs
│       ├── database.rs
│       ├── utils.rs
│       ├── constants.rs
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
