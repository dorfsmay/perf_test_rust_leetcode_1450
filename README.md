# Testing performance of different solutions in Rust for leetcode #1450

Unless there is something wrong with my testing approach, a simple test on an iteration using square indexing is 5 times faster zip + filter.

To test the simple iteration: `cargo run verify`

To compare perfs: `cargo run perf`
