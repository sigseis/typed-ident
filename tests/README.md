# Tests

This project has a few different kinds of tests that you can run:

* **Bench Tests** - testing the performance of the crate ([see the benches documentation about this](../benches/README.md)).
* **Doc Tests** - quick demonstration for how to use functions and types (`cargo test --all-features --doc`).
* **Integration Tests** - testing a lot of different things at once over a number of inputs (`cargo test --all-features --test '*'`).
* **Unit Tests** - testing specific interactions for a specific implementation for a single type (`cargo test --all-features --lib`).

This isn't anything surprising, it's just supposed to set the stage for our test strategy.

If you're a contributor, see the [CONTRIBUTING](../CONTRIBUTING.md) guide for some helpful commands.

## `DEBUG_TYPED_IDENT`

For integration tests specifically, if you set the `DEBUG_TYPED_IDENT` environment variable, you can see detailed logs about what the test is going (and then the test will explicitly fail itself so that you can see those logs).

This environment variable can either be set blank, or if it's set with a value, it will be used as a "contains" operation on the identifier under test - effectively limiting your logged output to the identifiers that you care about logging. This can be useful if you want to debug a certain identifier more closely.

## [`test_data`](./validation/test_data.rs)

This is a file that contains all of the possible identifiers under test.

If you would like to add an identifier to the integration test add it here, and fill out the necessary information. Don't worry about filling it out wrong, if you do it will be caught by the test framework. So just start by naming your identifier, filling in the information as best as possible, and then running the integration tests.

It will fail if you either filled it out wrong (though it will look like a normal integration test failure). But it will contain details for what failed and why - allowing you to debug the failure and make the necessary changes.
