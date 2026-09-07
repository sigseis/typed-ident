# Benchmarks

This benchmark suite uses [`criterion`](https://criterion-rs.github.io/) for testing.

However, it has quite a lot of benchmarks and groups that it runs, so it necessitates some documentation to help developers understand how to use these properly.

## Running Benchmarks

You should try to identify the subset of benchmarks that are important for you to run for whatever it is that you're testing. If you run more benchmarks than you need, you're probably going to be waiting a very long time for the result.

The format for the benchmark groups are very structured so that you can filter them on the command-line.

After you identify what you want to run, present it as a regex to criterion. You *have* to run with `--all-features` enabled, we do not support running only a subset based on feature selection currently.

```bash
cargo bench --all-features 'REGEX'
```

So, for example, if you want to test the `new` function for all types:

```bash
cargo bench --all-features '::new/'
```

After running your benchmarks, you can find HTML output at:

* `${TARGET}/criterion/report/index.html`

## Benchmark Group Name Format

All benchmark group names are of the following format:

* `//Input<{INPUT}>/Preset<{PRESET}>/{TYPE}::{FUNCTION}/Points<{POINTS}>/{VARIANT}`

**`INPUT`** is the kind of data that will be input to the type.

* `ascii` will have randomly-generated ASCII data that is validly the identifier type.
* `unicode` will have randomly-generated Unicode data that is validly the identifier type.

Note: Generation is a bit more complicated than this, for instance we try for some number of chunks and delimiters. You should look at [`generate.rs`](./ident/generate.rs) for details.

**`PRESET`** is the sub-configuration from the `presets` module that will be used.

* `ascii` will use the `typed_ident::presets::ascii` types.
* `strict` will use the `typed_ident::presets::strict` types.
* `unicode` will use the `typed_ident::presets::unicode` types.

**`TYPE`** is the type in question that will be tested.

* `LowerCamelIdent` for identifiers like `lowerCamel`.
* `CamelIdent` for identifiers like `lowerCamel` or `UpperCamel`.
* `UpperCamelIdent` for identifiers like `UpperCamel`.
* `LowerHybridIdent` for identifiers like `lowerHybrid` (like camel, but includes `-`).
* `HybridIdent` for identifiers like `lowerHybrid` or `UpperHybrid` (like camel, but includes `-`).
* `UpperHybridIdent` for identifiers like `UpperHybrid` (like camel, but includes `-`).
* `LowerKebabIdent` for identifiers like `lower-kebab`.
* `KebabIdent` for identifiers like `Mixed-Kebab`.
* `UpperKebabIdent` for identifiers like `UPPER-KEBAB`.
* `LowerSnakeIdent` for identifiers like `lower_snake`.
* `SnakeIdent` for identifiers like `Mixed_Snake`.
* `UpperSnakeIdent` for identifiers like `UPPER_SNAKE`.

**`FUNCTION`** is the function that is being tested (not all functions have benches).

* `new` tests construction and validation.
* `segments` tests segmentation (and filtering to words as well).
* `to_lower_camel` tests conversion to a lower camel string.
* `to_lower_kebab` tests conversion to a lower kebab string.
* `to_lower_snake` tests conversion to a lower snake string.
* `to_upper_camel` tests conversion to a upper camel string.
* `to_upper_kebab` tests conversion to a upper kebab string.
* `to_upper_snake` tests conversion to a upper snake string.

**`POINTS`** is the number of datapoints that are processed by this benchmark in one sample.

This only differs between function calls. So for instance, `new` may process more points than `segments`, but it will always process the same number of points for any test of `new` across any input/preset/type/variant. The reason for this complexity is that some functions just take longer to process than others. But we really need to have more than one data point per sample, otherwise we're very vulnerable to variability in the results (or just picking one non-representative input data point).

The important thing about this number is that you can use it to find the average time per individual call.

* `REPORTED_TIME / POINTS = REPORTED_TIME_PER_INDIVIDUAL_CALL`

For example, if `LowerHybridIdent::new` says it tested across `Points<100000>`, and on average took 20.408ms - that means on average each individual call took ~204.08ns.

**`VARIANT`** are variants of the function for comparison purposes.

You'll have to see the source code to identify what these groups mean.

These should, however, be well-documented.

See: [`benches.rs`](./ident/benches.rs)
