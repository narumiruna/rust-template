# Rust CLI template guidelines

## Commands

- From the repository root, run `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo test --locked --all-targets` to match [CI](.github/workflows/ci.yml).
- Run `prek run --all-files` to check repository hygiene and workflow security.
- Run `uv lock --check` after changing Python package metadata.

## Boundaries

- Run `changenote version` or `changenote tag`, trigger the release workflow, push tags, create GitHub Releases, or upload packages only when the user explicitly requests that action.

## Testing

- Test CLI behavior by running the built executable in integration tests under `tests/`.

## Git and commits

- Before committing a release-relevant change, add or update a note in `.changenote/` for the package in [`.changenote/config.json`](.changenote/config.json) and verify it with `changenote status`.
- Omit a change note when no release is needed and state the reason in the completion report.
