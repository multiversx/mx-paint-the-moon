#![no_std]
use multiversx_sc::imports::*;
use paint_the_moon_sc::Color;

mod custom_callbacks;
mod data;
mod events;
mod owner;
mod private;
mod storage;

pub use data::*;

/// Paint harvesting contract used for receiving paint once in a while.
#[multiversx_sc::contract]
pub trait PaintHarvestSc:
    owner::OwnerModule
    + storage::StorageModule
    + custom_callbacks::CustomCallbacks
    + private::PrivateModule
    + events::EventsModule
{
    #[init]
    fn init(&self, collection_token_id: TokenIdentifier, is_open: bool) {
        self.collection_token_id().set(collection_token_id);
        self.is_open().set(is_open);
        //setup only collection token identifier in init
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint]
    fn start_harvest(&self, color: Color) {
        // check if open
        self.require_contract_open();

        // check payment token id
        let payment = self.call_value().single_esdt();
        require!(
            payment.token_identifier == self.collection_token_id().get(),
            "Wrong NFT sent"
        );

        // check if user info is empty
        let caller = self.blockchain().get_caller();
        require!(
            self.user_info(&caller).is_empty(),
            "User already registered for harvesting"
        );

        // start harvest for the color
        // fill storage info
        let now = self.blockchain().get_block_timestamp();
        let user_info = UserInfo {
            nft_nonce: payment.token_nonce,
            current_harvest_color: color,
            start_timestamp: now,
        };
        self.user_info(&caller).set(user_info);

        // event
        self.started_harvest(&caller, payment.token_nonce, now, &color);
    }

    #[endpoint]
    fn claim_harvest(&self) {
        // check if open
        self.require_contract_open();

        // check if user info is not empty
        let caller = self.blockchain().get_caller();
        require!(
            !self.user_info(&caller).is_empty(),
            "User not registered for harvest"
        );

        // mint new paint according to rewards + accumulated rewards
        let now = self.blockchain().get_block_timestamp();
        // let user_info = self.user_info(&caller).get();
        self.refresh_accumulated_rewards(&caller, now);

        // claim rewards + accumulated rewards
        let mut all_rewards = ManagedVec::new();
        let paint_token_id = self.paint_token_id().get();

        for color in self.colors_harvested(&caller).iter() {
            let accumulated_rewards = self.accumulated_resources(&color, &caller).get();
            all_rewards.push(EsdtTokenPayment::new(
                paint_token_id.clone(),
                color as u64,
                BigUint::from(accumulated_rewards),
            ));

            // event
            self.claimed_harvest(&caller, &color, &BigUint::from(accumulated_rewards));

            // clear
            self.accumulated_resources(&color, &caller).clear()
        }

        // send rewards
        require!(!all_rewards.is_empty(), "Nothing to claim");
        self.tx().to(&caller).multi_esdt(all_rewards).transfer();

        // cleanup storage
        self.colors_harvested(&caller).clear();
    }

    #[endpoint]
    fn change_harvest_color(&self, new_color: Color) {
        // check if open
        self.require_contract_open();

        // check if user info is not empty
        let caller = self.blockchain().get_caller();
        require!(
            !self.user_info(&caller).is_empty(),
            "User not registered for harvest"
        );

        // refresh accumulated rewards
        let now = self.blockchain().get_block_timestamp();
        let user_info = self.user_info(&caller).get();
        self.refresh_accumulated_rewards(&caller, now);

        // change storage
        self.colors_harvested(&caller)
            .insert(user_info.current_harvest_color);
        self.user_info(&caller).update(|info| {
            info.current_harvest_color = new_color;
            info.start_timestamp = now;
        });

        // event
        self.changed_harvest_color(&caller, &new_color);
    }

    #[endpoint]
    fn stop_harvest(&self) {
        // check if open
        self.require_contract_open();

        // check if user info is not empty
        let caller = self.blockchain().get_caller();
        require!(!self.user_info(&caller).is_empty(), "User not registered");

        // refresh accumulated rewards
        let now = self.blockchain().get_block_timestamp();
        let user_info = self.user_info(&caller).get();
        self.refresh_accumulated_rewards(&caller, now);

        // claim rewards + accumulated rewards
        let mut all_rewards = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();
        let paint_token_id = self.paint_token_id().get();

        for color in self.colors_harvested(&caller).iter() {
            let accumulated_rewards = self.accumulated_resources(&color, &caller).get();
            all_rewards.push(EsdtTokenPayment::new(
                paint_token_id.clone(),
                color as u64,
                BigUint::from(accumulated_rewards),
            ));

            // event
            self.claimed_harvest(&caller, &color, &BigUint::from(accumulated_rewards));

            // clear
            self.accumulated_resources(&color, &caller).clear()
        }

        if !all_rewards.is_empty() {
            self.tx().to(&caller).multi_esdt(all_rewards).transfer();
        }

        // send back nft
        self.tx()
            .to(&caller)
            .single_esdt(
                &self.collection_token_id().get(),
                user_info.nft_nonce,
                &BigUint::from(1u64),
            )
            .transfer();

        // cleanup storage
        self.colors_harvested(&caller).clear();
        self.user_info(&caller).clear();

        // event
        self.stopped_harvest(&caller, user_info.nft_nonce, now);
    }
}
