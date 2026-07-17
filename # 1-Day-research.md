# Research product requirements Crypto-deposiy-system-redesign.md 


## // Quee 
- Blockchain node → ZeroMQ topic → Blockbook listener → index/mempool synchronization


Rust Bitcoin Library - for generating 
https://docs.rs/bitcoin/latest/bitcoin/ PSBT 


1. What is PBST 
2. Do we need generate Wallets based on MNEMONIC ? 
3. Bitcoin and similar Nodes - Litecoin , Doge etc  suppirts  ZerMQ (https://github.com/bitcoin/bitcoin/blob/master/doc/zmq.md) for providing realtime data channel isntead of webscoket - using this requres Bitcoin ndoe configured specialy  aka requres your onw inftrastrture m vbecause I have have reserached and could not find providet .  the all only provide Http RPC nides pulling ...  so technically its better 

## ----------------------------------------------------------  ## 

## TODO: 

Continue research of BlockBook https://github.com/trezor/blockbook/tree/master
How api call organised - also look at alternatives. 
Run for one blockhain if possible ... 



** Code snippet generate  Wallet code Butcoin BIP-39 **

```rs

use bip39::{Mnemonic, Language, Seed};
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use bitcoin::secp256k1::Secp256k1;
use std::str::FromStr;

fn main() {

    let secp = Secp256k1::new();
    let network = Network::Bitcoin; // Use Network::Testnet for test networks

    // 1. Generate a random 12-word BIP39 mnemonic phrase
    let mnemonic = Mnemonic::generate_in(Language::English, 12).unwrap();
    println!("Mnemonic Phrase: {}", mnemonic);

    // 2. Generate the seed from the mnemonic (with an optional passphrase "")
    let seed = Seed::new(&mnemonic, "");

    // 3. Derive the Master Extended Private Key (Xpriv) from the seed
    let master_xpriv = Xpriv::new_master(network, seed.as_bytes()).unwrap();

    // 4. Define BIP84 derivation path for Native SegWit (m/84'/0'/0'/0/0)
    // m / purpose' / coin_type' / account' / change / address_index
    let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap();

    // 5. Derive the specific child private key
    let child_xpriv = master_xpriv.derive_priv(&secp, &derivation_path).unwrap();
    
    // 6. Extract the standard Private Key, Public Key, and Address
    let private_key = child_xpriv.to_priv();
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    let address = Address::p2wpkh(&public_key, network).unwrap();

    // Output the results
    println!("\nDerivation Path: {}", derivation_path);
    println!("Private Key (WIF): {}", private_key.to_wif());
    println!("Public Key (Hex):  {}", public_key);
    println!("Bitcoin Address:   {}", address);
}

```