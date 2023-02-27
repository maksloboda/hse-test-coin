use std::env;
use ethers::providers::{Provider, Http, Middleware};
use std::convert::TryFrom;
use serde_json;
use serde::{Serialize, Deserialize};
use ethers::core::{abi::Abi, types::Address, types::U256, types::H256, types::Filter};
use ethers::contract::{Contract, EthEvent};
use ethers::signers::Wallet;
use ethers::core::abi::{Detokenize, Token, InvalidOutputType};
use std::fs;

#[derive(Clone, Debug, EthEvent)]
struct Transfer {
    #[ethevent(indexed, name = "_from")]
    from: Address,
    #[ethevent(indexed, name = "_to")]
    to: Address,
    #[ethevent(name = "_tokens")]
    tokens: U256,
}

#[derive(Serialize, Deserialize)]
struct ContractInfo {
    abi: Abi
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();
    let provider = Provider::<Http>::try_from(
        &env::var("INFURA_HTTPS").unwrap()
    ).unwrap();

    let &address = &env::var("CONTRACT_ADDRESS").unwrap().parse::<Address>().unwrap();

    let contract_info = serde_json::from_str::<ContractInfo>(&fs::read_to_string("../../artifacts/contracts/AmogusToken.sol/AmogusToken.json").unwrap()).unwrap();

    let abi: Abi = contract_info.abi;

    let contract = Contract::new(address, abi, provider);

    let init_value: U256 = contract
        .method::<_, U256>("totalSupply", ()).unwrap()
        .call()
        .await.unwrap();

    println!("{}", init_value);

    let logs: Vec<Transfer> = contract
        .event_for_name("Transfer").unwrap()
        .from_block(0u64)
        .query()
        .await.unwrap();

    println!("{:?}", logs);

    // let block = provider.get_block(100u64).await.unwrap();
    // println!("Got block: {}", serde_json::to_string(&block).unwrap());
    
    // let addr = "0x89d24a6b4ccb1b6faa2625fe562bdd9a23260359".parse::<Address>().unwrap();
    // let code = provider.get_code(addr, None).await.unwrap();
    // println!("Got code: {}", serde_json::to_string(&code).unwrap());
    Ok(())
}
