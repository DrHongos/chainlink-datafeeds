pub mod contracts;

use std::collections::HashMap;
use alloy_chains::{Chain, NamedChain};
use alloy_primitives::Address;
use alloy_rpc_types::CallInput;
use alloy_sol_types::SolCall;
use contracts::EACAggregatorProxy::EACAggregatorProxy::*;
use serde::Deserialize;

/// Refers to an instance of EACAggregatorProxy
/// used to request data feeds from Chainlink
/// this data is hardcoded and should be checked, updated and complete
/// <https://github.com/smartcontractkit/chainlink/blob/develop/contracts/src/v0.7/dev/AggregatorProxy.sol>
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Oracle {
    pub address: Address,
    pub deviation: u32,
    pub heartbeat: u64,
    pub decimals: u8, 
}
/// all CallInput functions aren't needed in here (self is not used) but it creates the "contract" usage style
impl Oracle {
    /// Reads the current answer from aggregator delegated to. (favour latest_round_data) 
    pub fn latest_answer_call(&self) -> CallInput {
        CallInput::new(latestAnswerCall{}.abi_encode().into())
    }
    /// get data about a round  
    pub fn get_round_data_call(&self, round_id: u128) -> CallInput {
        CallInput::new(getRoundDataCall { _roundId: round_id }.abi_encode().into())
    }  
    /// get data about the latest round
    pub fn latest_round_data_call(&self) -> CallInput {
        CallInput::new(latestRoundDataCall{}.abi_encode().into())
    }  
    /// Used if an aggregator contract has been proposed
    pub fn proposed_get_round_data_call(&self, round_id: u128) -> CallInput {
        CallInput::new(proposedGetRoundDataCall{_roundId: round_id}.abi_encode().into())
    }
    /// Used if an aggregator contract has been proposed.
    pub fn proposed_latest_round_data_call(&self) -> CallInput {
        CallInput::new(proposedLatestRoundDataCall{}.abi_encode().into())   
    }
    /// returns the current phase's aggregator address.
    pub fn aggregator_call(&self) -> CallInput {
        CallInput::new(aggregatorCall{}.abi_encode().into())
    }
    /// represents the number of decimals the aggregator responses represent
    pub fn decimals_call(&self) -> CallInput {
        CallInput::new(decimalsCall{}.abi_encode().into())
    }
    /// Returns description
    pub fn description_call(&self) -> CallInput {
        CallInput::new(descriptionCall{}.abi_encode().into())
    }
    /// Returns the address of the phase required by id
    pub fn phase_aggregators(&self, id: u16) -> CallInput {
        CallInput::new(phaseAggregatorsCall{_0: id}.abi_encode().into())
    }
    /// Returns the current phase
    pub fn phase_id(&self) -> CallInput {
        CallInput::new(phaseIdCall{}.abi_encode().into())
    }

    // getAnswer
    // getTimestamp
    // owner
    // latestTimestamp
    // and some more

}

/// Key for storage of data feeds
/// probably dont need Chain but NamedChain
/// also necessary to think about other storages like tokens(chain) or base(token(chain))
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Key {
    pub chain: Chain,
    pub token: String,
    pub base: String,
}
impl Key {
    pub fn from(chain: Chain, token: String, base: String) -> Self {
        Self {
            chain,
            token,
            base
        }
    }
}

/// References
/// see all networks online, map names (gnosis is xdai-mainnet, etc)
pub fn get_ref_data_dir_url(chain: NamedChain) -> Option<String> {
    let url = match chain {
        NamedChain::Mainnet => "https://reference-data-directory.vercel.app/feeds-mainnet.json",
        NamedChain::Sepolia => "https://reference-data-directory.vercel.app/feeds-ethereum-testnet-sepolia.json",
        NamedChain::Goerli => "https://reference-data-directory.vercel.app/feeds-goerli.json",
        NamedChain::BinanceSmartChain => "https://reference-data-directory.vercel.app/feeds-bsc-mainnet.json",
        NamedChain::BinanceSmartChainTestnet => "https://reference-data-directory.vercel.app/feeds-bsc-testnet.json",
        NamedChain::Polygon => "https://reference-data-directory.vercel.app/feeds-matic-mainnet.json",
        NamedChain::PolygonMumbai => "https://reference-data-directory.vercel.app/feeds-matic-testnet.json", 
        NamedChain::Gnosis => "https://reference-data-directory.vercel.app/feeds-xdai-mainnet.json",
        NamedChain::Avalanche => "https://reference-data-directory.vercel.app/feeds-avalanche-mainnet.json",
        NamedChain::AvalancheFuji => "https://reference-data-directory.vercel.app/feeds-avalanche-fuji-testnet.json",
        NamedChain::Fantom => "https://reference-data-directory.vercel.app/feeds-fantom-mainnet.json",
        NamedChain::FantomTestnet => "https://reference-data-directory.vercel.app/feeds-fantom-testnet.json",
        NamedChain::Arbitrum => "https://reference-data-directory.vercel.app/feeds-ethereum-mainnet-arbitrum-1.json",
        NamedChain::ArbitrumSepolia => "https://reference-data-directory.vercel.app/feeds-ethereum-testnet-sepolia-arbitrum-1.json",
        NamedChain::Optimism => "https://reference-data-directory.vercel.app/feeds-ethereum-mainnet-optimism-1.json",
        NamedChain::OptimismSepolia => "https://reference-data-directory.vercel.app/feeds-ethereum-testnet-sepolia-optimism-1.json",
        NamedChain::OptimismGoerli => "https://reference-data-directory.vercel.app/feeds-ethereum-testnet-goerli-optimism-1.json",
        NamedChain::Moonriver => "https://reference-data-directory.vercel.app/feeds-kusama-mainnet-moonriver.json",
        NamedChain::Moonbeam => "https://reference-data-directory.vercel.app/feeds-polkadot-mainnet-moonbeam.json",
        NamedChain::Metis => "https://reference-data-directory.vercel.app/feeds-ethereum-mainnet-andromeda-1.json",
        NamedChain::Base => "https://reference-data-directory.vercel.app/feeds-ethereum-mainnet-base-1.json",
        NamedChain::BaseGoerli => "https://reference-data-directory.vercel.app/feeds-ethereum-testnet-goerli-base-1.json",
        NamedChain::Celo => "https://reference-data-directory.vercel.app/feeds-celo-mainnet.json",
        NamedChain::CeloAlfajores => "https://reference-data-directory.vercel.app/feeds-celo-testnet-alfajores.json",
        NamedChain::Scroll => "https://reference-data-directory.vercel.app/feeds-ethereum-mainnet-scroll-1.json",
        NamedChain::ScrollSepolia => "https://reference-data-directory.vercel.app/feeds-ethereum-testnet-sepolia-scroll-1.json",
        NamedChain::Linea => "https://reference-data-directory.vercel.app/feeds-ethereum-mainnet-linea-1.json",
        NamedChain::LineaTestnet => "https://reference-data-directory.vercel.app/feeds-ethereum-testnet-goerli-linea-1.json",
        NamedChain::ZkSync => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-mainnet-zksync-1.json",
        NamedChain::ZkSyncTestnet => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-testnet-goerli-zksync-1.json",
        NamedChain::PolygonZkEvm => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-mainnet-polygon-zkevm-1.json",
        NamedChain::PolygonZkEvmTestnet => "https://reference-data-directory-qy7u5hvya-chainlinklabs.vercel.app/feeds-ethereum-testnet-goerli-polygon-zkevm-1.json",            
        _ => "Nooope"
    };
    Some(url.to_string())
}

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

/// Chainlink reference-data-directory model
/// many fields are actually enums
/// check numeric values if are ok
/// study docs
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DataFeedIndex {
    compare_offchain: Option<String>,
    contract_address: Option<String>,
    contract_type: Option<String>,
    contract_version: u32,
    decimal_places: u8,
    ens: Option<String>,
    format_decimal_places: u8,
    health_price: Option<String>,
    heartbeat: u32,
    history: bool,
    multiply: Option<String>,
    name: Option<String>,
    pair: Vec<String>,
    path: Option<String>,
    proxy_address: Option<String>,
    threshold: f64,
    value_prefix: Option<String>,
    asset_name: Option<String>,
    feed_category: Option<String>,
    feed_type: Option<String>,
    docs: HashMap<String, String>,//DataFeedIndexDocs,
    decimals: u8
}

/// Main storage of data feeds oracles (NEEDS UPDATE)
/// now is hardcoded but requires a deep check, update and maintenance
/// adjust to reference found
pub struct DataFeeds {
    info: HashMap<Key, Oracle>,
}

impl DataFeeds {
    /// Returns a Datafeed loaded with chain specific indexes    
    pub async fn load_chain_feeds(chain: Chain) -> () {
        let url = get_ref_data_dir_url(chain.named().unwrap()).expect("Error getting url of feeds");
        println!("Populating information with {url}");
        let body = reqwest::get(url)
            .await.expect("Nope")
            .json::<serde_json::Value>()
            .await.expect("Noooope");
        //println!("{:?}", body);
        let res: Vec<DataFeedIndex> = serde_json::from_value(body).expect("Deserialization is still not possible"); 
        println!("{:?}", res);
        // deserialize data
    }

    /// get it for a chain -> then get the references values online (see above)
    /// Starts and loads the storage, if possible, find a better way (read a csv?)
    pub fn with_initial_values() -> Self {
        let mut info = HashMap::new();

        // Add pre-defined mappings
        info.insert(
            Key::from(Chain::mainnet(),"1INCH".to_string(),"ETH".to_string()),
            Oracle {
                address: "0x72AFAECF99C9d9C8215fF44C77B94B99C28741e8".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"1INCH".to_string(),"USD".to_string()),
            Oracle {
                address: "0xc929ad75B72593967DE83E7F7Cda0493458261D9".parse().unwrap(),
                deviation: 100, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"AAVE".to_string(),"ETH".to_string()),
            Oracle {
                address: "0x6Df09E975c830ECae5bd4eD9d90f3A95a4f88012".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"AAVE".to_string(),"USD".to_string()),

            Oracle {
                address: "0x547a514d5e3769680Ce22B2361c10Ea13619e8a9".parse().unwrap(),
                deviation: 100, 
                heartbeat: 3600, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"ALCX".to_string(),"ETH".to_string()),

            Oracle {
                address: "0x194a9AaF2e0b67c35915cD01101585A33Fe25CAa".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"AMPL".to_string(),"ETH".to_string()),

            Oracle {
                address: "0x492575FDD11a0fCf2C6C719867890a7648d526eB".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"AMPL".to_string(),"USD".to_string()),

            Oracle {
                address: "0xe20CA8D7546932360e37E9D72c1a47334af57706".parse().unwrap(),
                deviation: 100000, 
                heartbeat: 172800, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"ANKR".to_string(),"USD".to_string()),

            Oracle {
                address: "0x7eed379bf00005CfeD29feD4009669dE9Bcc21ce".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"APE".to_string(),"ETH".to_string()),

            Oracle {
                address: "0xc7de7f4d4C9c991fF62a07D18b3E31e349833A18".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"APE".to_string(),"USD".to_string()),

            Oracle {
                address: "0xD10aBbC76679a20055E167BB80A24ac851b37056".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"ARB".to_string(),"USD".to_string()),

            Oracle {
                address: "0x31697852a68433DbCc2Ff612c516d69E3D9bd08F".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"AUD".to_string(),"USD".to_string()),

            Oracle {
                address: "0x77F9710E7d0A19669A13c055F62cd80d313dF022".parse().unwrap(),
                deviation: 15, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"AVAX".to_string(),"USD".to_string()),
            
            Oracle {
                address: "0xFF3EEb22B5E3dE6e705b44749C2559d704923FD7".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"ARB".to_string(),"HEALTHCHECK".to_string()),

            Oracle {
                address: "0x32EaFC72772821936BCc9b8A32dC394fEFcDBfD9".parse().unwrap(),
                deviation: 100, 
                heartbeat: 86400, 
                decimals: 1 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"BADGER".to_string(),"ETH".to_string()),

            Oracle {
                address: "0x58921Ac140522867bf50b9E009599Da0CA4A2379".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );
        
        info.insert(
            Key::from(Chain::mainnet(),"BAL".to_string(),"ETH".to_string()),
        
            Oracle {
                address: "0xC1438AA3823A6Ba0C159CfA8D98dF5A994bA120b".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"BAL".to_string(),"USD".to_string()),

            Oracle {
                address: "0xdF2917806E30300537aEB49A7663062F4d1F2b5F".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"BAT".to_string(),"ETH".to_string()),

            Oracle {
                address: "0x0d16d4528239e9ee52fa531af613AcdB23D88c94".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"BNB".to_string(),"USD".to_string()),

            Oracle {
                address: "0x14e613AC84a31f709eadbdF89C6CC390fDc9540A".parse().unwrap(),
                deviation: 100, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"BTC".to_string(),"ETH".to_string()),

            Oracle {
                address: "0xdeb288F737066589598e9214E782fa5A8eD689e8".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 18 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"BTC".to_string(),"USD".to_string()),

            Oracle {
                address: "0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c".parse().unwrap(),
                deviation: 50, 
                heartbeat: 3600, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"C3M".to_string(),"EUR".to_string()),

            Oracle {
                address: "0xD41390267Afec3fA5b4c0B3aA6c706556CCE75ec".parse().unwrap(),
                deviation: 200, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(),"CAD".to_string(),"USD".to_string()),

            Oracle {
                address: "0xa34317DB73e77d453b1B8d04550c44D10e981C8e".parse().unwrap(),
                deviation: 15, 
                heartbeat: 86400, 
                decimals: 8 
            }
        );

        info.insert(
            Key::from(Chain::mainnet(), "ETH".to_string(), "USD".to_string()),

            Oracle {
                address: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".parse().unwrap(),
                deviation: 50, 
                heartbeat: 3600, 
                decimals: 8 
            }
        );

//------------------------
// continue?

// can i do an iterable from this?
// search for Key arguments? etc... 
// test capabilities and then consider to continue
// other hashmaps will be needed: proofofreserve, nftFloorPrice, etc




        DataFeeds { info }
    }
/* 
    pub fn insert_mapping(&mut self, key: Key, address: String) {
        self.map.insert(key, address);
    }
 */
    pub fn print_all_entries() -> () {
        // print all entries
        ()
    }
    
    /// Returns the oracle in storage if existent
    pub fn get_oracle(&self, chain: Chain, token: &str, base: &str) -> Option<&Oracle> {
        self.info.get(&Key { chain, token: token.into(), base: base.into() })
    }


}

#[cfg(test)]
mod tests {
    use super::*;

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

//    #[test] getting price?

}
