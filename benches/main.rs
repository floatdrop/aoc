use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day5;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day5::part1", |b| b.iter(|| day5::part1()));
    c.bench_function("day5::part2", |b| b.iter(|| day5::part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

