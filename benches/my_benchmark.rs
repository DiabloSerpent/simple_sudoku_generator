use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};

use simple_sudoku_generator::bench_main;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sudoku main", |b| b.iter(|| bench_main()));
}

criterion_group!{
    name    = benches;
    config  = Criterion::default()
              .sample_size(500)
              .warm_up_time(Duration::from_secs(10))
              .measurement_time(Duration::from_secs(45));
    targets = criterion_benchmark
}
criterion_main!(benches);