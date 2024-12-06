use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_6::{count_obstructions, parse_grid};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string("input.txt").unwrap();
    let grid = parse_grid(&content).unwrap();
    c.bench_function("count_obstructions", |b| {
        b.iter(|| count_obstructions(black_box(&grid)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
