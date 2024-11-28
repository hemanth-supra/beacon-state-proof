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