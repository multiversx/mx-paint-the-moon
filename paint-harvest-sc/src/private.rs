// use common::Color;
use super::Color;
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait PrivateModule:
    crate::storage::StorageModule + crate::owner::OwnerModule + crate::custom_callbacks::CustomCallbacks
{
    fn refresh_accumulated_rewards(&self, user: &ManagedAddress, now: u64) {
        let user_info = self.user_info(user).get();

        // calc rewards for now - user_info.start_timestamp for current harvest color
        let accumulated_rewards = self.calculate_rewards(
            &user_info.current_harvest_color,
            user_info.start_timestamp,
            now,
        );
        self.accumulated_resources(&user_info.current_harvest_color, user)
            .update(|val| *val += accumulated_rewards);

        // maybe also mint accumulated tokens and leave them on the SC
        self.add_quantity(
            user_info.current_harvest_color as u64,
            &BigUint::from(accumulated_rewards),
        );
    }

    fn calculate_rewards(&self, color: &Color, start: u64, now: u64) -> u64 {
        let harvest_duration = self.harvest_duration(color).get();
        let time_since_start = now - start;
        time_since_start / harvest_duration
    }

    fn require_contract_open(&self) {
        require!(self.is_open().get(), "maintenance")
    }
}
