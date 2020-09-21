# Contributing

Thanks for considering contributing to mmio! Before making a non-trivial change, please open an issue for discussion.

## Issues

Issues should be added to the [issue tracker](https://github.com/akiekintveld/mmio/issues). Please describe your problem clearly and make sure it is not covered by an existing issue.

## Pull Requests

Try to make one pull request per change.

### Updating the Changelog

Update the [CHANGELOG](https://github.com/akiekintveld/mmio/blob/master/CHANGELOG.md) under the **Unreleased** section to describe the changes you made.

Changes should be described under one of the following subsections:

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet, create it!

## Development

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/akiekintveld/mmio
cd mmio
cargo build
```

### Useful Commands

- Run Clippy:

  ```shell
  cargo clippy --all
  ```

- Run all tests:

  ```shell
  cargo test --all
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
