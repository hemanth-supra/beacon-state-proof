# State Proof Fetcher

This library enables the arbitrary fetching of merkle inclusion proofs for any field of the Ethereum beacon chain state. Please be aware that this is a very heavy operation, as it requires downloading the entire beacon state at the given slot, which can be several 100s of MBs in size.

## Features

- Fetch beacon state at a given slot.
- Compute Merkle proofs for specific indices within the state.
- Asynchronous API using `tokio`.
- Custom error handling for easier debugging.

## Installation

Add this to your `Cargo.toml`: 

```toml
[dependencies]

beacon-state-proof = { git = "https://github.com/petscheit/beacon-state-proof" }
```


## Usage

### Fetch a state proof based on the given slot and index

```rust
use beacon_state_proof::state_proof_fetcher::StateProofFetcher;

#[tokio::main]
async fn main() {
    let fetcher = StateProofFetcher::new("BEACON_NODE_RPC_URL".to_string());
    match fetcher.fetch_state_proof(6408035, 55).await {
        Ok(proof) => println!("Proof: {:?}", proof),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Fetch next sync committee proof based on the given slot

```rust
use beacon_state_proof::state_proof_fetcher::{StateProofFetcher, SyncCommitteeProof};

#[tokio::main]
async fn main() {
    let fetcher = StateProofFetcher::new("http://127.0.0.1:5052".to_string());
    let proof: SyncCommitteeProof = match fetcher.fetch_next_sync_committee_proof(6408035).await {
        Ok(proof) => proof,
        Err(e) => {
            println!("Error fetching state proof: {:?}", e);
            return;
        }
    };

    println!("{:?}", proof.leaf);
} 