use fuels::types::errors::Error;
use fuels::types::Bytes32;
use fuels::types::transaction::Transaction;
use fuels::programs::contract::ContractCallHandler;
use fuels::prelude::{
    abigen,
    Contract,
    LoadConfiguration,
    TransactionType,
    ScriptTransaction,
    Provider,
    WalletUnlocked,
    BuildableTransaction,
    TxPolicies,
    TransactionBuilder,
    Account,
    WalletsConfig,
    launch_custom_provider_and_get_wallets,
};

// Load abi from json
abigen!(Contract(
    name = "TickTockContract",
    abi = "sample-contract/out/debug/sample-contract-abi.json"
));

#[tokio::main]
async fn main() {
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(Some(1), Some(2), Some(100)),
        None,
        None,
    )
    .await
    .unwrap();

    let wallet_a = wallets.pop().unwrap();
    let provider = wallet_a.provider().unwrap();

    let binary_filepath = "./sample-contract/out/debug/sample-contract.bin";
    let contract = Contract::load_from(
        binary_filepath,
        LoadConfiguration::default(),
    ).unwrap();

    // Deploy the contract
    let contract_id = contract.deploy(&wallet_a, TxPolicies::default()).await.unwrap();

    let instance = TickTockContract::new(contract_id.clone(), wallet_a.clone());

    // Call tick() to update the tick counter
    let before_ticks = instance.methods().count_ticks().simulate().await.unwrap();

    let tx = build_contract_tx(&wallet_a, instance.methods().tick()).await.unwrap();
    let txid = provider.send_transaction(tx.clone()).await.unwrap();

    println!("tx inputs: {:?}\n", &tx.inputs());

    let after_ticks = instance.methods().count_ticks().simulate().await.unwrap();

    // Show that the transaction was executed as expected since the counter incremented
    assert_eq!(before_ticks.value + 1, after_ticks.value);

    let fetched_tx = provider.get_transaction_by_id(&txid).await.unwrap().unwrap().transaction;
    match fetched_tx {
        TransactionType::Script(c) => {
            println!("fetched tx inputs: {:?} \n", c.inputs());
        },
        _ => {}
    }
}


pub async fn build_contract_tx(
    wallet: &WalletUnlocked,
    call_handler: ContractCallHandler<WalletUnlocked, ()>,
) -> Result<ScriptTransaction, Error> {
    let provider = wallet.provider().unwrap();
    let mut stb = call_handler.transaction_builder().await.unwrap();
    wallet.adjust_for_fee(&mut stb, 0).await.unwrap();
    let _ = stb.add_signer(wallet.clone());
    stb.build(&provider).await
}

pub async fn fetch_txs(
    provider: &Provider,
    tx_hashes: Vec<Bytes32>,
) -> Vec<TransactionType> {
    let mut txs = vec![];
    for tx_hash in tx_hashes {
        let tx = provider.get_transaction_by_id(&tx_hash).await.unwrap().unwrap().transaction;
        txs.push(tx);
    }
    txs
}
