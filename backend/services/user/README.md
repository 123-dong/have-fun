User service (minimal)

Purpose: implement business logic and persistence for "user" bounded context.

Suggested minimal layout inside `services/user/src/`

- `main.rs` - bootstraps gRPC server
- `service.rs` / `usecase.rs` - business logic (pure functions, inject repository trait)
- `repository.rs` - repository trait (interface)
- `repository_impl.rs` - concrete DB implementation
- `adapters.rs` - proto <-> domain mapping
- `tests/` - unit + integration tests

Keep domain models in `shared` and reuse proto messages for transport.
