# rust-cli-template

GitHub Actions checks formatting, runs Clippy, and runs tests on Linux for pull requests and pushes to `main`.

[Changenote](https://github.com/changenote/changenote) uses [`.changenote/`](.changenote/) to manage a release pull request. On pushes to `main`, it also publishes tags and GitHub Releases for verified releases. Manual runs only manage the release pull request. In the repository's Actions settings, allow GitHub Actions to create pull requests. No custom secrets or registry uploads are needed.
