// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

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
    pub fn block_size(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("block_size")
            .original_result()
    }

    pub fn paint<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<usize>,
        Arg2: ProxyArg<u8>,
    >(
        self,
        x: Arg0,
        y: Arg1,
        new_color: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("paint")
            .argument(&x)
            .argument(&y)
            .argument(&new_color)
            .original_result()
    }

    pub fn paint_rect<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<usize>,
        Arg2: ProxyArg<usize>,
        Arg3: ProxyArg<usize>,
        Arg4: ProxyArg<u8>,
    >(
        self,
        x0: Arg0,
        y0: Arg1,
        xr: Arg2,
        yr: Arg3,
        new_color: Arg4,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("paint_rect")
            .argument(&x0)
            .argument(&y0)
            .argument(&xr)
            .argument(&yr)
            .argument(&new_color)
            .original_result()
    }
}