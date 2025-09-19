# warframe_market

This library is a wrapper for the [Warframe Market API](https://42bytes.notion.site/WFM-Api-v2-Documentation-5d987e4aa2f74b55a80db1a09932459d).

This library gets the response from the WFM API and deserializes it into the corresponding Rust type. The [Reqwest](https://crates.io/crates/reqwest) library is used to get the response from the WFM API, then [Serde](https://crates.io/crates/serde) is used to deserialize the [json](https://crates.io/crates/serde_json) into rust.

The intention of this library is to match the [official WFM API v2 Data Models](https://42bytes.notion.site/Data-Models-65e9ab01868c4dcca6ba499e68a04ac9#209845f8a3f24b6cad8597b667dca3dc) to Rust types.

Most of the struct fields are wrapped in an Option, as the WFM API responses are generic for many types of possible Warframe items. For instance, a mod can have a max rank, but a weapon has no use for this field.

No underlying rate limiter exists in this library. The WFM API states: "3 requests per second."

## Changelog
* Query parameters are now Option<&Hashmap<String, String>>
* Added endpoints:
    * auth/signin
    * auth/signout
    * me
    * order
    * order/close
    * orders/my
* Added enums:
    * Role
    * Status
    * Tier
    * Type
* Added structs:
    * CurrentUser
    * CurrentUserContainer
    * Transaction
    * TransactionItem
    * UserPrivate

## Examples
```
use warframe_market::MarketClient;

let client = MarketClient::new();

// Get all items. Returns `ItemShort`s with fewer fields than `Item`s.
let items = client.get_items().await.unwrap();
assert_eq!(items.len(), 3613);
assert_eq!(items[0].slug, "secura_dual_cestra".to_string());

// Get an individual item. Returns `Item` which can have more fields than `ItemShort`.
let item = client.get_item("secura_dual_cestra").await.unwrap();
assert_eq!(item.req_mastery_rank.unwrap(), 10);
assert_eq!(item.trading_tax.unwrap(), 2000);

// Now accepts None instead of an empty HashMap.
let archon = client.get_orders_item_top("archon_intensify", None).await.unwrap();
assert_eq!(archon.get("buy").unwrap()[0].order.rank, Some(10));

let query = HashMap::new();
query.insert("rank".to_string(), "1".to_string());
let queried_archon = client.get_orders_item_top("archon_intensify", Some(&query)).await.unwrap();
assert_eq!(queried_archon.get("buy").unwrap()[0].order.rank, Some(1));

// Now supports some calls to post endpoints (refer to changelog):
// Sign in.
let user = client.sign_in(<username>, <password>, "my_device_id").await.unwrap();    // Fill in your information

// Post a new order.
let post_order = PostOrder {
    item_id: "653833d6123a401b2563081d".to_string(),
    r#type: Type::Sell,
    platinum: 100,
    quantity: 1,
    ..Default::default()
};
let post_order = client.post_order(&post_order).await.unwrap();
```

## Known Issues
The sign out function is producing unexpected behavior. It returns an Ok status,
but the token remains valid for use. Still, the MarketClient replaces the auth 
token with None, preventing further usage of the token that was expected to be terminated.
