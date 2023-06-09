use criterion::{criterion_group, criterion_main, Criterion};

use bashrand::{CertainCracker, New3Cracker, Old3Cracker};

fn new_bench(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| {
            let cracker = New3Cracker::new([7195, 26887, 6346]);
            assert_eq!(cracker.find(), Some(31337));
        })
    });
}

fn old_bench(c: &mut Criterion) {
    c.bench_function("old", |b| {
        b.iter(|| {
            let cracker = Old3Cracker::new([895, 5874, 11135]);
            assert_eq!(cracker.find(), Some(31337));
        })
    });
}

criterion_group!(benches, new_bench, old_bench);
criterion_main!(benches);
