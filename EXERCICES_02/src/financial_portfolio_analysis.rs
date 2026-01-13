use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Risk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct Asset {
    ticker: &'static str,
    quantity: u32,
    price: f64,
    risk: Risk,
}

pub fn portfolio() -> Vec<Asset> {
    vec![
        Asset { ticker: "AAPL", quantity: 10, price: 150.0, risk: Risk::Low },
        Asset { ticker: "TSLA", quantity: 5, price: 700.0, risk: Risk::High },
        Asset { ticker: "AMZN", quantity: 2, price: 3000.0, risk: Risk::Medium },
        Asset { ticker: "GOOGL", quantity: 3, price: 2500.0, risk: Risk::Medium },
        Asset { ticker: "NVDA", quantity: 4, price: 200.0, risk: Risk::High },
    ]
}

pub fn total_value(assets: &[Asset]) -> f64 {
    assets
        .iter()
        .map(|a| a.price * a.quantity as f64)
        .fold(0.0, |acc, v| acc + v)
}

pub fn diversification_by_risk(assets: &[Asset]) -> HashMap<Risk, f64> {
    let mut map = HashMap::new();

    for asset in assets {
        let value = asset.price * asset.quantity as f64;
        *map.entry(asset.risk.clone()).or_insert(0.0) += value;
    }

    map
}

pub fn diversification_score(map: &HashMap<Risk, f64>) -> usize {
    map.len()
}

pub fn high_risk_assets(assets: &[Asset]) -> Vec<Asset> {
    assets
        .iter()
        .filter(|a| a.risk == Risk::High)
        .cloned()
        .collect()
}

pub fn assets_above_threshold(assets: &[Asset], threshold: f64) -> Vec<Asset> {
    assets
        .iter()
        .filter(|a| a.price * a.quantity as f64 >= threshold)
        .cloned()
        .collect()
}

