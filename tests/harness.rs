use fuels::test_helpers::{AssetConfig, WalletsConfig};
use fuels::prelude::*;
use std::str::FromStr;

// Load abi from json
abigen!(MyContract, "out/debug/sway_liquidity-abi.json");

pub fn base_asset() -> AssetId {
    AssetId::from_str("0x9ae5b658754e096e4d681c548daf46354495a437cc61492599e33fc64dcdc30c").unwrap()
}

async fn get_contract_instance() -> (MyContract, Bech32ContractId) {
    let asset_ids = [Default::default(), base_asset()];
    let asset_configs = asset_ids
        .map(|id| AssetConfig {
            id,
            num_coins: 1,
            coin_amount: 1_000_000,
        })
        .into_iter()
        .collect();

    let wallet_config = WalletsConfig::new_multiple_assets(1, asset_configs);
    let wallets = launch_custom_provider_and_get_wallets(wallet_config, None).await;
    let wallet = &wallets[0];

    let id = Contract::deploy(
        "./out/debug/sway_liquidity.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/sway_liquidity-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContractBuilder::new(id.to_string(), wallet.clone()).build();

    (instance, id)
}

#[tokio::test]
async fn can_get_contract_id() -> Result<(), Error> {
    let (contract_instance, contract_id) = get_contract_instance().await;

    let wallet = contract_instance._get_wallet();
    let deposit_amount = 1000;
    let call_params = CallParameters::new(Some(deposit_amount), Some(base_asset()), None);
    contract_instance
        .deposit(wallet.address().into())
        .call_params(call_params)
        .call()
        .await?;

    let lp_asset_id = AssetId::from(*contract_id.hash());
    let lp_token_balance = wallet.get_asset_balance(&lp_asset_id).await?;

    println!("{:?}", lp_token_balance);

    Ok(())
}
