use std::collections::HashMap;

use anyhow::Result;
use bp::Outpoint;
use rgb_schemata::{nia_rgb20, nia_schema};
use rgbstd::{
    containers::Consignment,
    interface::{rgb20, ContractBuilder},
    stl::{Amount, ContractData, DivisibleAssetSpec, Precision, RicardianContract, Timestamp},
};
#[derive(Clone, Debug)]
pub struct Rgb20Contract {
    name: String,
    decimal: Precision,
    desc: String,
    owner: Outpoint,
    supply: u64,
    // todo: need to improve for multi-chain support in future.
    is_testnet: bool,
    builder: ContractBuilder,
}

impl Rgb20Contract {
    pub fn new(
        name: &str,
        decimal: &Precision,
        desc: &str,
        owner: &Outpoint,
        supply: u64,
        is_testnet: bool,
    ) -> Result<Self> {
        let builder = ContractBuilder::with(rgb20(), nia_schema(), nia_rgb20(), is_testnet)
            .expect("schema fails to implement RGB20 interface");

        let spec = DivisibleAssetSpec::with("RGB20", name, *decimal, Some(desc)).unwrap();
        let builder = builder
            .add_global_state("spec", spec)
            .expect("invalid nominal");

        let terms = RicardianContract::default();
        let contract_data = ContractData { terms, media: None };

        let builder = builder
            .add_global_state("data", contract_data)
            .expect("invalid contract text");

        Ok(Rgb20Contract {
            name: name.to_owned(),
            decimal: *decimal,
            desc: desc.to_owned(),
            owner: *owner,
            supply,
            is_testnet,
            builder,
        })
    }

    pub fn issue(&self) -> Result<Consignment<false>> {
        let builder = self.builder.to_owned();
        let builder = builder
            .add_global_state("issuedSupply", Amount::from(self.supply))
            .expect("invalid issued supply");
        let builder = builder
            .add_global_state("created", Timestamp::now())
            .expect("invalid creation date");
        let contract = builder
            .issue_contract()
            .expect("contract doesn't fit schema requiremetns");
        Ok(contract)
    }

    pub fn airdrop(&mut self, beneficias: &HashMap<Outpoint, u64>) {
        let mut airdrop_amount = 0;
        let mut builder = self.builder.to_owned();
        for (user, amount) in beneficias {
            airdrop_amount += amount;
            builder = builder
                .add_fungible_state("assetOwner", user, *amount)
                .expect("invalid asset amount");
        }
        let rest = self.supply - airdrop_amount;
        builder = builder
            .add_fungible_state("assetOwner", self.owner, rest)
            .expect("invalid asset amount");
        self.builder = builder;
    }
}
