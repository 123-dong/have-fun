```
project-root/
├── Cargo.toml                  # workspace
├── proto/                      # .proto files
│   ├── user/v1/user.proto
│   └── build.rs                # tonic-build
│
├── shared/                     # common crate (DB, models, utils, config)
│   ├── Cargo.toml
│   └── src/
│       ├── config.rs
│       ├── database.rs
│       ├── models.rs
│       ├── utils.rs
│       └── lib.rs
│
├── user-service/               # gRPC service (tonic)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # entrypoint gRPC server
│       ├── server_impl.rs      # tonic server + handlers
│       ├── service.rs          # business logic
│       ├── repository.rs       # sqlx repo
│
│
└── gateway/                     # REST ↔ gRPC (Axum)
    ├── Cargo.toml
    └── src/
        ├── main.rs             # entrypoint REST server
        ├── router.rs           # routes
        ├── handler.rs          # REST handlers calling gRPC
        └── middleware.rs       # logging, auth, rate-limiting
```

--name rust_db \
 -e POSTGRES_USER=admin \
 -e POSTGRES_PASSWORD=123 \
 -e POSTGRES_DB=demo_db \
 -p 5432:5432 \
 postgres:16-alpine

```
Frontend (REST/JSON)
    ↓
REST Gateway (REST ↔ gRPC)
    ↓
gRPC Service (tonic)
    ↓
Repository (sqlx)
    ↓
Database (Postgres)
```

```
Browser ──HTTP──▶ Gateway (Axum)
                         │
                         ▼
                 gRPC Client (tonic)
                         │
                         ▼
                 User Service (gRPC Server)
                         │
                         ▼
                      Database
```
