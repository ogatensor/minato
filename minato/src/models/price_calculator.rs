// price_calculator.rs

use crate::transaction_parameters::TransactionParameters;

pub struct PriceCalculator;

impl PriceCalculator {
    pub fn calculate_realized_price(params: TransactionParameters) -> f64 {
        params.output_token_amount / (params.input_token_amount + params.gas_used * (params.base_fee_per_gas + params.priority_fee_per_gas))
    }
}

