# Contributing to Zeta

Thank you for your interest in contributing to Zeta! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/zeta.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo
- An Anthropic API key (for testing voice features)

### Running Locally

For web development:
```bash
dx serve --features web
```

For desktop development:
```bash
cargo run --features desktop
```

## Code Style

- Follow Rust's official style guidelines
- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings

## Testing

Before submitting a PR:

1. Ensure the project builds: `cargo build --features web && cargo build --features desktop`
2. Test both web and desktop features
3. Verify the UI works as expected
4. Check that voice recording (in web mode) functions properly

## Pull Request Guidelines

- Keep PRs focused on a single feature or bug fix
- Write clear commit messages
- Update documentation if needed
- Add tests if applicable
- Ensure CI passes

## Feature Requests

Feature requests are welcome! Please:

1. Check if the feature has already been requested
2. Clearly describe the feature and its use case
3. Explain why it would be useful to other users

## Bug Reports

When reporting bugs, please include:

1. A clear description of the issue
2. Steps to reproduce
3. Expected behavior
4. Actual behavior
5. System information (OS, browser, Rust version)
6. Screenshots if applicable

## Questions?

Feel free to open an issue for questions about contributing.

Thank you for helping make Zeta better!
