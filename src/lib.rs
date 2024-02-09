pub mod contracts;

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::Address;
use serde::Deserialize;

/// References
/// taken from https://reference-data-directory.vercel.app
pub fn get_references_url(chain: NamedChain) -> Option<String> {
    let base = "https://reference-data-directory.vercel.app";
    let url = match chain {
        NamedChain::Mainnet => format!("{}/feeds-mainnet.json",base),
        NamedChain::Sepolia => format!("{}/feeds-ethereum-testnet-sepolia.json",base),
        NamedChain::Goerli => format!("{}/feeds-goerli.json",base),
        NamedChain::BinanceSmartChain => format!("{}/feeds-bsc-mainnet.json",base),
        NamedChain::BinanceSmartChainTestnet => format!("{}/feeds-bsc-testnet.json",base),
        NamedChain::Polygon => format!("{}/feeds-matic-mainnet.json",base),
        NamedChain::PolygonMumbai => format!("{}/feeds-matic-testnet.json",base), 
        NamedChain::Gnosis => format!("{}/feeds-xdai-mainnet.json",base),
        NamedChain::Avalanche => format!("{}/feeds-avalanche-mainnet.json",base),
        NamedChain::AvalancheFuji => format!("{}/feeds-avalanche-fuji-testnet.json",base),
        NamedChain::Fantom => format!("{}/feeds-fantom-mainnet.json",base),
        NamedChain::FantomTestnet => format!("{}/feeds-fantom-testnet.json",base),
        NamedChain::Arbitrum => format!("{}/feeds-ethereum-mainnet-arbitrum-1.json",base),
        NamedChain::ArbitrumSepolia => format!("{}/feeds-ethereum-testnet-sepolia-arbitrum-1.json",base),
        NamedChain::Optimism => format!("{}/feeds-ethereum-mainnet-optimism-1.json",base),
        NamedChain::OptimismSepolia => format!("{}/feeds-ethereum-testnet-sepolia-optimism-1.json",base),
        NamedChain::OptimismGoerli => format!("{}/feeds-ethereum-testnet-goerli-optimism-1.json",base),
        NamedChain::Moonriver => format!("{}/feeds-kusama-mainnet-moonriver.json",base),
        NamedChain::Moonbeam => format!("{}/feeds-polkadot-mainnet-moonbeam.json",base),
        NamedChain::Metis => format!("{}/feeds-ethereum-mainnet-andromeda-1.json",base),
        NamedChain::Base => format!("{}/feeds-ethereum-mainnet-base-1.json",base),
        NamedChain::BaseGoerli => format!("{}/feeds-ethereum-testnet-goerli-base-1.json",base),
        NamedChain::Celo => format!("{}/feeds-celo-mainnet.json",base),
        NamedChain::CeloAlfajores => format!("{}/feeds-celo-testnet-alfajores.json",base),
        NamedChain::Scroll => format!("{}/feeds-ethereum-mainnet-scroll-1.json",base),
        NamedChain::ScrollSepolia => format!("{}/feeds-ethereum-testnet-sepolia-scroll-1.json",base),
        NamedChain::Linea => format!("{}/feeds-ethereum-mainnet-linea-1.json",base),
        NamedChain::LineaTestnet => format!("{}/feeds-ethereum-testnet-goerli-linea-1.json", base),
        NamedChain::ZkSync => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-mainnet-zksync-1.json".to_string(),
        NamedChain::ZkSyncTestnet => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-testnet-goerli-zksync-1.json".to_string(),
        NamedChain::PolygonZkEvm => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-mainnet-polygon-zkevm-1.json".to_string(),
        NamedChain::PolygonZkEvmTestnet => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-testnet-goerli-polygon-zkevm-1.json".to_string(),            
        _ => panic!("Network not supported")
    };
    Some(url)
}

/// Chainlink reference-data-directory model
/// many fields are actually enums
/// check numeric values if are ok (i just guess'em)
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Oracle {
    pub compare_offchain: Option<String>,
    pub contract_address: Option<Address>,
    pub contract_type: Option<String>,
    pub contract_version: Option<u8>,
    pub decimal_places: Option<u8>,
    pub ens: Option<String>,
    pub format_decimal_places: Option<u8>,
    pub health_price: Option<String>,
    pub heartbeat: Option<u32>,
    pub history: bool,
    pub multiply: Option<String>,
    pub name: Option<String>,
    pub pair: Vec<String>,
    pub path: Option<String>,
    pub proxy_address: Option<Address>,
    pub threshold: Option<f64>,
    pub value_prefix: Option<String>,
    pub asset_name: Option<String>,
    pub feed_category: Option<String>,
    pub feed_type: Option<String>,
    pub docs: serde_json::Value,
    pub decimals: Option<u8>
}

/// Main storage of data feeds oracles
/// Different types of feeds (see feed_category) are mixed in this index
#[derive(Debug)]
pub struct OraclesIndex {
    pub chain: Chain,
    pub feeds: Vec<Oracle>,
}

impl OraclesIndex {
    /// Returns a struct loaded with chain specific indexes for prices feeds    
    pub async fn load_reference_feeds(chain: Chain) -> Self {
        let url = get_references_url(chain.named().unwrap()).expect("Error getting url of feeds");
        //println!("Populating information with {url}");
        let body = reqwest::get(url)
            .await.expect("Cannot request reference index")
            .json::<serde_json::Value>()
            .await.expect("Could not retrieve reference index");
        let res: Vec<Oracle> = serde_json::from_value(body).expect("Deserialization is not possible"); 
        Self {
            chain: chain,
            feeds: res
        }
    }

    pub fn print_all_references(&self) -> () {
        println!("{:#?}", self.feeds)
    }
    
    /// Returns the oracle in storage if existent
    pub fn get_oracle(&self, token: &str, base: &str) -> Option<&Oracle> {
        let index = format!("{} / {}", token, base);
        self.feeds
            .iter()
            .find(|r| r.name.clone().unwrap_or("none".to_string()) == index)
    }
}

 /* 
not used bc of the dyn nature of it.. currently using serde_json::Value
/// Chainlink reference-data-directory docs
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DataFeedIndexDocs {
    asset_class: Option<String>,//"Fiat",
    asset_name: Option<String>,//"Japanese Yen",
    base_asset: Option<String>,//"JPY",
    base_asset_clic: Option<String>,//"JPY_FX",
    blockchain_name: Option<String>,//"Ethereum",
    clic_product_name: Option<String>,//"JPY/USD-RefPrice-DF-Ethereum-001",
    delivery_channel_code: Option<String>,//"DF",
    feed_category: Option<String>,//"verified",
    feed_type: Option<String>,//"Forex",
    market_hours: Option<String>,//"Forex",
    product_sub_type: Option<String>,//"Reference",
    product_type: Option<String>,//"Price",
    product_typeCode: Option<String>,//"RefPrice",
    quote_asset: Option<String>,//"USD",
    quote_asset_clic: Option<String>,//"USD_FX"
} 
*/