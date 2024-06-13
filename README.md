# Testing performance of different solutions in Rust for leetcode #1450

Or how not to benchmark.

## Origin

This started as a simple tests, the results were surprising so I ended up writing a loop to test more, then adding more and adding more. See branch "benchmarking\_with\_a\_simple\_loop".

A simple loop with square indexing was running 4 to 5 times than zip and filter. Folks at [Calgary Rust](http://calgaryrust.ca/) suggested I use `cargo bench` and the result were reversed as expected, the simple loop is now twice  as slow as the zip + filter.

Problem used: [leetcode # 1450](https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/)


## Usage
- `cargo test`
- `cargo bench`

