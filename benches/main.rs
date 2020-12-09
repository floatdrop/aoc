use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day9;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day9::part1", |b| b.iter(|| day9::part1()));
    c.bench_function("day9::part2", |b| b.iter(|| day9::part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
