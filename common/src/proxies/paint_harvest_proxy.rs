// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

use super::paint_the_moon_proxy;

pub struct PaintHarvestScProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for PaintHarvestScProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = PaintHarvestScProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        PaintHarvestScProxyMethods { wrapped_tx: tx }
    }
}

pub struct PaintHarvestScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> PaintHarvestScProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<bool>,
    >(
        self,
        collection_token_id: Arg0,
        is_open: Arg1,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&collection_token_id)
            .argument(&is_open)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PaintHarvestScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PaintHarvestScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn start_harvest<
        Arg0: ProxyArg<paint_the_moon_proxy::Color>,
    >(
        self,
        color: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("start_harvest")
            .argument(&color)
            .original_result()
    }

    pub fn claim_harvest(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claim_harvest")
            .original_result()
    }

    pub fn change_harvest_color<
        Arg0: ProxyArg<paint_the_moon_proxy::Color>,
    >(
        self,
        new_color: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("change_harvest_color")
            .argument(&new_color)
            .original_result()
    }

    pub fn stop_harvest(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("stop_harvest")
            .original_result()
    }

    pub fn open_contract(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("open_contract")
            .original_result()
    }

    pub fn close_contract(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("close_contract")
            .original_result()
    }

    pub fn change_color_harvest_duration<
        Arg0: ProxyArg<paint_the_moon_proxy::Color>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        color: Arg0,
        new_duration: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("change_color_harvest_duration")
            .argument(&color)
            .argument(&new_duration)
            .original_result()
    }

    /// issue - color as SFT 
    pub fn issue_semi_fungible<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("issue_semi_fungible")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .original_result()
    }

    pub fn set_special_roles(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("set_special_roles")
            .original_result()
    }

    pub fn create_sft<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedVec<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        color: Arg0,
        attributes: Arg1,
        uris: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("create_sft")
            .argument(&color)
            .argument(&attributes)
            .argument(&uris)
            .original_result()
    }

    pub fn add_quantity<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        color_nonce: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("add_quantity")
            .argument(&color_nonce)
            .argument(&amount)
            .original_result()
    }
}