use fuels::prelude::*;

async fn setup_wallets() -> (WalletUnlocked, Wallet) {
    let mut wallet_1 = WalletUnlocked::new_random(None);
    let mut wallet_2 = WalletUnlocked::new_random(None).lock();

    let coins = setup_single_asset_coins(wallet_1.address(), BASE_ASSET_ID, 1, 5);

    let (client, _) = setup_test_client(coins, vec![], None, None, None).await;
    let provider = Provider::new(client);

    wallet_1.set_provider(provider.clone());
    wallet_2.set_provider(provider);

    (wallet_1, wallet_2)
}

#[tokio::main]
async fn main() -> Result<()> {
    let (wallet_1, wallet_2) = setup_wallets().await;

    let coins_before_transfer = wallet_1.get_coins(BASE_ASSET_ID).await?;

    wallet_1
        .transfer(
            wallet_2.address(),
            1,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await?;

    let coins_after_transfer = wallet_1.get_coins(BASE_ASSET_ID).await?;

    eprintln!("Coins before: {coins_before_transfer:?}");
    // Had only the one coin it was given in the setup
    assert_eq!(coins_before_transfer.len(), 1);

    eprintln!("Coins after: {coins_after_transfer:?}");
    // Should have the original coin but spent, and the tx change
    assert_eq!(coins_after_transfer.len(), 2);

    Ok(())
}
