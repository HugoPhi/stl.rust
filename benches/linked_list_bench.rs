use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use hym::LinkedList;

fn bench_push_head(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList Operations");
    for size in [1000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::new("push_head", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_head(&black_box(i));
                }
            })
        });
    }
    group.finish();
}

fn bench_push_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList Operations");
    for size in [1000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::new("push_back", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_back(&black_box(i));
                }
            })
        });
    }
    group.finish();
}

fn bench_pop_head(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList Operations");
    for size in [1000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::new("pop_head", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_head(&black_box(i));
                }
                for _ in 0..size {
                    list.pop_head().unwrap();
                }
            })
        });
    }
    group.finish();
}

fn bench_pop_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList Operations");
    for size in [1000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::new("pop_back", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_back(&black_box(i));
                }
                for _ in 0..size {
                    list.pop_back().unwrap();
                }
            })
        });
    }
    group.finish();
}

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList Operations");
    for size in [1000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::new("insert_middle", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_back(&black_box(i));
                }
                list.insert(&black_box(u64::MAX), size as usize / 2)
                    .unwrap();
            })
        });
    }
    group.finish();
}

fn bench_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList Operations");
    for size in [1000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::new("remove_middle", size), size, |b, &size| {
            b.iter(|| {
                let mut list = LinkedList::new();
                for i in 0..size {
                    list.push_back(&black_box(i));
                }
                list.remove(size as usize / 2).unwrap();
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_push_head,
    bench_push_back,
    bench_pop_head,
    bench_pop_back,
    bench_insert,
    bench_remove
);
criterion_main!(benches);
