# 🗂 Microservice App

## Overview

A full-stack microservice system with the following core services:

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

## Tech Stack

- **Backend:** Rust (Axum, Tonic, Tokio, sqlx), Go (Fiber/Gin, GORM)
- **Frontend:** React, TypeScript, Vite
- **Databases:** PostgreSQL, MongoDB, Redis
- **Communication:** gRPC, REST
- **Deployment:** Docker Compose

## Setup

### Prerequisites

- Docker & Docker Compose
- Node.js & npm (for frontend)
- Rust & Go toolchains (for local dev)

### Running Locally

1. Clone the repo:
   ```sh
   git clone https://github.com/123-dong/have-fun.git
   cd have-fun
   ```
