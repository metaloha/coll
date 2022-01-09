use crypto::digest::Digest;
use crypto::sha2::Sha256;
use letter_sequence::SequenceBuilder;
use num_format::{Locale, ToFormattedString};
use std::convert::TryFrom;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let target_hash = "a51b1fb1ca68e4b476c2bf2b3b6702a67880d75c7cd3f5c71bb8cf9d990d42c8";
    println!("Checking for '{}'", target_hash);

    let mut counter: u128 = 0;
    let mut last_length: u32 = 0;
    let mut seq = SequenceBuilder::try_from("a")
        .unwrap()
        .build()
        .unwrap();
    let mut sha = Sha256::new();

    loop {
        let test_content = seq.to_string();
        if last_length != test_content.chars().count() as u32 {
            println!("Length {} in {} seconds, {} iterations checked.", last_length.to_formatted_string(&Locale::en), now.elapsed().as_secs().to_formatted_string(&Locale::en), counter.to_formatted_string(&Locale::en));
            last_length = test_content.chars().count() as u32;
        }
        sha.input_str(test_content.as_str());
        if sha.result_str() == target_hash {
            // write test_content to a file
            println!("FOUND SHA256 COLLISION IN {} SECONDS, {} ITERATIONS!", now.elapsed().as_secs().to_formatted_string(&Locale::en), counter.to_formatted_string(&Locale::en));
            println!("{}", test_content.as_str());
            break;
        }
        counter = counter + 1;
        seq.next();
        sha.reset();
    }
}
