use criterion::{criterion_group, criterion_main, Criterion};

use simple_sudoku_generator::bench_main;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sudoku main", |b| b.iter(|| bench_main()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);