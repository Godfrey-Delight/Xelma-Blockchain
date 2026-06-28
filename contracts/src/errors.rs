//! Contract error types for the XLM Price Prediction Market.

use soroban_sdk::{xdr::ScErrorType, Error};

/// Contract error types
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Contract has already been initialized
    AlreadyInitialized = 1,
    /// Admin address not set - call initialize first
    AdminNotSet = 2,
    /// Oracle address not set - call initialize first
    OracleNotSet = 3,
    /// Only admin can perform this action
    UnauthorizedAdmin = 4,
    /// Only oracle can perform this action
    UnauthorizedOracle = 5,
    /// Bet amount must be greater than zero
    InvalidBetAmount = 6,
    /// No active round exists
    NoActiveRound = 7,
    /// Round has already ended
    RoundEnded = 8,
    /// User has insufficient balance
    InsufficientBalance = 9,
    /// User has already placed a bet in this round
    AlreadyBet = 10,
    /// Arithmetic overflow occurred
    Overflow = 11,
    /// Invalid price value
    InvalidPrice = 12,
    /// Invalid duration value
    InvalidDuration = 13,
    /// Invalid round mode (must be 0 or 1)
    InvalidMode = 14,
    /// Wrong prediction type for current round mode
    WrongModeForPrediction = 15,
    /// Round has not reached end_ledger yet
    RoundNotEnded = 16,
    /// Invalid price scale (must represent 4 decimal places)
    InvalidPriceScale = 17,
    /// Oracle data is too old (STALE)
    StaleOracleData = 18,
    /// Oracle payload round_id doesn't match ActiveRound
    InvalidOracleRound = 19,
    /// An active round already exists and cannot be overwritten
    RoundAlreadyActive = 20,
    /// Admin and Oracle addresses cannot be identical
    AdminIsOracle = 21,
    /// Contract is paused for emergency recovery
    ContractPaused = 22,
    /// One or more window values exceed configured maximum bounds
    WindowOutOfRange = 23,
    /// Oracle payload timestamp is in the future
    FutureOracleData = 24,
    /// Arithmetic overflow in payout accumulation — no funds moved
    PayoutOverflow = 25,
    /// Round has been cancelled and cannot be resolved
    RoundCancelled = 26,
    /// Round cannot be cancelled (no active round or already resolved)
    RoundNotCancellable = 27,
    /// Bet amount exceeds the configured maximum stake
    StakeExceedsMax = 28,
    /// User's cumulative exposure in this round exceeds the configured cap
    ExposureCapExceeded = 29,
    /// Pending winnings accumulation would exceed the configured cap
    PendingWinningsCapExceeded = 30,
    /// Start price is below the minimum allowed value
    StartPriceTooLow = 31,
    /// Start price exceeds the maximum allowed value
    StartPriceTooHigh = 32,
    /// Oracle payload nonce was already consumed for this round (replay)
    OracleNonceReused = 33,
    /// Round has fewer participants than the configured minimum for competitive settlement
    InsufficientParticipants = 34,
    /// Minimum participants value is out of valid range (must be 1–10000)
    InvalidMinParticipants = 35,
    /// Oracle heartbeat status is out of range (must be 0, 1, or 2)
    InvalidOracleStatus = 36,
    /// Oracle stale threshold is out of valid range (must be 60–86400 seconds)
    InvalidStaleThreshold = 37,
    /// Precision participant cap is out of range (must be 1–10000)
    InvalidPrecisionParticipantCap = 38,
    /// Precision round has reached the configured participant cap
    PrecisionParticipantCapExceeded = 39,
    /// Oracle max deviation bps is invalid (must be > 0)
    InvalidOracleDeviationBps = 40,
    /// Oracle final price deviates beyond configured threshold
    OracleDeviationExceeded = 41,
    /// Stored schema version is unknown or unsupported by this contract build
    UnsupportedSchemaVersion = 42,
    /// Migration path is invalid for the stored schema version
    InvalidMigrationPath = 43,
    /// Migration cannot run while a round is active
    MigrationActiveRound = 44,
    /// Commitment for precision prediction not found
    CommitmentNotFound = 45,
    /// Precision prediction has already been revealed
    AlreadyRevealed = 46,
    /// Attempted to reveal prediction outside the valid window
    InvalidRevealWindow = 47,
    /// Revealed prediction hash does not match committed hash
    HashMismatch = 48,
    /// Oracle payload network_id does not match the runtime network
    OracleNetworkMismatch = 49,
    /// Oracle payload contract_addr does not match the current contract
    OracleContractMismatch = 50,
    /// Protocol fee bps is outside the allowed range (must be in `1..=MAX_PROTOCOL_FEE_BPS`)
    InvalidProtocolFeeBps = 51,
    /// Treasury withdrawal would underflow the accumulated treasury balance
    FeeTreasuryUnderflow = 52,
}

impl From<ContractError> for Error {
    fn from(e: ContractError) -> Self {
        Error::from_contract_error(e as u32)
    }
}

impl From<&ContractError> for Error {
    fn from(e: &ContractError) -> Self {
        Error::from_contract_error(*e as u32)
    }
}

fn to_contract_error(code: u32) -> Option<ContractError> {
    match code {
        1 => Some(ContractError::AlreadyInitialized),
        2 => Some(ContractError::AdminNotSet),
        3 => Some(ContractError::OracleNotSet),
        4 => Some(ContractError::UnauthorizedAdmin),
        5 => Some(ContractError::UnauthorizedOracle),
        6 => Some(ContractError::InvalidBetAmount),
        7 => Some(ContractError::NoActiveRound),
        8 => Some(ContractError::RoundEnded),
        9 => Some(ContractError::InsufficientBalance),
        10 => Some(ContractError::AlreadyBet),
        11 => Some(ContractError::Overflow),
        12 => Some(ContractError::InvalidPrice),
        13 => Some(ContractError::InvalidDuration),
        14 => Some(ContractError::InvalidMode),
        15 => Some(ContractError::WrongModeForPrediction),
        16 => Some(ContractError::RoundNotEnded),
        17 => Some(ContractError::InvalidPriceScale),
        18 => Some(ContractError::StaleOracleData),
        19 => Some(ContractError::InvalidOracleRound),
        20 => Some(ContractError::RoundAlreadyActive),
        21 => Some(ContractError::AdminIsOracle),
        22 => Some(ContractError::ContractPaused),
        23 => Some(ContractError::WindowOutOfRange),
        24 => Some(ContractError::FutureOracleData),
        25 => Some(ContractError::PayoutOverflow),
        26 => Some(ContractError::RoundCancelled),
        27 => Some(ContractError::RoundNotCancellable),
        28 => Some(ContractError::StakeExceedsMax),
        29 => Some(ContractError::ExposureCapExceeded),
        30 => Some(ContractError::PendingWinningsCapExceeded),
        31 => Some(ContractError::StartPriceTooLow),
        32 => Some(ContractError::StartPriceTooHigh),
        33 => Some(ContractError::OracleNonceReused),
        34 => Some(ContractError::InsufficientParticipants),
        35 => Some(ContractError::InvalidMinParticipants),
        36 => Some(ContractError::InvalidOracleStatus),
        37 => Some(ContractError::InvalidStaleThreshold),
        38 => Some(ContractError::InvalidPrecisionParticipantCap),
        39 => Some(ContractError::PrecisionParticipantCapExceeded),
        40 => Some(ContractError::InvalidOracleDeviationBps),
        41 => Some(ContractError::OracleDeviationExceeded),
        42 => Some(ContractError::UnsupportedSchemaVersion),
        43 => Some(ContractError::InvalidMigrationPath),
        44 => Some(ContractError::MigrationActiveRound),
        45 => Some(ContractError::CommitmentNotFound),
        46 => Some(ContractError::AlreadyRevealed),
        47 => Some(ContractError::InvalidRevealWindow),
        48 => Some(ContractError::HashMismatch),
        49 => Some(ContractError::OracleNetworkMismatch),
        50 => Some(ContractError::OracleContractMismatch),
        51 => Some(ContractError::InvalidProtocolFeeBps),
        52 => Some(ContractError::FeeTreasuryUnderflow),
        _ => None,
    }
}

impl TryFrom<&Error> for ContractError {
    type Error = Error;
    fn try_from(e: &Error) -> Result<Self, Self::Error> {
        if e.is_type(ScErrorType::Contract) {
            to_contract_error(e.get_code()).ok_or(*e)
        } else {
            Err(*e)
        }
    }
}

impl TryFrom<Error> for ContractError {
    type Error = Error;
    fn try_from(e: Error) -> Result<Self, Self::Error> {
        if e.is_type(ScErrorType::Contract) {
            to_contract_error(e.get_code()).ok_or(e)
        } else {
            Err(e)
        }
    }
}
