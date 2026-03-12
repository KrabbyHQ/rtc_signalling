# Krabby RTC Signalling Server

[![CI](https://github.com/KrabbyHQ/rtc_signalling/actions/workflows/ci.yml/badge.svg)](https://github.com/KrabbyHQ/rtc_signalling/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

Core RTC signalling server for Krabby audio/video call implementations.

## Features

- WebSocket-based signalling for WebRTC peer-to-peer connections.
- Support for ICE candidate exchange, SDDs (Offer/Answer), and call negotiation.
- Room-based signaling logic.
- JWT-based authentication for signaling sessions.
- Multi-layer configuration system.

## Setup and Execution

### 1. Core Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable).
- [Node.js](https://nodejs.org/en/download/) and [Bun](https://bun.sh/) for contribution standards tooling.

### 2. Install Dependencies

```shell
git clone https://github.com/KrabbyHQ/rtc_signalling.git
cd rtc_signalling
cargo build
```

### 3. Running the Server

*Ensure to have installed `cargo-watch`.*

```shell
cargo install cargo-watch
```

To start the server in development mode (auto-reload enabled):

```shell
cargo watch -x run
```

### 4. Contribution Standards Tooling

This repository uses Husky and Commitlint (via Bun) to enforce commit conventions.

1. Install dependencies:

```shell
bun install
```

## Testing

The project maintains reliability through comprehensive testing.

### 1. Unit Tests

Located within the source files.

**Run unit tests:**

```shell
cargo test --lib
```

### 2. Integration Tests (Roadmap)

All integration tests are expected to verify end-to-end RTC signalling flows, including socket connections, room management, and signal relaying.

### 3. How to add new tests

- **Unit Tests**: Add a `#[cfg(test)]` block at the end of your module. Followed by the respective tests as intended.

E.g.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert!(true);
    }
}
```

## Continuous Integration (CI)

This project has a CI setup configured via GitHub Actions. If you fork the repository and want to verify the CI builds on your fork, you may need to add repository secrets to your fork's settings for any environment-specific values required by the tests.

### How to Add Secrets to Your GitHub Fork:

1.  Navigate to your fork of the repository on GitHub.
2.  Click on the **Settings** tab at the top.
3.  In the left sidebar, click on **Secrets and variables** and then select **Actions**.
4.  Click the **New repository secret** button.
5.  Enter the secret **Name** and its corresponding **Value**.
6.  Click **Add secret**.
7.  Repeat this process for all required secrets.

## Contributing

Contributions are what make the open-source community such an incredible place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

Please check our [Contributing Guidelines](CONTRIBUTING.md) to get started.

### Code of Conduct

Please review [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

Please report vulnerabilities according to [SECURITY.md](SECURITY.md).

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
