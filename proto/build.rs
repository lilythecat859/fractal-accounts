fn main() -> Result<(), Box<dyn std::error::Error>> {
    for proto in &["auth.proto", "user.proto", "ledger.proto"] {
        tonic_build::configure()
            .build_server(true)
            .compile(&[proto], &[ "." ])?;
    }
    Ok(())
}
