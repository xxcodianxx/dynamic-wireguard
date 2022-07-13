use std::io::{stdout, Write};
use sha1::{Digest, Sha1};
use colored::*;

pub fn ask_user_to_verify_fingerprint(public_key: &[u8]) -> bool {
    // compute fingerprint
    let mut hasher = Sha1::new();
    hasher.update(public_key);

    let fingerprint = hex::encode_upper(hasher.finalize())
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 2 == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();

    // let randomart = BishopArt::new()
    //     .chain(public_key)
    //     .draw_with_opts(&DrawingOptions {
    //         top_text: "X25519".to_string(),
    //         ..Default::default()
    //     })
    //     .replace("\n", "\n    ");

    println!( //Randomart of remote public key:\n\n    {}\n
        "Fingerprint of remote public key:\n\n    {}\n",
        fingerprint.bold()
    );

    println!(
        "{} Please verify that this fingerprint matches the remote's real\n         \
                  fingerprint as it will be used to ensure the identity\n         \
                  of the server in the future.\n",
        "WARNING:".bright_yellow().bold()
    );

    print!("Is this correct? <y/N>: ");
    stdout().flush().unwrap_or(());

    let mut decision = String::new();
    std::io::stdin().read_line(&mut decision).unwrap();

    return decision.to_lowercase() == "y\n";
}