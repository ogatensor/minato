// transaction_parameters.rs

use std::env;

pub struct TransactionParameters {
    pub input_token_amount: f64,
    pub output_token_amount: f64,
    pub gas_used: f64,
    pub base_fee_per_gas: f64,
    pub priority_fee_per_gas: f64,
}

impl TransactionParameters {
    pub fn from_environment() -> TransactionParameters {
        TransactionParameters {
            input_token_amount: env::var("INPUT_TOKEN_AMOUNT")
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                })
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing INPUT_TOKEN_AMOUNT: {}", err);
                    std::process::exit(1);
                }),
            output_token_amount: env::var("OUTPUT_TOKEN_AMOUNT")
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                })
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing OUTPUT_TOKEN_AMOUNT: {}", err);
                    std::process::exit(1);
                }),
            gas_used: env::var("GAS_USED")
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                })
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing GAS_USED: {}", err);
                    std::process::exit(1);
                }),
            base_fee_per_gas: env::var("BASE_FEE_PER_GAS")
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                })
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing BASE_FEE_PER_GAS: {}", err);
                    std::process::exit(1);
                }),
            priority_fee_per_gas: env::var("PRIORITY_FEE_PER_GAS")
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                })
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing PRIORITY_FEE_PER_GAS: {}", err);
                    std::process::exit(1);
                }),
        }
    }
}

