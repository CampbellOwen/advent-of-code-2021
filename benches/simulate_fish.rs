use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn tick_fish(state: &mut [usize; 9]) {
    state.rotate_left(1);
    state[6] += state[8];
}

fn simulate_days(initial_state: &mut [usize; 9], num_days: usize) {
    for _ in 0..num_days {
        tick_fish(initial_state);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simualte_fish", |b| {
        b.iter(|| {
            let mut initial_state = [0, 84, 59, 54, 48, 55, 0, 0, 0];
            simulate_days(&mut initial_state, 256);
            let _ = initial_state.iter().sum::<usize>();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
