# Contributing to `typed-ident`

This project welcomes contributions from everyone in the form of suggestions, bug reports, pull requests, and feedback. However, we have some requests for how each is done.

## Submitting Bug Reports & Feature Requests

When submitting a bug report or feature request, please use the GitHub Issues board.

Start with a template of the following format:

```markdown
# Summary

(A brief 1~2 line summary of what is being requested.)

# Details

(Additional detail to clarify the request or provide additional context.)
```

You may add other sections as you see fit.

If the issue is a `bug`, please provide as much detail for your issue as possible, specifically include what you expected, what happened, and why that was counter to your expectation.

If possible, please also provide a minimal sample reproduction of the bug. Ideally your code sample would be in the form of a small codeblock, as if it would be a unit test that would not pass currently (does not have to compile, just a rough "a bug of this form" kind of thing). For some tips on how to approach this, read about how to produce a [Minimal, Complete, and Verifiable example](https://stackoverflow.com/help/minimal-reproducible-example).

If the issue is a `feature`, and if it is not obvious, provide some justification for why this feature should be added. For example, why was it a burden that this feature was not implemented within this project? Why is this problem general enough to be here and not in a user's crate? etc.

If the issue is `documentation`, pointing to the file in question, and the relevant information to locate the issue would be nice. But if it's just a general issue with documentation, or if it impacts many files, this is not required.

## Submitting a Pull Request

Before working on a pull request, please read this:

* **No pull request will even be considered if it didn't originate from a triaged issue!**
* **If a work item is already assigned that means someone is already working on it - we will not accept other's work!**

This is to avoid having others do a bunch of work all for us to say "no, thank you".

### Testing Your Change

While working on your change, you can test it along the way with:

```shell
cargo test --all-features
```

This is fine during active development, but when you think you're ready to create a pull request, you should run a more comprehensive command.

First, you will need to install dependencies (this only needs done once per system):

```shell
cargo install cargo-docs-rs
cargo install cargo-hack
rustup +nightly component add miri
```

Then, you can run this command (this may take some time):

```shell
cargo fmt &&
    cargo clippy --all-targets --all-features -- -D warnings &&
    cargo hack check --feature-powerset &&
    cargo hack build --feature-powerset &&
    cargo hack test --feature-powerset --lib &&
    cargo test --all-targets --all-features &&
    cargo +nightly miri test --all-features --lib &&
    cargo +nightly miri test --all-features --doc &&
    # We exclude integration tests because they take forever to run under Miri.
    cargo +nightly docs-rs
```

If you have made a foundational change, you might want to run A/B comparisons with the [benchmark tests](./benches/README.md).

### Testing Coverage

We like to use [`tarpaulin`](https://github.com/xd009642/tarpaulin) for code coverage.

You should *NOT* code coverage the benchmarks - those are for testing the efficiency and comparing with other implementations or baselines. But you *SHOULD* code coverage all of the other tests (doc, integration, and unit). In order to do that, you *have* to run with `--all-features` enabled.

*(Note: You should set the `--output-dir` so that it goes into an ignored directory.)*

You can install tarpaulin via:

```bash
cargo install tarpaulin
```

Then you can run the coverage testing via:

```bash
cargo tarpaulin \
    --all-features --doc --tests \
    --out html \
    --output-dir target \
    --include-files 'typed-ident/src/*'
```

After running your tests, you can find the HTML output at:

* `${TARGET}/tarpaulin-report.html`
