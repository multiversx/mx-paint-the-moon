use crate::data::Color;

#[multiversx_sc::module]
pub trait EventsModule {
    #[event]
    fn changed_harvest_color(&self, #[indexed] user: &ManagedAddress, #[indexed] new_color: &Color);

    #[event]
    fn claimed_harvest(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] color: &Color,
        amount: &BigUint,
    );

    #[event]
    fn started_harvest(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] nft_nonce: u64,
        #[indexed] start_timestamp: u64,
        #[indexed] starting_color: &Color,
    );

    #[event]
    fn stopped_harvest(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] nft_nonce: u64,
        #[indexed] end_timestamp: u64,
    );
}
