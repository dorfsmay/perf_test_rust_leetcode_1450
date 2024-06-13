# Testing performance of different solutions in Rust for leetcode #1450

Or how not to benchmark.

## Branches

- main: using Vec<i32>, black\_box  and cargo bench
- using-slices: using slices (not working on leetcode), black_box and bench
- benchmarking\_with\_a\_simple\_loop: using blackbox and a home brewed crude attempt at benchmarking

## Origin

This version is now using Vec<i32> as required by the leetcode exercise, as a result:

  - the fastest test is 4 times slower than when using slices (43 ns instead of 9 ns)
  - the difference between the fastest and the simple loop is a factor of 1.5 instead of 4

I suspect the cloing on every iteration of the benchmark loop is what is making it slower and masking the differences between the functions.

Take a look at the branch "using-slices" for the version with slices where the simple loop with square indexing is twice as slow as using zip + filter.

Problem used: [leetcode # 1450](https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/)

Thanks to the folks at [Calgary Rust](http://calgaryrust.ca/) for suggesting using bench and black\_box.


## Usage
- `cargo test`
- `cargo bench`

