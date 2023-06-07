use advent_of_code::day_6::{get_marker, get_marker_fixed_bitwise, INPUT as DAY6INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("get_marker_hashset_4", |b| {
    //     b.iter(|| get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", black_box(4)))
    // });
    // c.bench_function("get_marker_hashset_input_4", |b| {
    //     b.iter(|| get_marker(DAY6INPUT, black_box(4)))
    // });
    // c.bench_function("get_marker_fixed_bitwise_4", |b| {
    //     b.iter(|| get_marker_fixed_bitwise("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", black_box(4)))
    // });
    c.bench_function("get_marker_fixed_bitwise_input_w_4", |b| {
        b.iter(|| get_marker_fixed_bitwise(DAY6INPUT, black_box(4)))
    });
    c.bench_function("get_marker_fixed_bitwise_input_w_14", |b| {
        b.iter(|| get_marker_fixed_bitwise(DAY6INPUT, 14))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
