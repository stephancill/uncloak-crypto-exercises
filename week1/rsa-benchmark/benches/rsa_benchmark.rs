use rsa_benchmark::rsa_encrypt;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn rsa_benchmark(c: &mut Criterion) {
    let data = black_box(b"hello world");
    c.bench_function("rsa_encrypt", |b| b.iter(|| rsa_encrypt(data)));
}

criterion_group!(benches, rsa_benchmark);
criterion_main!(benches);