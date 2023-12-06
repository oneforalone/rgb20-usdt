use seals::txout::ExplicitSeal;
use strict_encoding::{StrictDeserialize, TypeName};
use std::{collections::HashMap, str::FromStr};
use indexmap::IndexMap;
use amplify::{Display, Error, From};
use rgbstd::{
    containers::{Bindle, Transfer},
    contract::{GraphSeal},
    interface::TypedState,
    persistence::{Inventory, Stash, Stock},
    resolvers::ResolveHeight,
    validation::{ ConsignmentApi, ResolveTx, Status,},
};
use rgb::ContractId;
use bp::{seals::txout::CloseMethod, Txid};
use bpstd::{ Outpoint};
use invoice::Network;
use crate::invoice::{InvoiceState, RgbInvoice, RgbTransport,Beneficiary};

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum NewInvoiceError {
    /// '{0}' is an invalid iface name
    WrongIface(String),
    /// '{0}' is an invalid contract id
    WrongContract(String),
    /// '{0}' is an invalid seal definition
    WrongSeal(String),
    /// Network cannot be decoded. {0}
    WrongNetwork(String),
    /// {0} is unspecified or wrong contract id
    NoContract(String),
    /// There are no contracts defined in Stash
    EmptyContracts,
    /// Error saving secret seal: {0}
    StoreSeal(String),
}

pub fn create_invoice(
    contract_id: &str,
    iface: &str,
    amount: u64,
    outpoint:&Outpoint,
    params: HashMap<String, String>,
    stock: &mut Stock,
) -> Result<RgbInvoice, NewInvoiceError> {
    let ty =
        TypeName::from_str(iface).map_err(|_| NewInvoiceError::WrongIface(iface.to_string()))?;
    let iface = stock
        .iface_by_name(&ty)
        .map_err(|_| NewInvoiceError::WrongIface(iface.to_string()))?;

    let contract_id = ContractId::from_str(contract_id)
        .map_err(|_| NewInvoiceError::NoContract(contract_id.to_string()))?;

    // Temporary removal
    // if !stock
    //     .contract_ids()
    //     .map_err(|_| NewInvoiceError::EmptyContracts)?
    //     .contains(&contract_id)
    // {
    //     return Err(NewInvoiceError::NoContract(contract_id.to_string()));
    // };


    // let seal = ExplicitSeal::<Txid>::from_str(seal)
    //     .map_err(|_| NewInvoiceError::WrongIface(seal.to_string()))?;
    // let seal = GraphSeal::new(seal.method.into(), seal.txid, seal.vout);
    // // Query Params

            let seal = GraphSeal::new(
                CloseMethod::TapretFirst,
                outpoint.txid,
                outpoint.vout,
            );

    let beneficiary=Beneficiary::BlindedSeal(seal.to_concealed_seal());


    let mut query = IndexMap::default();
    for (k, v) in params {
        query.insert(k, v);
    }

    // Generate Invoice
    let invoice = RgbInvoice {
        transports: vec![RgbTransport::UnspecifiedMeans],
        contract: Some(contract_id),
        iface: Some(iface.name.clone()),
        operation: None,
        assignment: None,
        // beneficiary: seal.to_concealed_seal().into(),
        beneficiary,
        owned_state: InvoiceState::Amount(amount),
        unknown_query: query,
        expiry: None,
        network: Some(Network::Testnet3),
    };

    println!("{invoice}");

    Ok(invoice)
}