// minato/src/uniswap_api.rs

use reqwest;
use serde_json;

// New UniswapAPI class
struct UniswapAPI {
    // HTTP client
    client: reqwest::blocking::Client,
}

impl UniswapAPI {
    // Constructor
    pub fn new() -> Self {
        UniswapAPI {
            client: reqwest::blocking::Client::new(),
        }
    }

    // Method to fetch optimal route and gas estimates from Uniswap API
    pub fn fetch_optimal_route(
        &self,
        input_token: &Token,
        output_token: &Token,
        input_amount: U256,
        block_number: u64,
    ) -> (Route, U256) {
        // Prepare API request
        let api_url = format!("https://api.uniswap.org/v1/route?input_token={}&output_token={}&input_amount={}&block_number={}",
                              input_token.address, output_token.address, input_amount, block_number);
        let api_response = self.client.get(&api_url).bytes().unwrap();

        // Parse API response
        let api_data: serde_json::Value = serde_json::from_slice(&api_response).unwrap();
        let optimal_route = api_data["route"].as_str().unwrap().parse::<Route>().unwrap();
        let estimated_gas = api_data["gas"].as_u64().unwrap().into();

        return (optimal_route, estimated_gas);
    }
}

