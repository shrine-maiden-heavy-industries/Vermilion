# Vermilion [afl.rs] Fuzzing Harnesses

## tl;dr

* Install `cargo-afl`
* Put initial corpus data into `corpus`
* `cargo afl build --bin fuzz_<target_name>`
* `cargo afl fuzz -i ./fuzzing/corpus -o ./fuzzing/artifacts ./target/debug/fuzz_<target_name>`

[afl.rs]: https://github.com/rust-fuzz/afl.rs
