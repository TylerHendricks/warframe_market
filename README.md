# warframe_market

This library is a wrapper for the [Warframe Market API](https://42bytes.notion.site/WFM-Api-v2-Documentation-5d987e4aa2f74b55a80db1a09932459d). **Currently only the v2 GET endpoints are supported**.

This library gets the response from the WFM API and deserializes it into the corresponding Rust type. The [Reqwest](https://crates.io/crates/reqwest) library is used to get the response from the WFM API, then [Serde](https://crates.io/crates/serde) is used to deserialize the [json](https://crates.io/crates/serde_json) into rust.

The intention of this library is to match the [official WFM API v2 Data Models](https://42bytes.notion.site/Data-Models-65e9ab01868c4dcca6ba499e68a04ac9#209845f8a3f24b6cad8597b667dca3dc) to Rust types. Additional functions may be added to each struct in the future, but the starting goal is simply to give users a starting point for accessing data gathered from the WFM API.

Most of the struct fields are wrapped in an Option, as the WFM API responses are generic for many types of possible Warframe items. For instance, a mod can have a max rank, but a weapon has no use for this field. I've found that the [official WFM API v2 Data Models](https://42bytes.notion.site/Data-Models-65e9ab01868c4dcca6ba499e68a04ac9#209845f8a3f24b6cad8597b667dca3dc) page actually downplays how many fields can be ommitted. I supsect more testing will show that I need to wrap even more fields in Options.

No underlying rate limiter exists in this library. The WFM API states: "3 requests per second, for now, probably i'll rise it later."

## Examples
```
use warframe_market::MarketClient;

let client = MarketClient::new();

let items = client.get_items().await.unwrap();  // Get all items. Returns ItemShorts with fewer fields than Items.
assert_eq!(items.len(), 3613);
assert_eq!(items[0].slug, "secura_dual_cestra".to_string());

let item = client.get_item("secura_dual_cestra").await.unwrap();    // Get an individual item. Returns Item which can have more fields.
assert_eq!(item.req_mastery_rank.unwrap(), 10);
assert_eq!(item.trading_tax.unwrap(), 2000);
```
