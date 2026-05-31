use std::{error::Error, io, io::BufRead};

use algorithms4::{UnionFind, WeightedQuickUnionWPC};
use rand::prelude::*;

fn stats(v: Vec<usize>) -> (f64, f64, f64) {
    let mean = v.iter().sum::<usize>() as f64 / v.len() as f64;
    let variance = v.iter().map(|&v| (v as f64 - mean).powi(2)).sum::<f64>() / v.len() as f64;
    let stddev = variance.sqrt();

    (mean, variance, stddev)
}

fn count(n: usize) -> usize {
    let mut uf = WeightedQuickUnionWPC::new(n);
    let mut rand = rand::rng();
    let mut edges = 0;
    while uf.count() > 1 {
        let v = rand.random_range(0..n);
        let w = rand.random_range(0..n);
        if !uf.connected(v, w) {
            uf.union(v, w).unwrap();
        }
        edges += 1;
    }

    edges
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines
        .next()
        .ok_or("no number of vertices given")??
        .parse::<usize>()?;
    let trials = lines
        .next()
        .ok_or("no numer of trials given")??
        .parse::<usize>()?;

    let mut edges = Vec::with_capacity(trials);

    for _ in 0..trials {
        edges.push(count(n));
    }

    let (mean, _, stddev) = stats(edges);
    println!("1/2 n ln n = {}", 0.5 * n as f64 * (n as f64).ln());
    println!("mean       = {}", mean);
    println!("std. dev.  = {}", stddev);

    /*
        // report statistics
        StdOut.println("1/2 n ln n = " + 0.5 * n * Math.log(n));
        StdOut.println("mean       = " + StdStats.mean(edges));
        StdOut.println("stddev     = " + StdStats.stddev(edges));
    */

    Ok(())
}
