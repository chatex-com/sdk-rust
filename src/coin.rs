/*
[
    Coin { decimals: 8, full_name: "Bitcoin", name: "btc" },
    Coin { decimals: 8, full_name: "Litecoin", name: "ltc" },
    Coin { decimals: 8, full_name: "Bitcoin Cash", name: "bch" },
    Coin { decimals: 6, full_name: "Ripple", name: "xrp" },
    Coin { decimals: 8, full_name: "Bitcoin Gold", name: "btg" },
    Coin { decimals: 18, full_name: "Ethereum", name: "eth" },
    Coin { decimals: 6, full_name: "Tron", name: "trx" },
    Coin { decimals: 8, full_name: "Dash", name: "dash" },
    Coin { decimals: 6, full_name: "USDT ERC20", name: "usdt" },
    Coin { decimals: 9, full_name: "TON Crystal", name: "ton_crystal" }
]
*/

// Implementing it as enum mb bad idea. Seems like set of coins could change at runtime.
#[derive(PartialEq, Clone, Debug)]
pub enum Coin {
    BTC,
    LTC,
    BCH,
    XRP,
    BTG,
    ETH,
    TRX,
    DASH,
    USDT,
    TON,
    Unknown(String),
}

impl Coin {
    pub fn get_name(&self) -> &str {
        match self {
            Coin::BTC => "btc",
            Coin::LTC => "ltc",
            Coin::BCH => "bch",
            Coin::XRP => "xrp",
            Coin::BTG => "btg",
            Coin::ETH => "eth",
            Coin::TRX => "trx",
            Coin::DASH => "dash",
            Coin::USDT => "usdt",
            Coin::TON => "ton_crystal",
            Coin::Unknown(name) => name,
        }
    }
}

impl std::fmt::Display for Coin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str(self.get_name())
    }
}

#[derive(PartialEq, Clone)]
pub struct CoinPair {
    pub left: Coin,
    pub right: Coin,
}

impl CoinPair {
    pub fn new(left: Coin, right: Coin) -> CoinPair {
        CoinPair { left, right }
    }

    pub fn reversed(&self) -> CoinPair {
        CoinPair::new(self.right.clone(), self.left.clone())
    }
}

impl From<&CoinPair> for String {
    fn from(pair: &CoinPair) -> String {
        format!(
            "{}/{}",
            String::from(pair.left.clone()),
            String::from(pair.right.clone())
        )
    }
}

impl From<CoinPair> for String {
    fn from(pair: CoinPair) -> String {
        format!("{}/{}", String::from(pair.left), String::from(pair.right))
    }
}

impl From<&str> for Coin {
    fn from(coin: &str) -> Coin {
        match coin {
            "btc" => Coin::BTC,
            "ltc" => Coin::LTC,
            "bch" => Coin::BCH,
            "xrp" => Coin::XRP,
            "btg" => Coin::BTG,
            "eth" => Coin::ETH,
            "trx" => Coin::TRX,
            "dash" => Coin::DASH,
            "usdt" => Coin::USDT,
            "ton_crystal" => Coin::TON,
            _ => Coin::Unknown(String::from(coin)),
        }
    }
}

impl From<Coin> for String {
    fn from(coin: Coin) -> String {
        match coin {
            Coin::BTC => "btc".to_owned(),
            Coin::LTC => "ltc".to_owned(),
            Coin::BCH => "bch".to_owned(),
            Coin::XRP => "xrp".to_owned(),
            Coin::BTG => "btg".to_owned(),
            Coin::ETH => "eth".to_owned(),
            Coin::TRX => "trx".to_owned(),
            Coin::DASH => "dash".to_owned(),
            Coin::USDT => "usdt".to_owned(),
            Coin::TON => "ton_crystal".to_owned(),
            Coin::Unknown(name) => name,
        }
    }
}
