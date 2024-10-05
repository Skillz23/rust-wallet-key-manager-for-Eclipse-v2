use std::fs::File;
use std::io::Write;
use bs58;

fn main() {
    let private_key_str = "your_base58_private_key_here";
    let public_key_str = "your_base58_public_key_here";

    let mut private_key_bytes = bs58::decode(private_key_str).into_vec().expect("Invalid private key format");
    let public_key_bytes = bs58::decode(public_key_str).into_vec().expect("Invalid public key format");

    if private_key_bytes.len() == 64 {
        private_key_bytes = private_key_bytes[0..32].to_vec();
    }

    assert_eq!(private_key_bytes.len(), 32, "Private key must be 32 bytes");
    assert_eq!(public_key_bytes.len(), 32, "Public key must be 32 bytes");

    let mut keypair = Vec::new();
    keypair.extend_from_slice(&private_key_bytes);
    keypair.extend_from_slice(&public_key_bytes);

    let json_content = serde_json::to_string(&keypair).unwrap();

    let mut file = File::create("wallet.json").unwrap();
    file.write_all(json_content.as_bytes()).unwrap();

    println!("wallet.json file created with the keys in the correct format.");
}
