// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{Client, Seed};

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b3").unwrap(),
    )
    .unwrap(); // Insert your seed

    let addresses = iota.find_addresses(&seed).account_index(0).range(0..4).get().unwrap();

    println!(
        "List of generated address: {:#?}",
        addresses.iter().map(|(a, _)| a.to_bech32()).collect::<Vec<String>>()
    );
}
