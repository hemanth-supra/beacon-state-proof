use crate::error::Error;
use reqwest::header::ACCEPT;
use types::beacon_state::BeaconState;
use types::beacon_block::BeaconBlock;
use types::eth_spec::MainnetEthSpec;
use types::fork_name::ForkName;

/// Fetches the beacon state from the given RPC endpoint and slot.
///
/// # Arguments
///
/// * `url` - The RPC endpoint URL.
/// * `slot` - The slot number of the beacon state.
///
/// # Returns
///
/// A `BeaconState` instance for the given slot.
pub async fn fetch_beacon_state(
    url: &str,
    slot: u64,
) -> Result<BeaconState<MainnetEthSpec>, Error> {
    let url = format!("{}/eth/v2/debug/beacon/states/{}", url, slot);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await?;

    let consensus_version = response
        .headers()
        .get("Eth-Consensus-Version")
        .and_then(|h| h.to_str().ok())
        .ok_or(Error::MissingConsensusVersion)?;

    let capitalized = consensus_version[..1].to_uppercase() + &consensus_version[1..];
    let fork_name = ForkName::try_from(capitalized).map_err(|_| Error::InvalidConsensusVersion)?;

    let bytes = response.bytes().await?;

    let state = BeaconState::from_fork_ssz_bytes(bytes.as_ref(), fork_name)
        .map_err(|_| Error::SszError)?;

    Ok(state)
} 

pub async fn fetch_beacon_block(
    url: &str,
    head: u64,
) -> Result<BeaconBlock<MainnetEthSpec>, Error> {
    // https://lodestar-mainnet.chainsafe.io/eth/v2/beacon/blocks/head
    let url = format!("{}/eth/v2/beacon/blocks/{}", url, head);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await?;

    let consensus_version = response
        .headers()
        .get("Eth-Consensus-Version")
        .and_then(|h| h.to_str().ok())
        .ok_or(Error::MissingConsensusVersion)?;

    let capitalized = consensus_version[..1].to_uppercase() + &consensus_version[1..];
    let fork_name = ForkName::try_from(capitalized).map_err(|_| Error::InvalidConsensusVersion)?;

    let bytes = response.bytes().await?;

    let beaco_block: BeaconBlock<MainnetEthSpec> = BeaconBlock::from_ssz_bytes_for_fork(bytes.as_ref(), fork_name)
        .map_err(|_| Error::SszError)?;

    Ok(beaco_block)
} 