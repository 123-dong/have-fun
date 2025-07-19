# 📄 Microservice Project — README

## ✅ Overview

Microservice system with these core services:

| Service              | Purpose                      | Language | DB                 |
| -------------------- | ---------------------------- | -------- | ------------------ |
| User Service         | CRUD user/profile            | Go       | PostgreSQL         |
| Auth Service         | Auth/JWT/OAuth2              | Rust     | PostgreSQL         |
| Product Service      | CRUD product/catalog         | Rust     | PostgreSQL         |
| Order Service        | Order management             | Go       | PostgreSQL         |
| Cart Service         | Shopping cart session        | Go       | Redis + PostgreSQL |
| Payment Service      | Payment logic, transaction   | Rust     | PostgreSQL         |
| Notification Service | Email/SMS/push notifications | Go       | MongoDB            |
| Analytics Service    | Event log, tracking          | Rust     | MongoDB            |
| API Gateway          | Entry point, proxy, routing  | Rust     | -                  |

---

## ✅ Stack

- **Languages**: Rust (Axum + Tokio), Go (Fiber or Gin)
- **Databases**:

  - **PostgreSQL**: main relational DB
  - **MongoDB**: flexible log store
  - **Redis**: fast cache/session store

---

## ✅ Key Design

- Stateless services → easy scaling.
- Async everywhere (Rust: Tokio; Go: goroutines).
- Connection pool for DB (`sqlx::PgPool`, `gorm` pool).
- Redis for caching session/cart.
- API Gateway handles routing, rate limiting, JWT check.
- Use gRPC/HTTP REST for service-to-service comms.

---

## ✅ Example Request Flow

```
Client -> Gateway (Rust)
   |
   ├─> Auth Service (Rust): login, JWT
   |
   ├─> User Service (Go): profile CRUD
   |
   ├─> Product Service (Rust): list/filter
   |
   ├─> Order Service (Go): create order
   |
   ├─> Payment Service (Rust): confirm transaction
   |
   ├─> Notification Service (Go): send email
   |
   └─> Analytics Service (Rust): log event
```

---

## ✅ Redis

- In-memory key-value store.
- Used for:

  - Session store (cart, auth sessions)
  - Caching heavy queries
  - Rate limiting
  - Pub/Sub

Redis = fast, stores data in RAM. Main DB (PostgreSQL) holds source-of-truth.

---

## ✅ Must Have

- `.env` for config (DB_URL, JWT_SECRET, REDIS_URL).
- `proto/` or OpenAPI for clear contracts.
- CI/CD to build multi-binary.
- Tracing: `tracing` crate for Rust, Prometheus for metrics.

---

## ✅ Principle

- Rust → critical logic, high concurrency.
- Go → CRUD & background tasks.
- PostgreSQL → relational core.
- MongoDB → log & event store.
- Redis → fast cache & session.

Keep it stateless. Test concurrency. Scale horizontally.

---

## ✅ Next

- Setup workspace: `common/` lib for shared models.
- Use `sqlx` for PostgreSQL, `deadpool-redis` for Redis.
- Use `jsonwebtoken` for JWT.
- Write async-first handlers.

---

**That’s it — fast, safe, scalable! 🚀**
