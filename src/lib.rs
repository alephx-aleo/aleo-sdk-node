#![deny(clippy::all)]

use env_logger::Builder;
use log::LevelFilter;
use napi::{
  bindgen_prelude::{BigInt, Undefined},
  Error, Result, Status,
};
use snarkvm::circuit::AleoTestnetV0;
use std::time::Instant;

mod execute;
mod fee;
mod process;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct BuildExecutionOptions {
  /// Base fee to execute transaction in microcredits
  pub base_fee: BigInt,
  pub endpoint: String,
  pub function_name: String,
  pub inputs: Vec<String>,

  /// Priority fee to execute transaction in microcredits
  pub priority_fee: BigInt,
  pub private_key: String,
  pub program_id: String,
  pub enable_log: Option<bool>,
}

#[napi(object)]
pub struct ExecutionResult {
  pub execution_time: String,
  pub id: String,

  /// Execution transaction payload to broadcast
  pub transaction: String,
}

#[napi]
pub async fn build_execution_transaction(opts: BuildExecutionOptions) -> Result<ExecutionResult> {
  let execution_time_start = Instant::now();

  let private_key = opts.private_key;
  let endpoint = opts.endpoint;
  let function_name = opts.function_name;
  let inputs = opts.inputs;
  let program_id = opts.program_id;

  let priority_fee = opts.priority_fee.get_u64().1;
  let base_fee = opts.base_fee.get_u64().1;
  let enable_log = opts.enable_log.is_some_and(|value| value == true);

  let options = execute::Options {
    private_key,
    endpoint,
    function_name,
    inputs,
    program_id,
    priority_fee,
    base_fee,
  };

  let level = if enable_log {
    LevelFilter::max()
  } else {
    LevelFilter::Off
  };
  let _ = Builder::new().filter_level(level).try_init();

  let build_result = execute::build_execution_transaction::<AleoTestnetV0>(options)
    .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

  let execution_time_end = execution_time_start.elapsed();
  let execution_time = format!("{:.2?}", execution_time_end);

  let result = ExecutionResult {
    transaction: build_result.transaction,
    execution_time,
    id: build_result.id,
  };

  Ok(result)
}

#[napi]
pub async fn load_program_keys() -> Result<Undefined> {
  let _ = process::load_program_keys::<AleoTestnetV0>()
    .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

  Ok(())
}
