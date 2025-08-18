Minimal architecture and mapping for this repo

Goal: provide a minimal, testable 3-layer layout (Transport / UseCase / Infra) and map it to existing crates.

Tree (minimal)

/

- Cargo.toml (workspace)
- proto/ # protobuf definitions + generated code
- shared/ # domain models, errors, db helpers
- services/user/ # user service (business + persistence)
- gateway/ # HTTP gateway (transport)

Mapping to existing files (quick)

- proto: `proto/user/v1/user.proto`, `proto/generated` (generated Rust)
- shared: `shared/src/models.rs`, `shared/src/errors.rs`, `shared/src/database.rs`
- services/user: `services/user/src/main.rs`, `services/user/src/repository.rs`, `services/user/src/service.rs`
- gateway: `gateway/src/main.rs`, `gateway/src/routes.rs`, `gateway/src/handlers.rs`, `gateway/src/grpc_client.rs`

Recommendation

- Keep `shared` small and only truly shared types.
- Implement repository trait + concrete impl separation in `services/user`.
- Gateway must be thin: mapping HTTP -> gRPC client (no business logic).

Next steps

- If you want code skeletons (traits, adapters, simple impls), reply and I will create them in `services/user` and `gateway`.
