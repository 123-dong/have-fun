Shared crate

Purpose: types and helpers shared across crates: domain models, common errors, database helper and config loaders.

Suggested modules (`shared/src/`)

- `models.rs` - domain entities / value objects
- `errors.rs` - AppError enum and conversions
- `database.rs` - connection pool helpers
- `config.rs` - environment/config parsing

Keep `shared` small and dependency-free where possible.
