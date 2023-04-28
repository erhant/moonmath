use log::debug;
use prompt::{puzzle, welcome};
use semaphore::{AccessSet, PrivKey, PubKey};
use std::{io::Write, time::Instant};
use winter_utils::Serializable;

// DATA
// ================================================================================================

/// Public keys of users in the access set.
const PUB_KEYS: [&str; 8] = [
    "04f6d8d05f52012c0a705c1e0dcb1ff64ba0842c8c14f1f0f18e95254bdcfbea",
    "af84cf58cb71709c5a94750e69f9cbad0244d6c8e437f4e822c58f0c45c69ea0",
    "964650c5645e30b1ff74574a6fc4cdb78eaa1be3dfd43f01050b1b0e41d4db36",
    "d5a494b415c20d7d00fbace4f725b596da7c646d80e622956d7f09eebc93fef9",
    "9d7083734388833056ae25382dbcfb39b6a1ee78a6d63f136d83400569adc319",
    "a7ae57a7b2c60871e86d152e9e712ab5a3630f6183a7c1d07ba4429fead88018",
    "1995c40e8e46a009b0d61d89634f3c959d13322ef3a84b410a811eb4fc06d08b",
    "cf855bce16bb7b37f874324da9f72dd0d0e6f6e9f9e29100f66c7b57c6895ef5",
];

/// Our private key; this key corresponds to the 4th public key above (d5a494b415c2...).
const MY_PRIV_KEY: &str = "86475af21e4445b71bfa496416ee2d0765946bd3a854a77fe07db53c7994d0a5";

/// A topic on which we'll send a signal
const TOPIC: &str = "The Winter is Coming...";

// SEMAPHORE TESTER
// ================================================================================================

pub fn main() {
    // configure logging
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(log::LevelFilter::Debug)
        .init();

    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    // build an access set from public keys
    let access_set = AccessSet::new(
        PUB_KEYS
            .iter()
            .map(|&k| PubKey::parse(k))
            .collect::<Vec<_>>(),
    );

    // parse our private key
    let my_key = PrivKey::parse(MY_PRIV_KEY);

    debug!("============================================================");

    // create a signal using this private key on some topic; this also includes building a STARK
    // proof attesting that the private key is in the access set, and that the nullifier contained
    // in the signal was built correctly.
    let now = Instant::now();
    let signal = access_set.make_signal(&my_key, TOPIC);
    debug!(
        "---------------------\nSignal created in {} ms",
        now.elapsed().as_millis()
    );

    // print out some stats about the proof
    debug!("{}", signal);
    debug!("---------------------");

    // the signal should be valid against this topic
    let now = Instant::now();
    match access_set.verify_signal(TOPIC, signal.clone()) {
        Ok(_) => debug!(
            "Signal verified in {:.1} ms",
            now.elapsed().as_micros() as f64 / 1000f64
        ),
        Err(err) => debug!("something went terribly wrong: {}", err),
    }
    debug!("============================================================");

    assert_ne!(
        signal.nullifier.to_bytes(),
        hex::decode("fa9f5e2287b26f5fc91643a65ecfebbf308c6230283cd5c2a6a57ffe8a60e19d").unwrap()
    );
}

// PUZZLE DESCRIPTION
// ================================================================================================

const PUZZLE_DESCRIPTION: &str = "\
Alice implemented a Semaphore protocol to collect anonymous votes from her friends on various
topics. She collected public keys from 7 of her friends, and together with her public key, built
an access set out of them.

During one of the votes, Alice collected 9 valid signals on the same topic. But that should not be
possible! The semaphore protocol guarantees that every user can vote only once on a given topic.
Someone must have figured out how to create multiple signals on the same topic.

Below is a transcript for generating a valid signal on a topic using your private key. Can you
figure out how to create a valid signal with a different nullifier on the same topic?
";
