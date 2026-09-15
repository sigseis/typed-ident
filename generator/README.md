# Generator

This is a private, non-exported binary that updates generated data in the repo.

## How To Use

It's expected that you run this from the repo root. In order to do this you need to provide the `--manifest-path` flag to point to this project.

## TL;DR

To update to the latest Unicode version, run:

```shell
cargo install ucd-generate
cargo run --manifest-path=generator/Cargo.toml update-tables
cargo run --manifest-path=generator/Cargo.toml generate
```

## Updating the Unicode Version

In order to update the version of Unicode targeted by this library, you need to have `ucd-generate` installed. You can install it via:

```shell
cargo install ucd-generate
```

After that, you can update this binary to the latest published Unicode version by running:

```shell
cargo run --manifest-path=generator/Cargo.toml update-tables
```

If you want to list known versions based on web scraping, you can run:

```shell
cargo run --manifest-path=generator/Cargo.toml list-versions
```

And if you want to update to a specific version, you can use the `--unicode-version` flag (this requires a full version string; `{major}.{minor}.{patch}`).

```shell
cargo run --manifest-path=generator/Cargo.toml update-tables --unicode-version 17.0.0
```

## Updating the Generated Source

After updating the generator, you can then build and run again with a different command to update the source that gets embedded into the main library.

```shell
cargo run --manifest-path=generator/Cargo.toml generate
```

That's all there is to it! Be sure to run unit tests to make sure nothing broke.
