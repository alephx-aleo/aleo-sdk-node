use anyhow::Result;
use snarkvm::{circuit::Aleo, package::Package};

/// Synthesize keys and load into process
pub fn load_program_keys<A: Aleo>() -> Result<()> {
  let path = std::env::current_dir()?;
  let package = Package::open(&path)?;
  package.build::<A>(None)?;

  Ok(())
}
