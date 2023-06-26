use ethers::prelude::*;
use ethers::signers::Wallet;
use ethers::types::{Address};
use std::{error::Error, str::FromStr, sync::Arc};

const CONTRACT_ADDRESS: &str = "";
const PRIVATE_KEY: &str = "0xebd24eb9345e93582be65937c87932f8b7e97c80c4d51e83b1e2bdc33c4acc42";
#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {

    let result = deploy().await;
    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("call error:");
            Err(e)
        }
    }
}

async fn call() -> Result<(), Box<dyn Error>> {
    let provider: Provider<Http> = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    let wallet: LocalWallet = Wallet::from_str(PRIVATE_KEY)?;

    let signer = Arc::new(SignerMiddleware::new(
        provider.clone(),
        wallet.with_chain_id(53077 as u64),
    ));

    abigen!(TestZkEVM, "./resource/abi/TestZkEVM.json");

    let testZkEVM: TestZkEVM<SignerMiddleware<Provider<Http>, _>> =
        TestZkEVM::new(Address::from_str(CONTRACT_ADDRESS)?, signer);

    let tx = testZkEVM.transfer(
        Address::from_str("0xa210b31C70737AA2E09A0fFC151CF21e18365954").unwrap(),
        10.into(),
    );
    let receipt = tx.send().await;
    match receipt {
        Ok(sent_tx) => println!("====transaction ID: {:?}", sent_tx),
        Err(e) => println!("call exception: {:?}", e),
    }
    Ok(())
}

async fn deploy() -> Result<(), Box<dyn Error>> {
    let provider: Provider<Http> = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    let wallet: LocalWallet = Wallet::from_str(PRIVATE_KEY)?;

    let signer = Arc::new(SignerMiddleware::new(
        provider.clone(),
        wallet.with_chain_id(53077 as u64),
    ));

    abigen!(TestZkEVM, "./resource/abi/TestZkEVM.json");
    let a: u64 = 10;
    let testZkEVM = TestZkEVM::deploy(signer, a.pow(18))
        .unwrap()
        .send()
        .await;

    match testZkEVM {
        Ok(sent_tx) => println!("====testZkEVM: {:?}", sent_tx),
        Err(e) => println!("call exception: {:?}", e),
    }

    Ok(())
}
