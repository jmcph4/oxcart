use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut vector: Vec<u64> = Vec::new();
    c.bench_function("vector_insert_front 1000000",
                     |b| b.iter(|| vector.insert(0, black_box(1000000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

