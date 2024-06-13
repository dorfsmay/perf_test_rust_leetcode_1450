use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate perf_test_rust_leetcode_1450;
use perf_test_rust_leetcode_1450::tests::slots;
use perf_test_rust_leetcode_1450::{
    busy_student_1, busy_student_2, busy_student_3, busy_student_4,
};

fn benchmark_1(c: &mut Criterion) {
    let (s, e, q, _) = slots("eleven");
    c.bench_function("busy_student_1 iter zip (iter) filter count", |b| {
        b.iter(|| busy_student_1(black_box(&s), black_box(&e), black_box(q)))
    });
}

fn benchmark_2(c: &mut Criterion) {
    let (s, e, q, _) = slots("eleven");
    c.bench_function("busy_student_2 iter copied zip copied filter count", |b| {
        b.iter(|| busy_student_2(black_box(&s), black_box(&e), black_box(q)))
    });
}

fn benchmark_3(c: &mut Criterion) {
    let (s, e, q, _) = slots("eleven");
    c.bench_function("busy_student_3 simple loop with square indexing", |b| {
        b.iter(|| busy_student_3(black_box(&s), black_box(&e), black_box(q)))
    });
}

fn benchmark_4(c: &mut Criterion) {
    let (s, e, q, _) = slots("eleven");
    c.bench_function("busy_student_4 simple loop over zipped", |b| {
        b.iter(|| busy_student_4(black_box(&s), black_box(&e), black_box(q)))
    });
}

criterion_group!(benches, benchmark_1, benchmark_2, benchmark_3, benchmark_4);
criterion_main!(benches);
