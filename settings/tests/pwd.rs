use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use hex::ToHex;

const PASSWORD: &str = "UJU2g1sqL7QcflwEGE"; // Bad passwo

#[test]
fn test_verify_time() {
    let now = std::time::Instant::now();
    let r = bcrypt::verify(
        &"UJU2g1sqL7QcflwEGE",
        &"$2b$10$LKo4l6G9yhXa6/kINbN9zO6JTaMDFs8dKe632w6xZN2WqmMrR.DoK",
    );
    let _ = dbg!(r);
    println!("{} ms", now.elapsed().as_millis())
}

#[test]
fn test_argon2_hash() -> eyre::Result<()> {
    // let PASSWORD = b"UJU2g1sqL7QcflwEGE"; // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(PASSWORD.as_bytes(), &salt)?
        .to_string();
    println!("{}", password_hash);

    let now = std::time::Instant::now();
    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&password_hash)?;
    assert!(
        Argon2::default()
            .verify_password(PASSWORD.as_bytes(), &parsed_hash)
            .is_ok()
    );
    println!("{} ms", now.elapsed().as_millis());

    Ok(())
}
