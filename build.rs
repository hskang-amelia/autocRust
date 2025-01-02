fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["proto/vehicle_status.proto"], &["proto"])?;
    Ok(())
}