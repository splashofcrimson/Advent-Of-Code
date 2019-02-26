extern crate Advent_lib;

#[macro_use]
extern crate criterion;

use criterion::Criterion;

use Advent_lib::*;

fn crit_benchmark(c: &mut Criterion) {
    // c.bench_function("day 01.1", |b| b.iter(|| day01::one()));
    // c.bench_function("day 01.2", |b| b.iter(|| day01::two()));
    // c.bench_function("day 02.1", |b| b.iter(|| day02::three()));
    // c.bench_function("day 02.2", |b| b.iter(|| day02::four()));
    // c.bench_function("day 03.1", |b| b.iter(|| day03::five()));
    c.bench_function("day 03.2", |b| b.iter(|| day03::six()));
}

criterion_group!(benches, crit_benchmark);
criterion_main!(benches);