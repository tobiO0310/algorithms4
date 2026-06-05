use std::hint::black_box;

use algorithms4::{
    HeapSort, MergeSort, QuickSort, binary_insertion_sort, insertion_sort,
    insertion_sort_x, selection_sort, shell_sort,
};
use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};
use rand::RngExt;

fn exp_range() -> Vec<usize> {
    (5..11).map(|x| 2usize.pow(x)).collect::<Vec<_>>()
}

fn bench_elementary(c: &mut Criterion) {
    let mut rand = rand::rng();
    let mut group = c.benchmark_group("sorting_elementary");
    for &i in exp_range().iter() {
        let mut vec = vec![0; i];
        let m = i as i64;
        for item in vec.iter_mut() {
            *item = rand.random_range(-m / 2..m)
        }
        group.throughput(Throughput::Elements(i as u64));
        group.bench_with_input(
            BenchmarkId::new("insertion_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(insertion_sort(black_box(&mut vec.clone())))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("insertion_sort_x", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(insertion_sort_x(black_box(&mut vec.clone())))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("binary_insertion_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(binary_insertion_sort(black_box(
                        &mut vec.clone(),
                    )))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("selection_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(selection_sort(black_box(&mut vec.clone())))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("shell_sort", i),
            &i,
            |b, _| {
                b.iter(|| black_box(shell_sort(black_box(&mut vec.clone()))))
            },
        );
    }
    group.finish();
}

fn bench_merge(c: &mut Criterion) {
    let mut rand = rand::rng();
    let mut group = c.benchmark_group("sorting_merge");
    for &i in exp_range().iter() {
        let mut vec = vec![0; i];
        let m = i as i64;
        for item in vec.iter_mut() {
            *item = rand.random_range(-m / 2..m)
        }
        group.throughput(Throughput::Elements(i as u64));
        group.bench_with_input(
            BenchmarkId::new("merge_top_down_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(MergeSort::top_down_sort(black_box(
                        &mut vec.clone(),
                    )))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("merge_bottom_up_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(MergeSort::bottom_up_sort(black_box(
                        &mut vec.clone(),
                    )))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("merge_index_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(MergeSort::index_sort(black_box(
                        &mut vec.clone(),
                    )))
                })
            },
        );
    }
    group.finish();
}

fn bench_quick(c: &mut Criterion) {
    let mut rand = rand::rng();
    let mut group = c.benchmark_group("sorting_quick");
    for &i in exp_range().iter() {
        let mut vec = vec![0; i];
        let m = i as i64;
        for item in vec.iter_mut() {
            *item = rand.random_range(-m / 2..m)
        }
        group.throughput(Throughput::Elements(i as u64));
        group.bench_with_input(
            BenchmarkId::new("quick_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(QuickSort::sort(black_box(&mut vec.clone())))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("quick_three_way_sort", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(QuickSort::three_way_sort(black_box(
                        &mut vec.clone(),
                    )))
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("quick_optimized", i),
            &i,
            |b, _| {
                b.iter(|| {
                    black_box(QuickSort::optimized(black_box(&mut vec.clone())))
                })
            },
        );
    }
    group.finish();
}

fn bench_pq(c: &mut Criterion) {
    let mut rand = rand::rng();
    let mut group = c.benchmark_group("sorting_pq");
    for &i in exp_range().iter() {
        let mut vec = vec![0; i];
        let m = i as i64;
        for item in vec.iter_mut() {
            *item = rand.random_range(-m / 2..m)
        }
        group.throughput(Throughput::Elements(i as u64));
        group.bench_with_input(BenchmarkId::new("heap_sort", i), &i, |b, _| {
            b.iter(|| black_box(HeapSort::sort(black_box(&mut vec.clone()))))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_elementary,
    bench_merge,
    bench_quick,
    bench_pq
);
criterion_main!(benches);
