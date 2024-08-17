use criterion::{criterion_group, criterion_main, Criterion};

use graveler::bits;

const TOTAL_ROLLS: u32 = 1_000_000_000;
//Runs 10 samples of 1 billion
pub fn bits_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bits_group");
    group.sample_size(10);
    group.bench_function("Bits", |b| b.iter(|| bits(TOTAL_ROLLS)));
    group.finish();
}



criterion_group!(benches, bits_benchmark);
criterion_main!(benches);