use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/exploring_fuel_and_sway-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId) {
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
        "./out/debug/exploring_fuel_and_sway.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
}

#[tokio::test]
async fn test_increment() {
    let (instance, _id) = get_contract_instance().await;
 
    // Increment the counter
    instance.methods().increment().call().await.unwrap();
 
    // Get the current value of the counter
    let result = instance.methods().count().call().await.unwrap();
 
    // Check that the current value of the counter is 1.
    // Recall that the initial value of the counter was 0.
    assert_eq!(result.value, 1);
}
