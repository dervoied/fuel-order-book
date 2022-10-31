use fuels::{
    prelude::{abigen, TxParameters},
    tx::{AssetId, Bytes32, Input, Output, Transaction, Word},
};
abigen!(
    LimitOrderStruct,
    "packages/contracts/order-settle-contract/out/debug/order-settle-contract-abi.json"
);
const MIN_GAS: u64 = 100_000;
const TAKE_ORDER_SCRIPT_BINARY: &str = "../order-script/out/debug/order-script.bin";
pub async fn get_take_order_script() -> Vec<u8> {
    let script_bytecode = std::fs::read(TAKE_ORDER_SCRIPT_BINARY).unwrap();
    script_bytecode
}

pub async fn build_take_order_tx(
    order: LimitOrder,
    gas_coin: Input,
    predicate_coins_input: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
    params: TxParameters,
) -> Transaction {
    let script_bytecode = get_take_order_script().await;

    // build the tx inputs
    let tx_inputs: Vec<Input> = Vec::new();
    tx_inputs.push(predicate_coins_input);
    tx_inputs.push(gas_coin);
    tx_inputs.append(&mut optional_inputs.to_vec());

    // build the tx outputs
    let tx_outputs: Vec<Output> = Vec::new();
    // enforce that the output of the transaction
    // means the maker gets paid the correct amount
    // the predicate will enforce this.
    tx_outputs.push(Output::Coin {
        to: order.maker,
        amount: order.maker_amount,
        asset_id: AssetId::from(order.maker_token),
    });

    Transaction::Script {
        gas_price: params.gas_price,
        gas_limit: MIN_GAS,
        maturity: params.maturity,
        receipts_root: Default::default(),
        script: script_bytecode,
        script_data: vec![],
        inputs: tx_inputs,
        outputs: tx_outputs,
        witnesses: (),
        metadata: (),
    }
}
