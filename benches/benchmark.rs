use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bashrand::{CollisionCracker, New3Cracker, Old3Cracker, OneResultCracker};
use crossbeam_channel::unbounded;

fn new_bench(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| {
            let cracker = New3Cracker::new(black_box([7195, 26887, 6346]));
            assert_eq!(cracker.find(), Some(black_box(31337)));
        })
    });
}

fn old_bench(c: &mut Criterion) {
    c.bench_function("old", |b| {
        b.iter(|| {
            let cracker = Old3Cracker::new(black_box([895, 5874, 11135]));
            assert_eq!(cracker.find(), Some(black_box(31337)));
        })
    });
}

fn collide_bench(c: &mut Criterion) {
    c.bench_function("collide", |b| {
        b.iter(|| {
            let (tx, rx) = unbounded();
            let cracker = CollisionCracker::new(black_box(37));
            cracker.find(&tx);
            let (res1, res2) = (rx.recv().unwrap(), rx.recv().unwrap());
            assert!(res1 == 901656913 || res2 == 901656913);
            assert!(res1 == 910416221 || res2 == 910416221);
        })
    });
}

criterion_group!(benches, new_bench, old_bench, collide_bench);
criterion_main!(benches);
