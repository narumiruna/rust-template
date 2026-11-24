# rust-template

The root [`Cargo.toml`](Cargo.toml) defines a Cargo workspace with one member, [`crates/cli`](crates/cli/). The CLI package has its own [`Cargo.toml`](crates/cli/Cargo.toml), source in [`src/`](crates/cli/src/), and integration tests in [`tests/`](crates/cli/tests/).

From the repository root, use `cargo build`, `cargo run`, or `cargo test`.

Keep [`LICENSE`](LICENSE) and [`crates/cli/LICENSE`](crates/cli/LICENSE) in sync. The CLI package includes its own copy when built with `cargo package`.

GitHub Actions checks formatting, runs Clippy, and runs tests on Linux for pull requests and pushes to `main`.

[Changenote](https://github.com/changenote/changenote) uses [`.changenote/`](.changenote/) to manage a release pull request. On pushes to `main`, it also publishes tags and GitHub Releases for verified releases. Manual runs only manage the release pull request. In the repository's Actions settings, allow GitHub Actions to create pull requests. No custom secrets or registry uploads are needed.
