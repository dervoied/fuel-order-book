use fuels::tx::{Address, Contract};

mod utils {
    pub mod builder;
    pub mod environment;
}

const PREDICATE: &str = "../order-predicate/out/debug/order-predicate.bin";
/// Gets the message to contract predicate
pub fn get_predicate() -> (Vec<u8>, Address) {
    let predicate_bytecode = std::fs::read(PREDICATE).unwrap();
    let predicate_root = Address::from(*Contract::root_from_code(&predicate_bytecode));
    (predicate_bytecode, predicate_root)
}
mod success {
    // abigen!(
    //     Order,
    //     "packages/contracts/order-settle-contract/out/debug/order-settle-contract-abi.json"
    // );

    use crate::{get_predicate, utils::builder::LimitOrder};
    use fuels::{
        contract::predicate::Predicate,
        prelude::{Bits256, Token, Tokenizable, TxParameters},
        test_helpers::DEFAULT_COIN_AMOUNT,
        tx::{AssetId, Input, TxPointer, UtxoId},
    };

    use crate::{utils::environment as env, PREDICATE};

    #[tokio::test]
    async fn test_limit_order_predicate() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        // might need to init another coin to correctly simulate the make/take
        let (maker, taker, coin_inputs, provider) = env::setup_environment(coin).await;
        let predicate = Predicate::load_from(PREDICATE).unwrap();
        let (predicate_bytecode, predicate_root) = get_predicate();
        // create the order (fund the predicate)
        let (_tx, _rec) = maker
            .transfer(predicate.address(), coin.0, coin.1, TxParameters::default())
            .await
            .unwrap();
        // get the utx_
        let predicate_coin = &provider
            .get_coins(&predicate_root.into(), AssetId::default())
            .await
            .unwrap()[0];

        let balance = maker.get_asset_balance(&coin.1).await.unwrap();
        let predicate_balance = provider
            .get_asset_balance(predicate.address(), coin.1)
            .await
            .unwrap();
        assert!(balance == 0);
        assert!(predicate_balance == coin.0);

        // this is the way i should do this, check the script. probably need to add some
        // inputs to this
        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: coin.0,
            taker_amount: coin.0,
            // this seems jank
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            salt: 42,
        };
        let predicate_coin_input = Input::CoinPredicate {
            utxo_id: UtxoId::from(predicate_coin.utxo_id.clone()),
            owner: predicate_root,
            amount: coin.0,
            asset_id: coin.1,
            tx_pointer: TxPointer::default(),
            maturity: 0,
            predicate: predicate_bytecode,
            predicate_data: vec![],
        };
        let _receipt = env::take_order(
            order,
            &taker,
            coin_inputs[0].clone(),
            predicate_coin_input,
            &vec![],
            &vec![],
        )
        .await;
        //
        //
        //
        //
        //
        //  below here is drafting, doesnt actually work, above we use a script

        // spend the predicate with taker
        // let make_coin: [u8; 32] = coin.1.into();
        // let take_coin: [u8; 32] = coin.1.into();
        // // currently cant pass any friendly data in the predicate
        // // data, so will just pass a byte array
        // let order = LimitOrder {
        //     maker_token: Bits256(make_coin),
        //     taker_token: Bits256(take_coin),
        //     maker_amount: coin.0,
        //     taker_amount: coin.0,
        //     // taker_token_fee: 0,
        //     // maker: maker,
        //     // taker: taker,
        //     // sender
        //     salt: 42,
        // };

        // // major hackage here to get this to work with poor support for predicates
        // // in the SDK
        // let mut predicate_data = vec![];
        // predicate_data.push(make_coin.to_vec());
        // predicate_data.push(take_coin.to_vec());
        // let maker_amount: [u8; 8] = coin.0.to_be_bytes();
        // let taker_amount: [u8; 8] = coin.0.to_be_bytes();
        // predicate_data.push(maker_amount.to_vec());
        // predicate_data.push(taker_amount.to_vec());

        // let raw_arr = predicate_data.into_iter().flatten().collect();
        // taker
        //     .receive_from_predicate(
        //         predicate.address(),
        //         predicate.code(),
        //         coin.0,
        //         coin.1,
        //         Some(raw_arr),
        //         TxParameters::default(),
        //     )
        //     .await
        //     .unwrap();

        // // test these balances
        // let maker = maker.get_asset_balance(&coin.1).await.unwrap();
        // let taker = taker.get_asset_balance(&coin.1).await.unwrap();
        // println!("{:?}\n{:?}", maker, taker)
    }
}

mod fail {

    #[tokio::test]
    async fn test_limit_order_predicate_wrong_take_coin() {}
}
