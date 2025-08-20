use std::{collections::HashMap, thread, time::Duration};

use warframe_market::MarketClient;

#[tokio::test]
async fn test_manifests() {
    let client = MarketClient::new();

    client.get_versions().await.unwrap();

    client.get_items().await.unwrap();

    client.get_item("secura_dual_cestra").await.unwrap();

    thread::sleep(Duration::from_secs(1));

    client.get_item_set("nyx_prime_blueprint").await.unwrap();

    let riven_weapons = client.get_riven_weapons().await.unwrap();

    client
        .get_riven_weapon(&riven_weapons[0].slug)
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(1));

    client.get_riven_attributes().await.unwrap();

    let lich_weapons = client.get_lich_weapons().await.unwrap();
    client.get_lich_weapon(&lich_weapons[0].slug).await.unwrap();

    client.get_lich_ephemeras().await.unwrap();

    thread::sleep(Duration::from_secs(1));

    client.get_lich_quirks().await.unwrap();

    let sister_weapons = client.get_sister_weapons().await.unwrap();

    client
        .get_sister_weapon(&sister_weapons[0].slug)
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(1));

    client.get_sister_ephemeras().await.unwrap();

    client.get_sister_quirks().await.unwrap();

    client.get_locations().await.unwrap();

    thread::sleep(Duration::from_secs(1));

    client.get_npcs().await.unwrap();

    client.get_missions().await.unwrap();

    thread::sleep(Duration::from_secs(1));
}

#[tokio::test]
async fn test_achievements() {
    let client = MarketClient::new();

    client.get_achievements().await.unwrap();

    let query = HashMap::new();
    client
        .get_achievements_user("f0rgotten-hero", &query)
        .await
        .unwrap();

    client
        .get_achievements_user_id("607df8f1d07cd700aa07a72d", &query)
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(1));
}

#[tokio::test]
async fn test_dashboard() {
    let client = MarketClient::new();

    client.get_dashboard_showcase().await.unwrap();

    thread::sleep(Duration::from_secs(1));
}

#[tokio::test]
async fn test_orders() {
    let client = MarketClient::new();

    let orders_recent = client.get_orders_recent().await.unwrap();

    let order_id = &orders_recent[0].id;
    client.get_order(order_id).await.unwrap();

    client.get_orders_item("secura_dual_cestra").await.unwrap();

    thread::sleep(Duration::from_secs(1));

    let query = HashMap::new();
    client
        .get_orders_item_top("secura_dual_cestra", &query)
        .await
        .unwrap();

    client.get_orders_user("f0rgotten-hero").await.unwrap();

    client
        .get_orders_user_id("607df8f1d07cd700aa07a72d")
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(1));
}

#[tokio::test]
async fn test_users() {
    let client = MarketClient::new();

    client.get_user("F0RGOTTEN_HERO").await.unwrap();

    thread::sleep(Duration::from_secs(1));
}
