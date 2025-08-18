# User Service (Minimal)

A minimal Rust/Tonic service implementing **business logic and persistence** for the "user" bounded context.

---

## Project Layout

```
services/user/src/
├── main.rs # Bootstraps gRPC server, logging, reflection
├── service.rs # Business logic (UserSvc), injects repository trait
├── repository.rs # Repository trait interface (UserRepoTrait)
├── handler.rs # gRPC handler (UserHdl), maps requests → service → responses
├── models.rs # Mapping between proto messages ↔ domain models
└── ../test_user.sh # Unit + integration tests
```

**Notes:**

- Keep domain models in `shared` crate for reuse across services.
- Use proto messages for transport (gRPC).

---

## Layers Overview

1. **Repository Layer**

   - Handles persistence.
   - Defines trait interface only (`UserRepoTrait`).
   - Concrete DB implementations (Postgres, SQLite) live outside handler.

2. **Service Layer**

   - Implements business logic (`UserSvc`).
   - Pure functions operating on domain models.
   - Uses repository trait for persistence.
   - No knowledge of transport (gRPC/HTTP).

3. **Handler Layer**
   - Implements gRPC service (`UserHdl`).
   - Converts gRPC requests → service calls → gRPC responses.
   - Minimal logic; just orchestrates transport.

**Flow:**

```
Client → Handler → Service → Repository → Database
```

---

## Getting Started

### Prerequisites

- Rust 1.7+ (or latest stable)
- PostgreSQL
- `grpcurl` for testing

### Build & Run

```bash
cargo run --bin user
```
