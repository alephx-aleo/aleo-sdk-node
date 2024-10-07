use std::str::FromStr;
use anyhow::{ensure, Result};
use log::debug;
use snarkvm::{circuit::Aleo, ledger::{query::Query, store::helpers::memory::BlockMemory, Transaction}, package::Package, prelude::{Identifier, PrivateKey, ProgramID, Value}};

use crate::fee::build_transaction_fee;

pub struct Options {
    pub base_fee: u64,
    pub endpoint: String,
    pub function_name: String,
    pub inputs: Vec<String>,
    pub priority_fee: u64,
    pub private_key: String,
    pub program_id: String
}

pub struct ExecuteOptions<A:Aleo> {
    pub base_fee: u64,
    pub endpoint: String,
    pub function_name: Identifier<A::Network>,
    pub inputs: Vec<Value<A::Network>>,
    pub priority_fee: u64,
    pub private_key: PrivateKey<A::Network>,
    pub program_id: ProgramID<A::Network>
}

impl<A:Aleo> ExecuteOptions<A> {
    pub fn new(options: Options) -> Self {
        // TODO: Add validations

        let function_name = Identifier::<A::Network>::from_str(&options.function_name).unwrap();
        let inputs: Vec<Value<A::Network>> = options.inputs.iter()
            .map(|item| Value::from_str(item).unwrap())
            .collect();
        let endpoint = options.endpoint;
        let private_key = PrivateKey::<A::Network>::from_str(&options.private_key).unwrap();
        let program_id = ProgramID::<A::Network>::from_str(&options.program_id).unwrap();

        Self {
            base_fee: options.base_fee,
            endpoint,
            function_name,
            inputs,
            priority_fee: options.priority_fee,
            private_key,
            program_id
        }
    }

    pub fn query(&self) -> Query::<A::Network, BlockMemory<A::Network>> {
        Query::<A::Network, BlockMemory<A::Network>>::from(&self.endpoint.clone())
    }
}

pub struct BuildResult {
    pub transaction: String,
    pub id: String,
    // pub priority_fee: u64,
    // pub base_fee: u64,
}

pub fn build_execution_transaction<A:Aleo>(options: Options) -> Result<BuildResult> {
    let execute_options = ExecuteOptions::<A>::new(options);
    let endpoint = &execute_options.endpoint;
    let function_name = &execute_options.function_name;
    let inputs = &execute_options.inputs;
    let private_key = &execute_options.private_key;
    let program_id = &execute_options.program_id;

    let rng = &mut rand::thread_rng();
    
    // Derive the program directory path.
    let path = std::env::current_dir()?;
    let package = Package::open(&path)?;
    let process = package.get_process()?;
  
    debug!("Executing the request...");
    let (
        _response,
        execution, 
        _metrics
    ) = package.execute::<A, _>(endpoint.clone(), &private_key, function_name.clone(), inputs, rng)?;

    let execution_id = execution.to_execution_id()?;
    let has_program = process.contains_program(&ProgramID::<A::Network>::from_str(&program_id.to_string())?);
    ensure!(has_program, "program is not found in process. Check if you have included main.aleo and program.json in the directory.");

    let fee = build_transaction_fee::<A>(process, rng, execution_id, execute_options)?;
    debug!("Fee: {:?}", fee);

    let transaction = Transaction::from_execution(execution, fee)?;

    debug!("[DONE]");

    let result = BuildResult {
        transaction: transaction.to_string(),
        id: transaction.id().to_string()
    };

    Ok(result)
}
