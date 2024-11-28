use crate::error::Error;
use crate::rpc::fetch_beacon_state;
use types::{Hash256, MainnetEthSpec, SyncCommittee};
pub use types::beacon_state::TreeHash;
use serde::{Serialize, Deserialize};

/// A fetcher for obtaining state proofs from a beacon node.
pub struct StateProofFetcher {
    rpc_endpoint: String,
}

impl StateProofFetcher {
    /// Creates a new `StateProofFetcher` with the given RPC endpoint.
    pub fn new(rpc_endpoint: String) -> Self {
        Self { rpc_endpoint }
    }

    /// Fetches the state proof for the given slot and index.
    ///
    /// # Arguments
    ///
    /// * `slot` - The slot number of the beacon state.
    /// * `index` - The index within the state for which to compute the Merkle proof.
    ///
    /// # Returns
    ///
    /// A vector of `Hash256` representing the Merkle proof.
    pub async fn fetch_state_proof(&self, slot: u64, index: usize) -> Result<Vec<Hash256>, Error> {
        let state = fetch_beacon_state(&self.rpc_endpoint, slot).await?;

        state
            .compute_merkle_proof(index)
            .map_err(Error::BeaconStateError)
    }

    /// Fetches a proof for the next sync committee from the beacon state.
    ///
    /// This method retrieves the beacon state for the given slot and computes a Merkle proof
    /// for the next sync committee at index 55 in the state tree. The proof can be used to
    /// verify the authenticity of the sync committee without requiring the full beacon state.
    ///
    /// # Arguments
    ///
    /// * `slot` - The slot number of the beacon state to fetch the proof from
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// * `SyncCommitteeProof` - A struct containing:
    ///   * The Merkle proof for the sync committee
    ///   * The next sync committee data
    ///   * The index (55) of the sync committee in the state tree
    /// * `Error` - If the beacon state cannot be fetched or proof computation fails
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The beacon state cannot be fetched from the RPC endpoint
    /// * The Merkle proof computation fails
    /// * The next sync committee cannot be retrieved from the state
    pub async fn fetch_next_sync_committee_proof(&self, slot: u64) -> Result<SyncCommitteeProof, Error> {
        let state = fetch_beacon_state(&self.rpc_endpoint, slot).await?;
        let proof = state.compute_merkle_proof(55)
            .map_err(Error::BeaconStateError)?;

        let next_sync_committee = state.next_sync_committee()
            .map_err(Error::BeaconStateError)?.as_ref().clone();

        let leaf = next_sync_committee.tree_hash_root();

        Ok(SyncCommitteeProof { proof, next_sync_committee, index: 55, leaf, slot })
    }
}


/// A struct representing a sync committee proof.
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncCommitteeProof {
    /// The Merkle proof for the sync committee.
    pub proof: Vec<Hash256>,
    /// The next sync committee.
    pub next_sync_committee: SyncCommittee<MainnetEthSpec>,
    /// The index of the sync committee in the state.
    pub index: usize,
    /// The leaf of the sync committee. This is the ssz root of the sync committee container.
    pub leaf: Hash256,
    /// The slot of the beacon state.
    pub slot: u64,
}
