use coinbase::Coinbase;
use dotenv::dotenv;
use rust_decimal::prelude::Decimal;
use std::env;

#[tokio::test]
async fn get_account() {
    let exchange = init();
    let resp = exchange.get_account().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init();
    let resp = exchange.get_all_open_orders().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_orders() {
    let exchange = init();
    let resp = exchange.get_all_orders(None).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_orders_for_a_given_product() {
    let exchange = init();
    let resp = exchange.get_all_orders(Some("ETH-BTC")).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn order_status() {
    let exchange = init();
    let order = exchange
        .market_buy("BTC-USD", Decimal::new(1, 3))
        .await
        .unwrap();

    let resp = exchange.order_status(order.id).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy() {
    let exchange = init();
    let resp = exchange
        .limit_buy("BTC-USD", Decimal::new(1, 3), Decimal::new(5000, 0))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init();
    let resp = exchange
        .limit_sell("BTC-USD", Decimal::new(1, 3), Decimal::new(20000, 0))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init();
    let resp = exchange
        .market_buy("BTC-USD", Decimal::new(1, 3))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init();
    let resp = exchange
        .market_sell("BTC-USD", Decimal::new(1, 3))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_all_orders() {
    let exchange = init();
    exchange
        .limit_sell("BTC-USD", Decimal::new(1, 3), Decimal::new(20000, 0))
        .await
        .unwrap();
    exchange
        .limit_sell("BTC-USD", Decimal::new(1, 3), Decimal::new(20000, 0))
        .await
        .unwrap();

    exchange
        .limit_buy("ETH-BTC", Decimal::new(2, 2), Decimal::new(2, 2))
        .await
        .unwrap();

    let resp = exchange.cancel_all_orders(Some("BTC-USD")).await.unwrap();

    println!("{:?}", resp);

    let resp = exchange.cancel_all_orders(None).await.unwrap();

    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init();
    let order = exchange
        .limit_sell("BTC-USD", Decimal::new(1, 3), Decimal::new(20000, 0))
        .await
        .unwrap();
    let resp = exchange
        .cancel_order(order.id, Some("BTC-USD"))
        .await
        .unwrap();

    println!("{:?}", resp);
}

#[tokio::test]
async fn get_fills_for_order() {
    let exchange = init();
    let order = exchange
        .limit_sell("BTC-USD", Decimal::new(1, 3), Decimal::new(20000, 0))
        .await
        .unwrap();
    let resp = exchange
        .get_fills_for_order(order.id.as_ref())
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_fills_for_product() {
    let exchange = init();
    let resp = exchange.get_fills_for_product("BTC-USD").await.unwrap();
    println!("{:?}", resp);
}

fn init() -> Coinbase {
    dotenv().ok();
    Coinbase::with_credential(
        &env::var("COINBASE_API_KEY").unwrap(),
        &env::var("COINBASE_API_SECRET").unwrap(),
        &env::var("COINBASE_PASSPHRASE").unwrap(),
        true,
    )
}
