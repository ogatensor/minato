use std::env;

struct TransactionParameters {
    input_token_amount: f64,
    output_token_amount: f64,
    gas_used: f64,
    base_fee_per_gas: f64,
    priority_fee_per_gas: f64,
}

impl TransactionParameters {
    fn from_environment() -> TransactionParameters {
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

fn main() {
    let params = TransactionParameters::from_environment();

    // Calculate the realized price using the formula from [[Realized Price Definition]]:
    // Realized Price = Output Token Amount / (Input Token Amount + Gas Used * (Base Fee Per Gas + Priority Fee Per Gas))
    let realized_price = params.output_token_amount / (params.input_token_amount + params.gas_used * (params.base_fee_per_gas + params.priority_fee_per_gas));

    // Output the calculated realized price to the console.
    println!("Realized Price: {}", realized_price);
}

