// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;
use multiversx_sc_snippets_dapp::multiversx_sc;

pub struct PaintTheMoonScProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for PaintTheMoonScProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = PaintTheMoonScProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        PaintTheMoonScProxyMethods { wrapped_tx: tx }
    }
}

pub struct PaintTheMoonScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> PaintTheMoonScProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PaintTheMoonScProxyMethods<Env, From, To, Gas>
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
impl<Env, From, To, Gas> PaintTheMoonScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn paint<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<Color>,
    >(
        self,
        point: Arg0,
        new_color: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("paint")
            .argument(&point)
            .argument(&new_color)
            .original_result()
    }

    pub fn get_all_points(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Point>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllPoints")
            .original_result()
    }

    pub fn initial_map_setup<
        Arg0: ProxyArg<ManagedVec<Env::Api, Point>>,
    >(
        self,
        points: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("initial_map_setup")
            .argument(&points)
            .original_result()
    }
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Copy, Clone, PartialEq, ManagedVecItem, Debug,
)]
pub enum Color {
    White,
    Black,
    Blue,
    Red,
    Yellow,
    Green,
    Purple,
    Grey,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Copy, Clone, Debug, PartialEq,
)]
pub struct Point {
    pub coordinates: u64,
    pub color: Color,
}