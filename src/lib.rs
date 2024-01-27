pub mod contracts;

use alloy_chains::{Chain, NamedChain};
use alloy_rpc_types::CallInput;
use alloy_sol_types::SolCall;
use alloy_primitives::Address;
use contracts::EACAggregatorProxy::EACAggregatorProxy::*;
use serde::Deserialize;

// move all this calls to better UX
/// Reads the current answer from aggregator delegated to. (favour latest_round_data) 
pub fn latest_answer_call() -> CallInput {
    CallInput::new(latestAnswerCall{}.abi_encode().into())
}
/// get data about a round  
pub fn get_round_data_call(round_id: u128) -> CallInput {
    CallInput::new(getRoundDataCall { _roundId: round_id }.abi_encode().into())
}  
/// get data about the latest round
pub fn latest_round_data_call() -> CallInput {
    CallInput::new(latestRoundDataCall{}.abi_encode().into())
}  
/// Used if an aggregator contract has been proposed
pub fn proposed_get_round_data_call(round_id: u128) -> CallInput {
    CallInput::new(proposedGetRoundDataCall{_roundId: round_id}.abi_encode().into())
}
/// Used if an aggregator contract has been proposed.
pub fn proposed_latest_round_data_call() -> CallInput {
    CallInput::new(proposedLatestRoundDataCall{}.abi_encode().into())   
}
/// returns the current phase's aggregator address.
pub fn aggregator_call() -> CallInput {
    CallInput::new(aggregatorCall{}.abi_encode().into())
}
/// represents the number of decimals the aggregator responses represent
pub fn decimals_call() -> CallInput {
    CallInput::new(decimalsCall{}.abi_encode().into())
}
/// Returns description
pub fn description_call() -> CallInput {
    CallInput::new(descriptionCall{}.abi_encode().into())
}
/// Returns the address of the phase required by id
pub fn phase_aggregators(id: u16) -> CallInput {
    CallInput::new(phaseAggregatorsCall{_0: id}.abi_encode().into())
}
/// Returns the current phase
pub fn phase_id() -> CallInput {
    CallInput::new(phaseIdCall{}.abi_encode().into())
}
// getAnswer
// getTimestamp
// owner
// latestTimestamp
// and some more

/// References
/// adapt the other fields
pub fn get_ref_data_dir_url_prices(chain: NamedChain) -> Option<String> {
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
pub struct DataFeedIndexPrices {
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
    pub docs: serde_json::Value,//HashMap<String, String>,//DataFeedIndexDocs,
    pub decimals: Option<u8>
}

/// Main storage of data feeds oracles
/// currently only handles prices datafeeds
#[derive(Debug)]
pub struct DataFeeds {
    prices: Vec<DataFeedIndexPrices>
    // proof_of_reserve
    // nft_floors
    // more
}

impl DataFeeds {
    /// Returns a Datafeed loaded with chain specific indexes for prices feeds    
    pub async fn load_chain_feeds_prices(chain: Chain) -> Self {
        let url = get_ref_data_dir_url_prices(chain.named().unwrap()).expect("Error getting url of feeds");
        //println!("Populating information with {url}");
        let body = reqwest::get(url)
            .await.expect("Nope")
            .json::<serde_json::Value>()
            .await.expect("Noooope");
        let res: Vec<DataFeedIndexPrices> = serde_json::from_value(body).expect("Deserialization is still not possible"); 
        Self {
            prices: res
        }
    }

    pub fn print_all_entries_prices(&self) -> () {
        println!("{:#?}", self.prices)
    }
    
    /// Returns the oracle in storage if existent
    pub fn get_data_feed_prices(&self, token: &str, base: &str) -> Option<&DataFeedIndexPrices> {
        let index = format!("{} / {}", token, base);
        self.prices
            .iter()
            .find(|r| r.name.clone().unwrap_or("none".to_string()) == index)
    }


}

#[cfg(test)]
mod tests {
    use super::*;
/* 
    #[test]
    fn it_works() {
        let st = DataFeeds::with_initial_values();
        let t = st.get_oracle(
             Chain::mainnet(),
             "1INCH",
            "ETH"
        ).unwrap();
        assert_eq!(t.address, ("0x72AFAECF99C9d9C8215fF44C77B94B99C28741e8".parse::<Address>().unwrap()))
    }
 */
//    #[test] getting price?

}
/* 
not used bc of the dyn nature of it.. currently using serde_json::Value
/// Chainlink reference-data-directory docs
/// investigate
/// yes, there are many variations of the docs below.. 
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