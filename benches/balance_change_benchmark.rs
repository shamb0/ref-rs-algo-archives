use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use v1_recursive::Block as Block1;
use v1_recursive::find_balance_changes as find_balance_changes_recursive;
use v2_iterative::Block as Block2;
use v2_iterative::find_balance_changes as find_balance_changes_iterative;

fn generate_mock_data(size: usize) -> (Vec<Block1>, Vec<Block2>) {
    let mut rng = rand::thread_rng();
    let data1: Vec<Block1> = (0..size)
        .map(|_| Block1 { balance: rng.gen_range(0..1000) })
        .collect();
    let data2: Vec<Block2> = data1.iter().map(|b| Block2 { balance: b.balance }).collect();
    (data1, data2)
}

fn benchmark_balance_change(c: &mut Criterion) {
    let (data1, data2) = generate_mock_data(2000);

    c.bench_function("recursive_approach", |b| {
        b.iter(|| find_balance_changes_recursive(black_box(&data1), black_box(0), black_box(1999)))
    });

    c.bench_function("iterative_approach", |b| {
        b.iter(|| find_balance_changes_iterative(black_box(&data2), black_box(0), black_box(1999)))
    });
}

criterion_group!(benches, benchmark_balance_change);
criterion_main!(benches);