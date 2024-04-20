use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "TickTockContract",
    abi = "out/debug/sample-contract-abi.json"
));

async fn get_contract_instance_and_wallet() -> (TickTockContract<WalletUnlocked>, ContractId, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/sample-contract.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = TickTockContract::new(id.clone(), wallet.clone());

    (instance, id.into(), wallet)
}

#[tokio::test]
async fn test_tick() {
    let (instance, _id, _wallet) = get_contract_instance_and_wallet().await;
    let before = instance.methods().count_ticks().call().await.unwrap();
    instance.methods().tick().call().await.unwrap();
    let after = instance.methods().count_ticks().call().await.unwrap();
    assert_eq!(before.value + 1, after.value);
}

#[tokio::test]
async fn test_tock() {
    let (instance, _id, _wallet) = get_contract_instance_and_wallet().await;
    let before = instance.methods().count_tocks().call().await.unwrap();
    instance.methods().tock().call().await.unwrap();
    let after = instance.methods().count_tocks().call().await.unwrap();
    assert_eq!(before.value + 1, after.value);
}

#[tokio::test]
async fn test_ticktock() {
    let (instance, _id, _wallet) = get_contract_instance_and_wallet().await;
    let before_ticks = instance.methods().count_ticks().call().await.unwrap();
    let before_tocks = instance.methods().count_tocks().call().await.unwrap();
    instance.methods().ticktock().call().await.unwrap();
    let after_ticks = instance.methods().count_ticks().call().await.unwrap();
    let after_tocks = instance.methods().count_tocks().call().await.unwrap();
    assert_eq!(before_ticks.value + 1, after_ticks.value);
    assert_eq!(before_tocks.value + 1, after_tocks.value);
}
