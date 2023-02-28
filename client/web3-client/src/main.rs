use std::env;
use ethers::providers::{Provider, Http};
use ethers::prelude::SignerMiddleware;
use std::convert::TryFrom;
use serde_json;
use serde::{Serialize, Deserialize};
use ethers::core::{abi::Abi, types::Address, types::U256, types::H256, types::Filter};
use ethers::contract::{Contract, EthEvent};
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::core::abi::{Detokenize, Token, InvalidOutputType, EventExt};
use std::fs;
use eth_keystore::encrypt_key;
use rand::RngCore;
use std::path::Path;

#[derive(Clone, Debug, EthEvent)]
struct Transfer {
    #[ethevent(indexed, name = "_from")]
    from: Address,
    #[ethevent(indexed, name = "_to")]
    to: Address,
    #[ethevent(name = "_tokens")]
    tokens: U256,
}

#[derive(Clone, Debug, EthEvent)]
struct Approval {
    #[ethevent(indexed, name = "_owner")]
    owner: Address,
    #[ethevent(indexed, name = "_spender")]
    spender: Address,
    #[ethevent(name = "_value")]
    value: U256,
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
    // Load compilation artifact and construct abi from it
    let contract_info = serde_json::from_str::<ContractInfo>(&fs::read_to_string("../../artifacts/contracts/AmogusToken.sol/AmogusToken.json").unwrap()).unwrap();
    let abi: Abi = contract_info.abi;
    let wallet: LocalWallet = env::var("GOERLI_PRIVATE_KEY").unwrap().parse().unwrap();
    let client = SignerMiddleware::new(provider, wallet);
    let contract = Contract::new(address, abi, client);

    // Call read only method
    let init_value: U256 = contract
        .method::<_, U256>("totalSupply", ()).unwrap()
        .call()
        .await.unwrap();
    println!("{}", init_value);

    // Request transfer events
    let logs: Vec<Transfer> = contract
        .event_for_name("Transfer").unwrap()
        .from_block(0u64)
        .query()
        .await.unwrap();
    println!("{:?}", logs);


    // Request transfer events but only for certain receiving address
    let other_addr = "0xab854be0a4d499b6fd8d0bb5f796ab5b33ce825b".parse::<Address>().unwrap();
    let event = &contract.abi().event("Transfer").unwrap();
    let logs: Vec<Transfer> = contract
        .event_with_filter(Filter::new().event(&event.abi_signature()).topic2(H256::from(other_addr)))
        .from_block(0u64)
        .query()
        .await.unwrap();
    println!("{:?}", logs);

    // Check approvals before call    
    let logs: Vec<Transfer> = contract
        .event_for_name("Approval").unwrap()
        .from_block(0u64)
        .query()
        .await.unwrap();
    println!("{:?}", logs);

    // Now lets try calling the some methods and checking the events
    let call = contract.method::<_, H256>("approve", (other_addr, U256::from(20))).unwrap();
    let pending_tx = call.send().await.unwrap();
    let receipt = pending_tx.confirmations(6).await.unwrap();
    println!("{:?}", receipt);

    // Check aprovals after call
    let logs: Vec<Transfer> = contract
        .event_for_name("Approval").unwrap()
        .from_block(0u64)
        .query()
        .await.unwrap();
    println!("{:?}", logs);

    Ok(())
}
