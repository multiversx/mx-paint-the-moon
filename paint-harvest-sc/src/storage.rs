use multiversx_sc::imports::*;

use crate::data::{Color, UserInfo};

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("isOpen")]
    fn is_open(&self) -> SingleValueMapper<bool>;

    // valid nfts for receiving rewards
    #[storage_mapper("collectionTokenId")]
    fn collection_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("paintTokenId")]
    fn paint_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // if empty on claim => error, not registered
    // if not empty on start harvest => error, one nft harvest per user
    // each user stakes one nonce from the collection
    #[storage_mapper("userInfo")]
    fn user_info(&self, user: &ManagedAddress) -> SingleValueMapper<UserInfo>;

    #[storage_mapper("colorsHarvested")]
    fn colors_harvested(&self, user: &ManagedAddress) -> UnorderedSetMapper<Color>;

    // each user stakes one nonce from the collection
    // #[storage_mapper("currentHarvestColor")]
    // fn current_harvest_color(&self, user: &ManagedAddress) -> SingleValueMapper<Color>;

    // how much time it takes to produce one unit of a specific color
    #[storage_mapper("harvestDuration")]
    fn harvest_duration(&self, color: &Color) -> SingleValueMapper<u64>;

    #[storage_mapper("accumulatedResources")]
    fn accumulated_resources(&self, color: &Color, user: &ManagedAddress)
        -> SingleValueMapper<u64>;
}
