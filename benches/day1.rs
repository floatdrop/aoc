use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day1;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1::part1", |b| b.iter(|| day1::part1()));
    c.bench_function("day1::part1_opt", |b| b.iter(|| day1::part1_opt()));
    c.bench_function("day1::part2", |b| b.iter(|| day1::part2()));
    c.bench_function("day1::part2_opt", |b| b.iter(|| day1::part2_opt()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

