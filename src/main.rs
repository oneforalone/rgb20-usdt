use std::convert::Infallible;
use std::fs;

use amplify::hex::FromHex;
use bp::{Outpoint, Tx, Txid};
use rgb_schemata::{nia_rgb20, nia_schema};
use rgbstd::containers::BindleContent;
use rgbstd::interface::{rgb20, ContractBuilder};
use rgbstd::resolvers::ResolveHeight;
use rgbstd::stl::{
    Amount, ContractData, DivisibleAssetSpec, Precision, RicardianContract, Timestamp,
};
use rgbstd::validation::{ResolveTx, TxResolverError};
use rgbstd::{Anchor, Layer1, WitnessAnchor};
use strict_encoding::StrictDumb;

struct DumbResolver;

impl ResolveTx for DumbResolver {
    fn resolve_tx(&self, _: Layer1, _: Txid) -> Result<Tx, TxResolverError> {
        Ok(Tx::strict_dumb())
    }
}

impl ResolveHeight for DumbResolver {
    type Error = Infallible;
    fn resolve_anchor(&mut self, _: &Anchor) -> Result<WitnessAnchor, Self::Error> {
        Ok(WitnessAnchor::strict_dumb())
    }
}

#[rustfmt::skip]
fn main() {
    let name = "MyToken";
    let decimal = Precision::CentiMicro; // Decimal: 8
    let desc = "A Customized RGB20 Token";
    let spec = DivisibleAssetSpec::with("RGB20", name, decimal, Some(desc)).unwrap();
    let terms = RicardianContract::default();
    let contract_data = ContractData { terms, media: None };
    let created = Timestamp::now();
    let txid = "0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516";
    let vout_index = 1;
    let beneficiary = Outpoint::new(Txid::from_hex(txid).unwrap(), vout_index);

    const ISSUE: u64 = 1_000_000_000;

    let is_testnet = true;

    let contract = ContractBuilder::with(rgb20(), nia_schema(), nia_rgb20(), is_testnet)
        .expect("schema fails to implement RGB20 interface")
        .add_global_state("spec", spec)
        .expect("invalid nominal")
        .add_global_state("created", created)
        .expect("invalid creation date")
        .add_global_state("issuedSupply", Amount::from(ISSUE))
        .expect("invalid issued supply")
        .add_global_state("data", contract_data)
        .expect("invalid contract text")
        .add_fungible_state("assetOwner", beneficiary, ISSUE)
        .expect("invalid asset amount")
        .issue_contract()
        .expect("contract doesn't fit schema requirements");

    let contract_id = contract.contract_id();
    debug_assert_eq!(contract_id, contract.contract_id());

    let bindle = contract.bindle();
    eprintln!("{bindle}");
    bindle.save("contracts/rgb20-usdt.contract.rgb")
        .expect("unable to save contract");
    fs::write("contracts/rgb20-usdt.contract.rgba", bindle.to_string())
        .expect("unable to save contract");
}
