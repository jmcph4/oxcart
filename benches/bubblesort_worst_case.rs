use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxcart::list::List;
use oxcart::arraylist::ArrayList;
use oxcart::bubblesort::bubblesort;

fn cmp_leq<T: Eq + Ord>(a: &T, b: &T) -> bool {
    a <= b
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut list: ArrayList<u64> = ArrayList::new();

    /* need to reverse to ensure not already in ascending order */
    for i in (0..1000).rev() {
        list.append(i);
    }

    c.bench_function("bubblesort_worst_case 1000",
                     |b| b.iter(|| bubblesort(&mut list, cmp_leq)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

