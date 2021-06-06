use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct SingleMarketResult {
    name: String,
    bid: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleMarket {
    success: bool,
    result: SingleMarketResult
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://ftx.com/api/markets/BTC-PERP")
        .await?
        .text()
        .await?;

    let res: SingleMarket = serde_json::from_str(&resp)?;

    println!("{:#?}", res);
    Ok(())
}