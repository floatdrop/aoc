use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day8;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day8::part1", |b| b.iter(|| day8::part1()));
    c.bench_function("day8::part2", |b| b.iter(|| day8::part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
