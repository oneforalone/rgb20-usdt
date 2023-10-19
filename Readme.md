# RGB20 USDT Demo

**DECLAIMER: This is just a demo for RGB20 on testnet, it's NOT the offical
USDT of RGB20**

## Pre-request

To complete the demo, you need to setup the following toolchains:

1. [Git][git]
2. [Rust][rust]
3. [Sparrow][sparrow]
4. [RGB-CLI][rgb-cli]

[git]: https://git-scm.com/
[rust]: https://www.rust-lang.org/tools/install
[sparrow]: https://sparrowwallet.com/
[rgb-cli]: https://github.com/RGB-WG/rgb

Also, you need to create two different wallets in Sparrow. Let's say `alice` and
`bob`, and get some satoshis in these two wallets. You may get satoshis faucet
from <https://bitcoinfaucet.uo1.net/> and <https://coinfaucet.eu/en/btc-testnet/>

> NOTE: Sparrow default is mainnet, so create wallet after restarting Sparrow
> in testnet.

Here are the tpub of wallets I created:

- Alice: `tpubDCRYBMFMJcwYAd7g7GEdNzTnqg7k8wE2PQZ6NgXq2tJER3aWSK4sRNpje4nnvxC9Ffs2borWAgdQMDi8J8b4sXBQNqMnFxVWWVA6BswzGyU`
- Bob: `tpubDCeaDjmVbxPkLxvXgmdsTjGF55WWL4Kf1Z64eQstJAyqQ7DaBD9DDGo7Q26yxww5ifbFuELZcmxnM8LkJ1Xmij6itneguA5VKcqc5YbMbjz`

### RGB-CLI

Cause there's issue while making transfer, so we need to make a trivial change
with source code and build rgb-cli from source.

First, clone it from Github:

```bash
$ git clone https://github.com/RGB-WG/rgb.git rgb-cli
```

Then, build it to downloading the dependencies:

```
$ cd rgb-cli
$ cargo build
```

After that, locate `rgb-wallet` source codes path, for me, it's:
`~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rgb-wallet-0.10.9/src`

Comment out **line 157** in `pay.rs`:

```rust
                // .filter(|index| *index == RGB_NATIVE_DERIVATION_INDEX || *index == RGB_TAPRET_DERIVATION_INDEX)
```

Finally, install `rgb` command line:

```
$ cd rgb-cli
$ cargo install rgb-contracts --all-features --force --path .
```

> For some reason, I failed with clone `rgb-wallet` lib source code and change
> its dependencies in `rgb` source codes on local machine. I know I can fork
> `rgb-wallet` into my repo, and modify it. Change `rgb` dependency to my repo,
> but i believe there must be a way to do it on local. Also, cause I'm not sure
> whether the `filter` is an issue or a feature. Therefore, I choose to make it
> in some kind nasty way. If anyone has good ideas, please let me know.

## Create and import

To create a RGB20 contract, just clone this repo to you local machine. Then
compile and run it.

```bash
$ git clone https://github.com/oneforalone/rgb20-usdt.git
$ cd rgb20-usdt
$ cargo run
```

Now, we are creating a RGB20 usdt contract, which stores in `contracts` fold.

Before importing contracts, let's import our wallets to rgb.

For Alice:
```bash
$ rgb -d .alice create default <alice-tpub>
$ rgb -d .alice import contracts/rgb20-usdt.contract.rgb
```

For Bob:
```bash
$ rgb -d .bob create default <bob-tpub>
$ rgb -d .bob import contracts/rgb20-usdt.contract.rgb
```

After that, we can inspect contracts state with `rgb state` command.

For Alice:
```bash
$ rgb -d .alice contracts
rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp

$ rgb -d .alice state rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp RGB20
Global:
  spec := (naming=(ticker=("RGB20"), name=("USDT"), details=1(("USD Tether Token"))), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (1000000000)
  created := (1697651102)

Owned:
  assetOwner:
    amount=1000000000, utxo=0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516:1, witness=~ # owner unknown

$ rgb -d .alice state rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp RGB20 -w default
Global:
  spec := (naming=(ticker=("RGB20"), name=("USDT"), details=1(("USD Tether Token"))), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (1000000000)
  created := (1697651102)

Owned:
  assetOwner:
    amount=1000000000, utxo=0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516:1, witness=~, derivation=*/0/0
```

For Bob:

```bash
$ rgb -d .bob contracts
rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp

$ rgb -d .bob state rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp RGB20
Global:
  spec := (naming=(ticker=("RGB20"), name=("USDT"), details=1(("USD Tether Token"))), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (1000000000)
  created := (1697651102)

Owned:
  assetOwner:
    amount=1000000000, utxo=0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516:1, witness=~ # owner unknown
```

Now we'are successfully create an USDT rgb20 token, and the owner is
`0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516:1`, which is
belongs to Alice.

## Transfer

There're about five steps in a complete transfer:
1. Create an invoice
2. Construct a psbt
3. Make a transfer
4. Accept transfer
5. Sign psbt and broadcast it

Let's say Alice need to send 1,000 USDT to Bob.

### Create an invoice

To receive 1,000 USDT, Bob needs to create an invoice and send it to Alice.

```bash
$ rgb -d .bob invoice s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp RGB20 1000 tapret1st:42a3d5805a4f7e315bb5905c0dc0dd95914f5c97239b409360bf223a7d9dea0e:0
rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp/RGB20/1000+utxob:bm9pt3W-KCodqSqLq-UPXdsdv8f-f5nszKdEe-T4yZhw5sj-TcdG5G
```
The last arguments is an utxo of Bob, you can specify any utxo on chian.

### Construct a psbt

Bob sent the invoice to Alice, and then Alice start to construct a psbt using
Sparrow. Just create a simple transfer tx to send some satoshis to an address
of herself. And save it before sign and broadcast. I save it in `psbt/usdt-transfer.psbt`.
For some reason i don't know, we need also `set-host` with the psbt:

```bash
$ rgb -d .alice set-host psbt/usdt-transfer.psbt
PSBT file 'psbt/usdt-transfer.psbt' is updated with tapret1st host now set.
```

### Make a transfer

After constructing a psbt file and `set-host` to it. Alice now can make a
tranfser with the psbt and invoice provided by Bob:

```bash
$ rgb -d .alice transfer psbt/usdt-transfer.psbt rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp/RGB20/1000+utxob:bm9pt3W-KCodqSqLq-UPXdsdv8f-f5nszKdEe-T4yZhw5sj-TcdG5G transfer.rgb
```

The transfer is saved in `transfer.rgb` file, and need to send it to Bob
waiting him to accept it.

### Accept transfer

After receiving the `transfer.rgb` file, Bob could validate it before accepting:

```bash
$ rgb -d .bob validate transfer.rgb
Consignment has non-mined terminal(s)
Non-mined terminals:
- 26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943
Validation warnings:
- terminal witness transaction 26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943 is not yet mined.
```

The result show that Alice not publich the psbt and not mined yet. So, let's
make Bob accept:

```bash
$ rgb -d .bob accept -f transfer.rgb
Consignment has non-mined terminal(s)
Non-mined terminals:
- 26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943
Validation warnings:
- terminal witness transaction 26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943 is not yet mined.

Transfer accepted into the stash
```

These show that Bob already accept the transfer.

### Sign psbt and broadcast it

Now it's Alice's turn to sign the psbt file and broadcast it. Just load the
psbt file with Sparrow, sign it, and broadcast it. And just waiting it mined.

After a few minutes, the psbt would mined on chain. In that time, Bob and Alice
would get different outputs with `rgb state` and `rgb validate`.

For Alice:

```bash
$ rgb -d .alice state rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp RGB20
Global:
  spec := (naming=(ticker=("RGB20"), name=("USDT"), details=1(("USD Tether Token"))), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (1000000000)
  created := (1697651102)

Owned:
  assetOwner:
    amount=999999000, utxo=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943:0, witness=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943 # owner unknown
    amount=1000000000, utxo=0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516:1, witness=~ # owner unknown
```

For Bob:

```bash
$ rgb -d .bob state rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp RGB20
Global:
  spec := (naming=(ticker=("RGB20"), name=("USDT"), details=1(("USD Tether Token"))), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (1000000000)
  created := (1697651102)

Owned:
  assetOwner:
    amount=999999000, utxo=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943:0, witness=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943 # owner unknown
    amount=1000, utxo=42a3d5805a4f7e315bb5905c0dc0dd95914f5c97239b409360bf223a7d9dea0e:0, witness=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943 # owner unknown
    amount=1000000000, utxo=0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516:1, witness=~ # owner unknown

$ rgb -d .bob state rgb:s6syetq-eN85tdbpb-hBut9K6F3-uFfjVZsJ6-WVYGv6FXa-UTwCnp RGB20 -w default
Global:
  spec := (naming=(ticker=("RGB20"), name=("USDT"), details=1(("USD Tether Token"))), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (1000000000)
  created := (1697651102)

Owned:
  assetOwner:
    amount=999999000, utxo=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943:0, witness=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943 # owner unknown
    amount=1000, utxo=42a3d5805a4f7e315bb5905c0dc0dd95914f5c97239b409360bf223a7d9dea0e:0, witness=26fbb878fa910068c7dc0b1cae2bef235f477ca095aa9b85eaeeb1a87adfe943, derivation=*/0/0
    amount=1000000000, utxo=0018dc9fff99382ac228a6cbcbdddb0989329d85d95c1e354b74a488da324516:1, witness=~ # owner unknown

$ rgb -d .bob validate transfer.rgb
Consignment is valid
```

Vola, we just made it.

The RGB21 and RGB25 are just the same except some initial arguments. You may
check the details with [rgb-schemata][rgb-schemata] repo.

[rgb-schemata]: https://github.com/RGB-WG/rgb-schemata/tree/master/examples
