use std::collections::HashMap;
use std::fs;

use amplify::hex::FromHex;
use bp::{Outpoint, Txid};
use rgbstd::containers::BindleContent;
use rgbstd::stl::Precision;

use rgb20_usdt::Rgb20Contract;

fn main() {
    let name = "MyToken";
    let decimal = Precision::CentiMicro; // Decimal: 8
    let desc = "A Customized RGB20 Token";

    let txid = "0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516";
    let vout_index = 1;
    let owner = Outpoint::new(Txid::from_hex(txid).unwrap(), vout_index);

    const ISSUE: u64 = 1_000_000_000;

    let is_testnet = true;

    let mut contract = Rgb20Contract::new(name, &decimal, desc, &owner, ISSUE, is_testnet).unwrap();

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
    bindle
        .save("contracts/rgb20-usdt.contract.rgb")
        .expect("unable to save contract");
    fs::write("contracts/rgb20-usdt.contract.rgba", bindle.to_string())
        .expect("unable to save contract");
}
