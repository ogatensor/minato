use std::env;

mod transaction_parameters;
mod price_calculator;

fn main() {
    let params = transaction_parameters::TransactionParameters::from_environment();

    let realized_price = price_calculator::PriceCalculator::calculate_realized_price(params);

    println!("Realized Price: {}", realized_price);
}

