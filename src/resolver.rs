
use std::convert::Infallible;

use bpstd::{Tx, Txid};
use rgbstd::resolvers::ResolveHeight;
use rgbstd::validation::{ResolveTx, TxResolverError};
use rgbstd::{Anchor, Layer1, WitnessAnchor};

pub struct PanickingResolver;
impl ResolveHeight for PanickingResolver {
    type Error = Infallible;
    fn resolve_anchor(&mut self, _: &Anchor) -> Result<WitnessAnchor, Self::Error> {
        unreachable!("PanickingResolver must be used only for newly issued contract validation")
    }
}
impl ResolveTx for PanickingResolver {
    fn resolve_tx(&self, _: Layer1, _: Txid) -> Result<Tx, TxResolverError> {
        unreachable!("PanickingResolver must be used only for newly issued contract validation")
    }
}
