use std::{hint::black_box, ops::Range};

use algorithms4::union_find::{
    QuickFind, QuickUnion, QuickUnionWPC, UnionFind, WeightedQuickUnion, WeightedQuickUnionWPC,
};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use num::PrimInt;
use rand::prelude::*;

fn quick_find(n: usize) -> impl UnionFind {
    let mut uf = QuickFind::new(n);
    let mut rand = rand::rng();

    let mut vec = (1..n).collect::<Vec<_>>();
    vec.shuffle(&mut rand);
    for i in vec {
        uf.union(0, i).unwrap();
    }

    uf
}

fn quick_union(n: usize) -> impl UnionFind {
    let mut uf = QuickUnion::new(n);
    let mut rand = rand::rng();

    let mut vec = (1..n).collect::<Vec<_>>();
    vec.shuffle(&mut rand);
    for i in vec {
        uf.union(0, i).unwrap();
    }

    uf
}

fn quick_union_with_path_compression(n: usize) -> impl UnionFind {
    let mut uf = QuickUnionWPC::new(n);
    let mut rand = rand::rng();

    let mut vec = (1..n).collect::<Vec<_>>();
    vec.shuffle(&mut rand);
    for i in vec {
        uf.union(0, i).unwrap();
    }

    uf
}

fn weighted_quick_union(n: usize) -> impl UnionFind {
    let mut uf = WeightedQuickUnion::new(n);
    let mut rand = rand::rng();

    let mut vec = (1..n).collect::<Vec<_>>();
    vec.shuffle(&mut rand);
    for i in vec {
        uf.union(0, i).unwrap();
    }

    uf
}

fn weighted_quick_union_with_path_compression(n: usize) -> impl UnionFind {
    let mut uf = WeightedQuickUnionWPC::new(n);
    let mut rand = rand::rng();

    let mut vec = (1..n).collect::<Vec<_>>();
    vec.shuffle(&mut rand);
    for i in vec {
        uf.union(0, i).unwrap();
    }

    uf
}

fn exp_range(c: Range<u32>) -> Vec<usize> {
    c.map(|x| 2.pow(x)).collect::<Vec<_>>()
}

fn bench_union_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("union_find");
    for i in exp_range(5..11).iter() {
        group.throughput(Throughput::Elements(*i as u64));
        group.bench_with_input(BenchmarkId::new("quick_find", i), i, |b, &i| {
            b.iter(|| quick_find(black_box(i)))
        });
        group.bench_with_input(BenchmarkId::new("quick_union", i), i, |b, &i| {
            b.iter(|| quick_union(black_box(i)))
        });
        group.bench_with_input(BenchmarkId::new("weighted_quick_union", i), i, |b, &i| {
            b.iter(|| weighted_quick_union(black_box(i)))
        });
        group.bench_with_input(
            BenchmarkId::new("quick_union_with_path_compression", i),
            i,
            |b, &i| b.iter(|| quick_union_with_path_compression(black_box(i))),
        );
        group.bench_with_input(
            BenchmarkId::new("weighted_quick_union_with_path_compression", i),
            i,
            |b, &i| b.iter(|| weighted_quick_union_with_path_compression(black_box(i))),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_union_find);
criterion_main!(benches);
