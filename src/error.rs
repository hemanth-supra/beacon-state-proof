use types::beacon_state::Error as BeaconStateError;

/// Custom error type for the state proof fetcher.
#[derive(Debug)]
pub enum Error {
    /// Error from the `reqwest` HTTP client.
    ReqwestError(reqwest::Error),
    /// Error decoding SSZ bytes.
    SszError,
    /// Missing `Eth-Consensus-Version` header in the response.
    MissingConsensusVersion,
    /// Invalid consensus version received.
    InvalidConsensusVersion,
    /// Error from the beacon state computations.
    BeaconStateError(BeaconStateError),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
} 