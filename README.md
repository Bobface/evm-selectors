# evm-selectors

`evm-signatures` is a Rust library for working with known Ethereum Virtual Machine (EVM) function, error, event, and other selectors, with support for downloading the selector database from the [OpenChain API](https://docs.openchain.xyz/).

## Usage
Downloading the database
```rust
use evm_selectors::EvmSelectors;
use std::path::Path;

// Download and return as string
let data = EvmSelectors::download(None).await?;

// Download and write to tempfile.txt
EvmSelectors::download_to_file(Path::new("tempfile.txt"), None).await?;
```

Loading the database
```rust
use evm_selectors::EvmSelectors;
use std::path::Path;

// From a string
let data = "...".to_string();
let db = EvmSelectors::new_from_raw(&data)?;

// From a file
let db = EvmSelectors::new_from_file(Path::new("tempfile.txt"))?;
``` 

Querying selectors
```rust
use evm_selectors::EvmSelectors;
use std::path::Path;

// Query a single selector
let db = EvmSelectors::new_from_file(Path::new("tempfile.txt"))?;
let functions = db.get(&[0x00, 0x01, 0x02, 0x03].into());

// Get all available selectors
let all = db.items();
```

## Requirements
If the `download` feature is active (it is by default), a SSL/TLS library must be present. See the [reqwest documentation](https://github.com/seanmonstar/reqwest?tab=readme-ov-file#requirements) for further details.
