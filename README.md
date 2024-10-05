# Rust Wallet Key Manager

This project decodes Base58-encoded private and public keys, ensures they are in the correct format, and serializes them into a JSON file (`wallet.json`).

## Features

- Base58 decoding
- Keypair serialization as JSON

## Prerequisites

- Rust installed (https://www.rust-lang.org/tools/install)
- Dependencies: bs58 and serde_json (automatically handled via Cargo)

## Directory Structure:
Make sure your repository has a clean and organized folder structure:

```rust-wallet-key-manager-for-Eclipse-v2/
├── .gitignore
├── README.md
└── wallet_key_json/
    ├── Cargo.toml
    └── src/
        └── main.rs
```
        
## Running the Code

Clone the repository:

   ```bash
   git clone https://github.com/Skillz23/rust-wallet-key-manager-for-Eclipse-v2.git
   cd rust-wallet-key-manager-for-Eclipse-v2/wallet_key_json
```

 ## Install dependencies and run the project:

 To run the project, use Cargo:
    `
   cargo run
   `
 ## Replace the placeholder keys with actual Base58-encoded keys:

 In the file `src/main.rs`, replace the following placeholders with your actual Base58-encoded keys:
 ```rust
 let private_key_str = "your_base58_private_key_here";
 let public_key_str = "your_base58_public_key_here";
 ```
 ## Running the project:

 After running the project with `cargo run`, the program will create a file called `wallet.json`, which will contain the serialized keypair.

 ## Example:
 
 You can replace the placeholders with actual keys, for example:

 Private Key: `5p3fsFU7GvNJU5L26WQiqK2TUTs8nXyZ2L9Qqc8vVNA1XctwUqe`
 
 Public Key: `B2NpV8eA5WNSnH5F8KTRVNYvExg5AQkrtk`
 
 When you run the project, the resulting `wallet.json` file will look like this:
 ```json
[67, 98, 65, 213, 135, 20, ... ]
```

