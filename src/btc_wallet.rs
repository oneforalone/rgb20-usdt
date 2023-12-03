use bip32::{Language, Mnemonic, Prefix, XPrv, XPub};
use hex::{FromHex, ToHex};
use rand_core::OsRng;
use std::str::FromStr;

pub enum BtcAddrType {
    // Legacy Address Type, start with `1`
    P2PKH,
    // Pay-To-Script-Hash Address Type, start with `3`
    P2SH,
    // SegWit Native Address Type, start with `bc1`
    P2WPKH,
    // Bitcoin Testnet Address, start with `m`/`n`/`tb1`
    Testnet,
}

pub struct BtcWallet {
    mnemonic: Mnemonic,
    derivation: &'static str,
    // password of mnemonic, BIP39
    password: &'static str,
    addr_type: BtcAddrType,
}

impl BtcWallet {
    pub fn new() -> Self {
        let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
        BtcWallet {
            mnemonic,
            derivation: "m/86'/1'/0'",
            password: "",
            addr_type: BtcAddrType::P2WPKH,
        }
    }

    pub fn with(
        derivation: &'static str,
        password: &'static str,
        address_type: BtcAddrType,
    ) -> Self {
        BtcWallet {
            mnemonic: Mnemonic::random(&mut OsRng, Default::default()),
            derivation,
            password,
            addr_type: address_type,
        }
    }

    pub fn from_seeds(seeds: &str) -> Self {
        let mnemonic = match bip39::Mnemonic::from_str(seeds) {
            Ok(res) => res,
            Err(err) => panic!("Can not import from {seeds}, error is: {err}"),
        };
        let entropy: [u8; 32] = match mnemonic.to_entropy().try_into() {
            Ok(res) => res,
            Err(err) => panic!("Error: {:?}", err),
        };

        let mnemonic = Mnemonic::from_entropy(entropy, Language::English);
        BtcWallet {
            mnemonic,
            derivation: "m/86'/1'/0'",
            password: "",
            addr_type: BtcAddrType::P2WPKH,
        }
    }

    pub fn from_hex(hex_str: &str) -> Self {
        let entropy = <[u8; 32]>::from_hex(hex_str).unwrap();

        let mnemonic = Mnemonic::from_entropy(entropy, Language::English);
        BtcWallet {
            mnemonic,
            derivation: "m/86'/1'/0'",
            password: "",
            addr_type: BtcAddrType::P2WPKH,
        }
    }

    pub fn get_seeds(&self) -> &str {
        self.mnemonic.phrase()
    }

    pub fn get_xprv(&self) -> XPrv {
        let seed = self.mnemonic.to_seed(self.password);
        let path = self.derivation.parse().unwrap();
        match XPrv::derive_from_path(&seed, &path) {
            Ok(res) => res,
            Err(_) => panic!("could not get xprv"),
        }
    }

    pub fn get_xpub(&self) -> XPub {
        self.get_xprv().public_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tpub() {
        let mut wallet = BtcWallet::new();
        wallet.addr_type = BtcAddrType::Testnet;

        let xprv = wallet.get_xprv();
        let xprv = xprv.to_string(Prefix::TPRV);
        assert!(xprv.starts_with("tprv"));

        let xpub = wallet.get_xpub();
        let xpub = xpub.to_string(Prefix::TPUB);
        assert!(xpub.starts_with("tpub"))
    }

    #[test]
    fn test_xpub() {
        let mut wallet = BtcWallet::new();
        wallet.addr_type = BtcAddrType::P2PKH;

        let xprv = wallet.get_xprv();
        let xprv = xprv.to_string(Prefix::XPRV);
        assert!(xprv.starts_with("xprv"));

        let xpub = wallet.get_xpub();
        let xpub = xpub.to_string(Prefix::XPUB);
        assert!(xpub.starts_with("xpub"));
    }

    #[test]
    fn test_ypub() {
        let mut wallet = BtcWallet::new();
        wallet.addr_type = BtcAddrType::P2SH;

        let xprv = wallet.get_xprv();
        let xprv = xprv.to_string(Prefix::YPRV);
        assert!(xprv.starts_with("yprv"));

        let xpub = wallet.get_xpub();
        let xpub = xpub.to_string(Prefix::YPUB);
        assert!(xpub.starts_with("ypub"));
    }

    #[test]
    fn test_zpub() {
        let mut wallet = BtcWallet::new();
        wallet.addr_type = BtcAddrType::P2WPKH;

        let xprv = wallet.get_xprv();
        let xprv = xprv.to_string(Prefix::ZPRV);
        assert!(xprv.starts_with("zprv"));

        let xpub = wallet.get_xpub();
        let xpub = xpub.to_string(Prefix::ZPUB);
        assert!(xpub.starts_with("zpub"));
    }

    #[test]
    fn test_wallet_from_seeds() {
        let seeds_orig = "arm coach story elbow quarter duck tomato twenty rough random walk tattoo orient crucial case snap mix cart charge bring kind icon steel expect";
        let mut wallet = BtcWallet::from_seeds(seeds_orig);
        wallet.addr_type = BtcAddrType::Testnet;

        let entropy = wallet.mnemonic.entropy().to_owned();
        let entropy_orig: [u8; 32] = [
            11, 165, 151, 90, 35, 154, 246, 135, 185, 7, 89, 188, 86, 55, 218, 111, 41, 200, 104,
            200, 206, 106, 142, 4, 96, 154, 14, 23, 168, 224, 181, 74,
        ];
        assert_eq!(entropy, entropy_orig);

        let seeds = wallet.get_seeds();
        assert_eq!(seeds, seeds_orig);

        let seeds_orig2 = "fold ask loud spy zebra just crazy outside unusual rough double room afford sketch load biology relief invite swing silly kick acquire page blue";
        let mut wallet2 = BtcWallet::from_seeds(seeds_orig2);
        wallet2.addr_type = BtcAddrType::Testnet;

        let xpub = wallet2.get_xpub();
        let xpub = xpub.to_string(Prefix::TPUB);
        let xpub_orig = "tpubDDFXYtk6MYAv7gvSwTXYaFBm2XJfzzrDsjBVSU1Cwo4BUgwCSWfX9KAhNgJFTDybaSzUhywUho9jfpot3QRAcB8ZpvhSTVixxiy5mHft9QL";
        assert_eq!(xpub.as_str(), xpub_orig);
    }

    #[test]
    fn test_wallet_from_hex() {
        let hex_str = "0ba5975a239af687b90759bc5637da6f29c868c8ce6a8e04609a0e17a8e0b54a";
        let mut wallet = BtcWallet::from_hex(hex_str);
        wallet.addr_type = BtcAddrType::Testnet;

        let entropy_orig = <[u8; 32]>::from_hex(hex_str).unwrap();
        let entropy = wallet.mnemonic.entropy();

        assert_eq!(entropy, &entropy_orig);

        let xpub = wallet.get_xpub();
        let xpub = xpub.to_string(Prefix::TPUB);
        let xpub_orig = "tpubDCSgjFcLS9iZkN6H4BxuhUgabTchh3qqad49fYqndF595dnhcVTApsaEfGXXMVJTh2wrT2wKWPbcrSaX6VNMSaBp8NYThxHj3DA8oDiUNcK";
        assert_eq!(xpub.as_str(), xpub_orig);
    }
}
