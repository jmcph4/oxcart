use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxcart::list::List;
use oxcart::arraylist::ArrayList;

fn criterion_benchmark(c: &mut Criterion) {
    let mut list: ArrayList<u64> = ArrayList::new();
    c.bench_function("arraylist_insert_head 1000000",
                     |b| b.iter(|| list.insert(0, black_box(1000000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

