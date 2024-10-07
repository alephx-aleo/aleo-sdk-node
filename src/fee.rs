use anyhow::Result;
use log::{debug, warn};
use rand::rngs::ThreadRng;
use snarkvm::{circuit::Aleo, ledger::Fee, prelude::{Field, Process}};

use crate::execute::ExecuteOptions;

/// Build fee for execution transaction. Mainly, to generate fee proof and transitions.
pub fn build_transaction_fee<A:Aleo>(
    process: Process<A::Network>,
    rng: &mut ThreadRng,
    execution_id: Field<A::Network>,
    options: ExecuteOptions<A>
) -> Result<Option<Fee<A::Network>>> {
    let base_fee = options.base_fee;
    let function_name = &options.function_name;
    let inputs = &options.inputs;
    let priority_fee = options.priority_fee;
    let private_key = &options.private_key;
    let program_id = &options.program_id;
    let query = options.query();

    let authorization = process.authorize::<A, _>(
        &private_key,
        program_id, 
        function_name, 
        inputs.iter(), 
        rng
    )?;

    let is_priority_fee_declared = priority_fee > 0;

    if is_priority_fee_declared {
        warn!("Priority fee is not supported currently");
    }

    let is_fee_required = !authorization.is_split();

    if is_fee_required {
        debug!("Fee is required\nCalculating fee...");

        let authorization_fee_public = process.authorize_fee_public::<A, _>(
            &private_key, 
            base_fee, 
            priority_fee, 
            execution_id, 
            rng
        )?;
    
        debug!("Fee public: {}", authorization_fee_public.to_string());
        let (
            _auth_res, 
            mut trace
        ) = process.execute::<A, _>(authorization_fee_public, rng)?;
        
        trace.prepare(query)?;
    
        let fee = trace.prove_fee::<A, _>(rng)?;
        debug!("Fee: {}", fee.to_string());

        Ok(Some(fee))
    } else {
        Ok(None)
    }
}
