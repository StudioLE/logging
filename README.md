# studiole-logging

Opinionated logging built on `tracing` and `tracing-subscriber`.

## Highlights

- Builder pattern for configuring log level and per-target overrides
- Compact stderr output with elapsed time
- `studiole-di` integration via `WithLogging` trait

## Structure

Cargo workspace with single crate:

- `crates/logging/src/configuration/` - `LogLevel`, `TargetFilter`, `ElapsedTime`
- `crates/logging/src/construction/` - `Logger`, `LoggerBuilder`
- `crates/logging/src/registration/` - `LoggerFactory`, `WithLogging` trait for DI

## CI/CD

Reusable workflow: `StudioLE/Actions/.github/workflows/ci-cd-rust.yml@v7`
