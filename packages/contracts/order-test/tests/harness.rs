use fuels::tx::{Address, Contract};

mod utils {
    pub mod builder;
    pub mod environment;
}

const PREDICATE: &str = "../order-predicate/out/debug/order-predicate.bin";
/// Gets the message to contract predicate
pub async fn get_contract_message_predicate() -> (Vec<u8>, Address) {
    let predicate_bytecode = std::fs::read(PREDICATE).unwrap();
    let predicate_root = Address::from(*Contract::root_from_code(&predicate_bytecode));
    (predicate_bytecode, predicate_root)
}
mod success {
    use fuels::{
        contract::predicate::Predicate,
        prelude::{Bech32Address, TxParameters},
        test_helpers::DEFAULT_COIN_AMOUNT,
        tx::AssetId,
    };

    use crate::{get_contract_message_predicate, utils::environment as env, PREDICATE};

    #[tokio::test]
    async fn test_limit_order_predicate() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        // might need to init another coin to correctly simulate the make/take
        let (maker, taker, coin_inputs, provider) = env::setup_environment(coin).await;

        let predicate = Predicate::load_from(PREDICATE).unwrap();

        // let (_bytecode, predicate_root) = get_contract_message_predicate().await;
        // let (tx, receipt) =
        //     env::send_coins_to_predicate(&maker, predicate.address(), coin.0, coin.1).await;

        let (tx, rec) = maker
            .transfer(predicate.address(), coin.0, coin.1, TxParameters::default())
            .await
            .unwrap();

        let balance = maker.get_asset_balance(&coin.1).await.unwrap();
        let predicate_balance = provider
            .get_asset_balance(predicate.address(), coin.1)
            .await
            .unwrap();
        assert!(balance == 0);
        assert!(predicate_balance == coin.0);

        // spend the predicate with taker
        let order = LimitOrder {
             
        };
        let mut predicate_data = Vec::new();
        predicate_data.push(1);
        taker
            .spend_predicate(
                predicate.address(),
                predicate.code(),
                coin.0,
                coin.1,
                taker.address(),
                Some(predicate_data),
                TxParameters::default(),
            )
            .await
            .unwrap();
    }
}

mod fail {

    #[tokio::test]
    async fn test_limit_order_predicate_wrong_take_coin() {}
}
