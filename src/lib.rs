// TODOs:
// - [x]  Create a wallet for Alice and Bob
//     - [x]  Get Alice&Bob’s pubkey
// - [ ]  Import a self-defined RGB20 contract for Alice and Bob
// - [ ]  Transfer
//     - [ ]  Create an invoice
//     - [ ]  Construct a PSBT
//     - [ ]  Make a transfer
//     - [ ]  Accept the transfer
//     - [ ]  Sign the PSBT and broadcast it
mod btc_wallet;
mod invoice;
mod issue;
mod resolver;
mod psbt;

use std::{collections::HashMap, path::PathBuf};
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

    pub fn load_contract(&self, contract: PathBuf) -> Result<Self> {
        // Load contract for local machine
        todo!();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;
    use std::{fs, path::Path};

    use amplify::hex::FromHex;
    use bp::{Outpoint, Txid};
    use rgbstd::containers::BindleContent;
    use rgbstd::stl::Precision;

    #[test]
    fn test_contract_creation() {
        let name = "MyToken";
        let decimal = Precision::CentiMicro; // Decimal: 8
        let desc = "A Customized RGB20 Token";

        let txid = "0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516";
        let vout_index = 1;
        let owner = Outpoint::new(Txid::from_hex(txid).unwrap(), vout_index);

        const ISSUE: u64 = 1_000_000_000;

        let is_testnet = true;

        let mut contract =
            Rgb20Contract::new(name, &decimal, desc, &owner, ISSUE, is_testnet).unwrap();

        let mut beneficias = HashMap::<Outpoint, u64>::new();
        beneficias.insert(
            Outpoint::new(
                Txid::from_hex("42bf62541b26b38d28f3cf1b8d7935b28711f61abb034287a122e8228164fafe")
                    .unwrap(),
                1,
            ),
            1_000_000,
        );

        contract.airdrop(&beneficias);

        // to issue the contract, airdrop must call before it. Otherwise, there's no contract owner
        let contract = contract.issue().unwrap();

        let contract_id = contract.contract_id();
        debug_assert_eq!(contract_id, contract.contract_id());

        let bindle = contract.bindle();
        eprintln!("{bindle}");

        let contract_file = "contracts/rgb20-usdt.contract.rgba";
        let contract_bin = "contracts/rgb20-usdt.contract.rgb";
        bindle
            .save(contract_bin)
            .expect("unable to save contract binary");
        fs::write(contract_file, bindle.to_string()).expect("unable to save contract plaintext");

        assert!(Path::new(contract_bin).exists());
        assert!(Path::new(contract_file).exists());
    }
}
