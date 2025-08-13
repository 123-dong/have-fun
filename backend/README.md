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
│       ├── database.rs
│       ├── errors.rs
│       ├── models.rs
│       ├── config.rs
│       └── macros.rs
│       ├── constants.rs
│       ├── utils.rs
├── services/                       # Each service as independent crate
│   ├── user/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── service_impl.rs
│   │       └── repository.rs
│   ├── auth/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── service_impl.rs
│   │       └── repository.rs
│   └── product/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── service_impl.rs
│           └── repository.rs
├── gateway/                        # API Gateway
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── client.rs
│       ├── handlers.rs
│       ├── middleware.rs
│       └── routes.rs
```

--name rust_db \
 -e POSTGRES_USER=admin \
 -e POSTGRES_PASSWORD=123 \
 -e POSTGRES_DB=demo_db \
 -p 5432:5432 \
 postgres:16-alpine
