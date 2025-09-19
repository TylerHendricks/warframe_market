use crate::{
    MarketClient,
    lich::{lich_ephemera::LichEphemera, lich_quirks::LichQuirk, lich_weapon::LichWeapon},
    market_error::MarketError,
    misc::{
        item::{Item, ItemShort},
        item_set::ItemSet,
        location::Location,
        mission::Mission,
        npc::Npc,
        versions::Versions,
    },
    riven::{riven_attribute::RivenAttribute, riven_item::Riven},
    sister::{
        sister_ephemera::SisterEphemera, sister_quirk::SisterQuirk, sister_weapon::SisterWeapon,
    },
};

impl MarketClient {
    /// This endpoint retrieves the current version number of the server's
    /// resources, formatted either as a semVer string or as an arbitrary
    /// version identifier.
    ///
    /// Whenever the server database is updated or new versions of mobile apps
    /// are released, the version number for relevant resources is also
    /// updated.
    ///
    /// Client applications can check this endpoint periodically to fetch the
    /// current server
    /// version. A discrepancy between the server's version number and the
    /// client's indicates that an update has occurred. In such cases, clients
    /// should refresh their local data, like re-downloading item Vecs, to stay
    /// synchronized with the server's latest updates.
    pub async fn get_versions(&self) -> Result<Versions, MarketError> {
        self.get("versions", None, false).await
    }

    /// Get Vec of all tradable items
    pub async fn get_items(&self) -> Result<Vec<ItemShort>, MarketError> {
        self.get("items", None, false).await
    }

    /// Get full info about one, particular item
    pub async fn get_item(&self, slug: &str) -> Result<Item, MarketError> {
        self.get(&format!("items/{slug}"), None, false).await
    }

    /// Retrieve Information on Item Sets
    ///
    /// In WFM, items can either be standalone or part of a set. A set is a
    /// collection of related items that are often traded together.
    ///
    /// If the queried item is not part of any set, the response will contain
    /// an array with just that one item.
    ///
    /// If the item is part of a set or is a set itself, the response will
    /// include an array of all items within that set.
    pub async fn get_item_set(&self, slug: &str) -> Result<Vec<Item>, MarketError> {
        let data: ItemSet = self.get(&format!("item/{slug}/set"), None, false).await?;
        Ok(data.items)
    }

    /// Get Vec of all tradable riven items.
    pub async fn get_riven_weapons(&self) -> Result<Vec<Riven>, MarketError> {
        self.get("riven/weapons", None, false).await
    }

    /// Get full info about one, particular riven item.
    pub async fn get_riven_weapon(&self, slug: &str) -> Result<Riven, MarketError> {
        self.get(&format!("riven/weapon/{slug}"), None, false).await
    }

    /// Get Vec of all attributes for riven weapons.
    pub async fn get_riven_attributes(&self) -> Result<Vec<RivenAttribute>, MarketError> {
        self.get("riven/attributes", None, false).await
    }

    /// Get Vec of all tradable lich weapons.
    pub async fn get_lich_weapons(&self) -> Result<Vec<LichWeapon>, MarketError> {
        self.get("lich/weapons", None, false).await
    }

    /// Get full info about one, particular lich weapon.
    pub async fn get_lich_weapon(&self, slug: &str) -> Result<LichWeapon, MarketError> {
        self.get(&format!("lich/weapon/{slug}"), None, false).await
    }

    /// Get Vec of all tradable lich ephemeras.
    pub async fn get_lich_ephemeras(&self) -> Result<Vec<LichEphemera>, MarketError> {
        self.get("lich/ephemeras", None, false).await
    }

    /// Get Vec of all tradable lich quirks.
    pub async fn get_lich_quirks(&self) -> Result<Vec<LichQuirk>, MarketError> {
        self.get("lich/quirks", None, false).await
    }

    /// Get Vec of all tradable sister weapons.
    pub async fn get_sister_weapons(&self) -> Result<Vec<SisterWeapon>, MarketError> {
        self.get("sister/weapons", None, false).await
    }

    /// Get full info about one, particular sister weapon.
    pub async fn get_sister_weapon(&self, slug: &str) -> Result<SisterWeapon, MarketError> {
        self.get(&format!("sister/weapon/{slug}"), None, false)
            .await
    }

    /// Get Vec of all tradable sister ephemeras.
    pub async fn get_sister_ephemeras(&self) -> Result<Vec<SisterEphemera>, MarketError> {
        self.get("sister/ephemeras", None, false).await
    }

    /// Get Vec of all tradable sister quirks.
    pub async fn get_sister_quirks(&self) -> Result<Vec<SisterQuirk>, MarketError> {
        self.get("sister/quirks", None, false).await
    }

    /// Get Vec of all locations.
    pub async fn get_locations(&self) -> Result<Vec<Location>, MarketError> {
        self.get("locations", None, false).await
    }

    /// Get Vec of all NPCs.
    pub async fn get_npcs(&self) -> Result<Vec<Npc>, MarketError> {
        self.get("npcs", None, false).await
    }

    /// Get Vec of all Missions.
    pub async fn get_missions(&self) -> Result<Vec<Mission>, MarketError> {
        self.get("missions", None, false).await
    }
}
