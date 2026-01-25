use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use rand::Rng;
use rand::distr::Alphanumeric;
use rand::rng;
use settings::Argon2Options;
use settings::BcryptOptions;
use settings::PwdHasher;

fn bcrypt_10() -> eyre::Result<()> {
    let rand_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(18)
        .map(char::from)
        .collect();
    let hasher = PwdHasher::Bcrypt(Some(BcryptOptions { cost: 10 }));
    hasher.hash(rand_string)?;
    Ok(())
}
fn argon2_default() -> eyre::Result<()> {
    let rand_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(18)
        .map(char::from)
        .collect();
    let hasher = PwdHasher::Argon2(Some(Argon2Options::default()));

    hasher.hash(rand_string)?;
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bcrypt_10", |b| b.iter(|| bcrypt_10()));
    c.bench_function("argon2_default", |b| b.iter(|| argon2_default()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
