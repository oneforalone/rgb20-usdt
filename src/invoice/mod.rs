// RGB wallet library for smart contracts on Bitcoin & Lightning network
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// #[macro_use]
// extern crate amplify;
// #[macro_use]
// extern crate strict_encoding;

#[allow(clippy::module_inception)]
mod invoice;
mod parse;
mod builder;
mod create_invoice;

pub use builder::RgbInvoiceBuilder;
pub use invoice::{Beneficiary, InvoiceState, RgbInvoice, RgbTransport};
pub use parse::{InvoiceParseError, TransportParseError};

#[cfg(test)]
mod test{
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;
    use amplify::hex::FromHex;
    use bp::{Outpoint, Txid, Vout};
    use rgb_schemata::{nia_rgb20, nia_schema};
    use rgbstd::containers::BindleContent;
    use rgbstd::interface::{FungibleAllocation, rgb20, Rgb20};
    use rgbstd::persistence::{Inventory, Stock};
    use rgbstd::stl::Precision;
    use crate::invoice::create_invoice;
    use crate::issue::Rgb20Contract;

    use crate::resolver::PanickingResolver;

    #[test]
    fn test_create_invoice(){
    //     let name = "MyToken";
    //     let decimal = Precision::CentiMicro; // Decimal: 8
    //     let desc = "A Customized RGB20 Token";
    //
    //     let txid = "0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516";
    //     let vout_index = 1;
    //     let owner = Outpoint::new(Txid::from_hex(txid).unwrap(), vout_index);
    //
    //     const ISSUE: u64 = 1_000_000_000;
    //
    //     let is_testnet = true;
    //
    //     let mut contract =
    //         Rgb20Contract::new(name, &decimal, desc, &owner, ISSUE, is_testnet).unwrap();
    //
    //     let mut beneficias = HashMap::<Outpoint, u64>::new();
    //     beneficias.insert(
    //         Outpoint::new(
    //             Txid::from_hex("42bf62541b26b38d28f3cf1b8d7935b28711f61abb034287a122e8228164fafe")
    //                 .unwrap(),
    //             1,
    //         ),
    //         1_000_000,
    //     );
    //
    //     contract.airdrop(&beneficias);
    //
    //     // to issue the contract, airdrop must call before it. Otherwise, there's no contract owner
    //     let contract = contract.issue().unwrap();
    //
    //     let contract_id = contract.contract_id();
    //     debug_assert_eq!(contract_id, contract.contract_id());
    //
    //     let bindle = contract.bindle();
    //     eprintln!("{bindle}");
    //     bindle
    //         .save("contracts/rgb20-usdt.contract.rgb")
    //         .expect("unable to save contract");
    //     fs::write("contracts/rgb20-usdt.contract.rgba", bindle.to_string())
    //         .expect("unable to save contract");
    // }
    //
    //     fn test_issue() {
    //         let txid = "af3b29844f4bddc4e9a0d8622b33b4f8e38013cbd647a84cfc4197057d67fdec";
    //         let vout_index = 0;
    //         let beneficiary = Outpoint::new(Txid::from_hex(txid).unwrap(), vout_index);
    //         let issueInfo = IssueInfo {
    //             tick: "COSM".to_string(),
    //             name: "COSMINMART".to_string(),
    //             decimal: Precision::CentiMicro,
    //             desc: "issue by BITLIGHT".to_string(),
    //             beneficiary: beneficiary,
    //             total_suppl: 1_000_000,
    //         };
    //
    //         let contract = IssueContract(&issueInfo).unwrap();
    //
    //         let contract_id = contract.contract_id();
    //         debug_assert_eq!(contract_id, contract.contract_id(), "Contract ID mismatch");
    //
    //         let bindle = contract.bindle();
    //         // eprintln!("{bindle}");
    //         bindle.save("contract/rgb20-usdt.contract.rgb")
    //             .expect("unable to save contract");
    //         fs::write("contract/rgb20-usdt.contract.rgba", bindle.to_string())
    //             .expect("unable to save contract");
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

            // Let's create some stock - an in-memory stash and inventory around it:
            let mut stock = Stock::default();
            stock.import_iface(rgb20()).unwrap();
            stock.import_schema(nia_schema()).unwrap();
            stock.import_iface_impl(nia_rgb20()).unwrap();

            //we verify our contract consignment and add it to the stock
        let mut resolver = PanickingResolver;
            let verified_contract = match bindle.unbindle().validate(&mut resolver,is_testnet) {
                Ok(consignment) => consignment,
                Err(consignment) => {
                    panic!(
                        "can't produce valid consignment. Report: {}",
                        consignment.validation_status()
                            .expect("status always present upon validation")
                    );
                }
            };

        //     // let contractUnbindle = bindle.unbindle();
        //     // let contract_terminals = &contractUnbindle.terminals;
        //     // let contract_schema = &contractUnbindle.schema;
        //     // let ifaces = &contractUnbindle.ifaces;
        //     // let root_schema_id = &contractUnbindle.root_schema_id();
        //     // let contract_id = &contractUnbindle.contract_id();
        //     // let schema_id = &contractUnbindle.schema_id();
        //     // let genesis = &contractUnbindle.genesis;
        //     //
        //     //
        //     // eprintln!("1:root_schema_id{root_schema_id:#?} \n \
        //     // 2:contract_id{contract_id:#?}\n\
        //     // 3:genesis{genesis:#?}\n
        //     // 4:schema_id{schema_id:#?}");
        //     //
        //
        //     stock.import_contract(verified_contract, &mut resolver).unwrap();
        //
        //     // Reading contract state through the interface from the stock:
        //     let contract = stock
        //         .contract_iface_id(contract_id, rgb20().iface_id())
        //         .unwrap();
        //     debug_assert_eq!(contract_id, contract.contract_id(), "Contract ID mismatch");
        //     let contract = Rgb20::from(contract);
        //     let totalsuppl: u64 = contract.total_supply().into();
        //     // debug_assert_eq!(issueInfo.total_suppl, totalsuppl, "total supply is mismatch");
        //     // debug_assert_eq!(issueInfo.name, contract.spec().naming.name.to_string(), "name is mismatch");
        //     // debug_assert_eq!(issueInfo.desc, contract.spec().naming.details.unwrap().to_string(), "desc is mismatch");
        //     // debug_assert_eq!(issueInfo.tick, contract.spec().naming.ticker.to_string(), "tick is mismatch");
        //     // debug_assert_eq!(issueInfo.decimal, contract.spec().precision, "decimal is mismatch");
        //
        //     //
        //     // let owner = vec![beneficiary];
        //     // let allocations = contract.fungible("assetOwner", &owner).unwrap();
        //     // eprintln!("{}", serde_json::to_string(&contract.spec()).unwrap());
        //     // for FungibleAllocation { owner, witness, value } in allocations {
        //     //     eprintln!("amount={value}, owner={owner}, witness={witness}");
        //     // }
        //     // eprintln!("totalSupply={}", contract.total_supply());
        //     // eprintln!("created={}", contract.created().to_local().unwrap());
        //
        //     //test stock
        //     let invoice_amount = 100;
        //     let iface = "RGB20".to_string();
        // let outpoint=Outpoint::new( Txid::from_str("a73fe785ce35573f551a179c607be34e394ebe784b4e44f691df0db919a6122b").unwrap(),Vout::from_u32(0));
        //     // let seal = "tapret1st:a73fe785ce35573f551a179c607be34e394ebe784b4e44f691df0db919a6122b:0";
        //     let network = "testnet";
        //     let params: Option<HashMap<String, String>> = None;
        //     let params = params.unwrap_or_default();
        //     let invoice = create_invoice::create_invoice(
        //         &contract_id.to_string(),
        //         &iface,
        //         invoice_amount,
        //         &outpoint,
        //         params,
        //         &mut stock,
        //     ).unwrap();
        //     eprintln!("invoice={}", invoice);
        //


    }
}
