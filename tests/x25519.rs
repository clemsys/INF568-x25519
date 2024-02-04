use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn correct_key_gen_alice() -> TestResult {
    let mut cmd = Command::cargo_bin("x25519")?;

    let alice_private = "77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a";
    let alice_public = "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a";

    cmd.args([alice_private])
        .assert()
        .success()
        .stdout(alice_public);
    Ok(())
}

#[test]
fn correct_key_exchange_alice() -> TestResult {
    let mut cmd = Command::cargo_bin("x25519")?;

    let alice_private = "77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a";
    let bob_public = "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f";
    let shared_secret = "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742";

    cmd.args([alice_private, bob_public])
        .assert()
        .success()
        .stdout(shared_secret);
    Ok(())
}

#[test]
fn correct_key_gen_bob() -> TestResult {
    let mut cmd = Command::cargo_bin("x25519")?;

    let bob_private = "5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb";
    let bob_public = "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f";

    cmd.args([bob_private])
        .assert()
        .success()
        .stdout(bob_public);
    Ok(())
}

#[test]
fn correct_key_exchange_bob() -> TestResult {
    let mut cmd = Command::cargo_bin("x25519")?;

    let bob_private = "5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb";
    let alice_public = "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a";
    let shared_secret = "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742";

    cmd.args([bob_private, alice_public])
        .assert()
        .success()
        .stdout(shared_secret);
    Ok(())
}
