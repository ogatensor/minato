use std::env;
use alloy_contract::CallBuilder;
use alloy_provider::ProviderBuilder;

mod transaction_parameters;
mod price_calculator;

mod contracts;
use crate::contracts::ofa_analysis_contract::OFAAnalysisContract;

fn main() {
    let params = transaction_parameters::TransactionParameters::from_environment();

    let provider = ProviderBuilder::new().build().await?;
    let call_builder = CallBuilder::new(&provider, contract_address);
    let result = call_builder.call_function("calculateRealizedPrice", params).await?;

    println!("Realized Price: {}", result);

    let ofa_analysis_contract = OFAAnalysisContract::new();
    let result = ofa_analysis_contract.execute();
    println!("{}", result);
}

