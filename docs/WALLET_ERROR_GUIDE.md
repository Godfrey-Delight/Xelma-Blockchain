# Wallet Error Integration & UX Guide

This guide maps every smart-contract error defined in `contracts/src/errors.rs` (`ContractError` enum) to consumer-facing UX copy, technical explanations, and wallet integration patterns (e.g., Freighter, Albedo, xBull, and custom dApp frontends).

---

## Error Summary Reference Table

| Hex | Dec | Enum Identifier | Technical Meaning | User-Facing Message (UX Copy) | Action / Recovery |
|:---:|:---:|:---|:---|:---|:---|
| `0x01` | 1 | `AlreadyInitialized` | Contract has already been initialized | "This contract is already initialized." | Do not call initialize again; interact with the active contract. |
| `0x02` | 2 | `AdminNotSet` | Admin address not set in storage | "Contract admin is not configured." | Initialize the contract before running admin operations. |
| `0x03` | 3 | `OracleNotSet` | Oracle address not set in storage | "Oracle address is not configured." | Initialize the contract with a designated oracle address. |
| `0x04` | 4 | `UnauthorizedAdmin` | Caller is not the configured admin | "Admin privileges required. Please connect with the admin wallet." | Switch to the authorized admin wallet account. |
| `0x05` | 5 | `UnauthorizedOracle` | Caller is not the configured oracle | "Oracle privileges required. This action is restricted to the authorized oracle." | Submit from the designated oracle signing address. |
| `0x06` | 6 | `InvalidBetAmount` | Bet amount must be greater than zero | "Bet amount must be greater than 0." | Enter a positive bet stake. |
| `0x07` | 7 | `NoActiveRound` | No active round currently exists | "There is currently no active round." | Wait for an admin to start a new round. |
| `0x08` | 8 | `RoundEnded` | Round betting window or lifetime has ended | "Betting for this round has closed." | Wait for the next round to open for betting. |
| `0x09` | 9 | `InsufficientBalance` | User balance is lower than requested stake | "Insufficient vXLM balance to place this bet." | Mint initial test tokens or claim pending winnings. |
| `0x0a` | 10 | `AlreadyBet` | User already placed a bet in this round | "You have already placed a bet in this round." | Wait for the next round to place another bet. |
| `0x0b` | 11 | `Overflow` | Arithmetic overflow occurred | "Calculation overflow encountered. Please try a smaller amount." | Reduce stake or numeric inputs. |
| `0x0c` | 12 | `InvalidPrice` | Price value is non-positive or invalid | "Invalid price value provided." | Provide a valid, non-zero price. |
| `0x0d` | 13 | `InvalidDuration` | Duration or window ledgers are invalid | "Invalid round duration configured." | Set positive ledgers where bet window < run window. |
| `0x0e` | 14 | `InvalidMode` | Round mode is not 0 (Up/Down) or 1 (Precision) | "Invalid round mode. Supported modes are Up/Down (0) or Precision (1)." | Use a supported mode identifier. |
| `0x0f` | 15 | `WrongModeForPrediction` | Prediction method does not match round mode | "Prediction type does not match the active round mode." | Submit directional bets to Up/Down rounds and exact guesses to Precision rounds. |
| `0x10` | 16 | `RoundNotEnded` | Round has not reached `end_ledger` yet | "The round has not reached its end ledger yet." | Wait until the round reaches `end_ledger` before resolving. |
| `0x11` | 17 | `InvalidPriceScale` | Price does not adhere to 4 decimal places | "Price scale is invalid. Price must be scaled to 4 decimal places." | Format prices with 4 decimals (e.g. 0.2297 → 2297). |
| `0x12` | 18 | `StaleOracleData` | Oracle payload timestamp exceeds staleness limit | "Oracle data is stale. Please submit fresh price data." | Publish recent oracle data within the active staleness window. |
| `0x13` | 19 | `InvalidOracleRound` | Payload `round_id` does not match active round | "Oracle payload round ID does not match the active round." | Sign and submit oracle data matching the current active round ID. |
| `0x14` | 20 | `RoundAlreadyActive` | An active round already exists | "An active round already exists. Settle or cancel it before starting a new round." | Complete or cancel the running round first. |
| `0x15` | 21 | `AdminIsOracle` | Admin and Oracle addresses are identical | "Admin and Oracle cannot be the same address." | Configure separate addresses for Admin and Oracle roles. |
| `0x16` | 22 | `ContractPaused` | Contract is paused for emergency recovery | "The contract is currently paused for maintenance." | Operations resume once admin unpauses the contract. |
| `0x17` | 23 | `WindowOutOfRange` | Window parameters exceed configured bounds | "Window duration values exceed allowable limits." | Provide ledger windows within configured protocol limits. |
| `0x18` | 24 | `FutureOracleData` | Oracle payload timestamp is ahead of current time | "Oracle timestamp is in the future. Check oracle clock synchronization." | Resynchronize oracle clock to ledger timestamp. |
| `0x19` | 25 | `PayoutOverflow` | Arithmetic overflow in payout accumulation | "Payout calculation overflow. Settlement aborted safely." | Verify pool sizes and winnings caps. |
| `0x1a` | 26 | `RoundCancelled` | Round was cancelled and cannot be resolved | "This round has been cancelled and cannot be resolved." | Cancelled rounds refund stakes; no settlement allowed. |
| `0x1b` | 27 | `RoundNotCancellable` | No active round or round already resolved | "Round cannot be cancelled because no active round exists." | Only currently active rounds can be cancelled. |
| `0x1c` | 28 | `StakeExceedsMax` | Bet amount exceeds maximum stake cap | "Bet amount exceeds the maximum allowable stake limit." | Reduce bet amount to be within `max_stake`. |
| `0x1d` | 29 | `ExposureCapExceeded` | Cumulative stake exceeds user round exposure cap | "Your total stake for this round exceeds the exposure cap." | Lower cumulative stake for this round. |
| `0x1e` | 30 | `PendingWinningsCapExceeded` | Claimable winnings exceed configured limit | "Pending winnings limit reached. Claim existing winnings before accumulating more." | Call `claim_winnings` to withdraw accumulated funds. |
| `0x1f` | 31 | `StartPriceTooLow` | Round start price is below allowable minimum | "Start price is below the minimum allowed price threshold." | Increase start price above configured minimum. |
| `0x20` | 32 | `StartPriceTooHigh` | Round start price exceeds allowable maximum | "Start price exceeds the maximum allowed price threshold." | Lower start price below configured maximum. |
| `0x21` | 33 | `OracleNonceReused` | Oracle payload nonce was previously consumed | "Oracle nonce has already been used. Please submit with a fresh nonce." | Supply a unique, incremented or random nonce. |
| `0x22` | 34 | `InsufficientParticipants` | Round has fewer participants than minimum | "Insufficient participants for competitive settlement; round will be refunded." | The contract refunds all stakes when participant threshold is unmet. |
| `0x23` | 35 | `InvalidMinParticipants` | Minimum participants value outside valid range | "Minimum participants setting must be between 1 and 10,000." | Configure participant threshold in the range `1..=10000`. |
| `0x24` | 36 | `InvalidOracleStatus` | Heartbeat status is not 0, 1, or 2 | "Invalid oracle heartbeat status code (must be 0, 1, or 2)." | Heartbeat status must be 0 (active), 1 (degraded), or 2 (offline). |
| `0x25` | 37 | `InvalidStaleThreshold` | Oracle stale threshold outside valid range | "Oracle stale threshold must be between 60 and 86,400 seconds (1 min to 24 hrs)." | Provide threshold between 60 and 86,400 seconds. |
| `0x26` | 38 | `InvalidPrecisionParticipantCap` | Precision participant cap outside valid range | "Precision participant cap must be between 1 and 10,000." | Set cap within `1..=10000`. |
| `0x27` | 39 | `PrecisionParticipantCapExceeded` | Precision round reached participant limit | "Participant capacity for this precision round is full." | Wait for the next round to place a precision prediction. |
| `0x28` | 40 | `InvalidOracleDeviationBps` | Deviation basis points invalid (must be > 0) | "Oracle max deviation basis points must be greater than 0." | Set deviation bps > 0 or `None` to disable. |
| `0x29` | 41 | `OracleDeviationExceeded` | Final price deviates beyond configured tolerance | "Oracle price deviation exceeds allowed limits. Settlement blocked." | Settle within deviation bounds or arm admin override. |
| `0x2a` | 42 | `UnsupportedSchemaVersion` | Stored schema version is unsupported | "Unsupported storage schema version. Please upgrade contract or client." | Verify contract version and run appropriate migration. |
| `0x2b` | 43 | `InvalidMigrationPath` | Migration called for invalid source schema | "Invalid migration path for the current storage schema version." | Execute migration only from supported source schema. |
| `0x2c` | 44 | `MigrationActiveRound` | Migration attempted while round is active | "Cannot migrate schema while a round is active. Complete or cancel the round first." | Resolve or cancel active round before running schema migration. |
| `0x2d` | 45 | `CommitmentNotFound` | No commitment found for user in precision round | "No committed prediction found for this account in the current round." | Ensure `commit_prediction` succeeded during the commit phase. |
| `0x2e` | 46 | `AlreadyRevealed` | Prediction has already been revealed | "You have already revealed your prediction for this round." | Commitments can only be revealed once. |
| `0x2f` | 47 | `InvalidRevealWindow` | Reveal attempted outside allowed reveal window | "Reveal period is not open. Predictions can only be revealed during the designated reveal window." | Reveal after `bet_end_ledger` and before round resolution. |
| `0x30` | 48 | `HashMismatch` | Revealed price + salt does not match commitment | "Revealed price and salt do not match your committed hash." | Verify the exact guessed price and salt used at commit time. |
| `0x31` | 49 | `OracleNetworkMismatch` | Oracle payload network ID does not match network | "Oracle payload network ID does not match the active network." | Ensure oracle targets current network (Testnet/Mainnet). |
| `0x32` | 50 | `OracleContractMismatch` | Oracle payload contract address mismatch | "Oracle payload target contract does not match this contract address." | Target the exact deployed contract ID in oracle payload. |

---

## Domain Categorization

### 1. Initialization & Role Access Control
* **`AlreadyInitialized` (1)**: Contract is already initialized.
* **`AdminNotSet` (2)** & **`OracleNotSet` (3)**: Uninitialized contract state.
* **`UnauthorizedAdmin` (4)** & **`UnauthorizedOracle` (5)**: Unauthorized caller attempted a privileged function.
* **`AdminIsOracle` (21)**: Separation of duties violation during initialization.

### 2. Round Lifecycle & State Guardrails
* **`NoActiveRound` (7)**: Call requires an active round, but none exists.
* **`RoundEnded` (8)**: Betting closed or round reached terminal state.
* **`RoundNotEnded` (16)**: Premature resolution attempt.
* **`RoundAlreadyActive` (20)**: Cannot overwrite active round.
* **`ContractPaused` (22)**: Emergency circuit-breaker active.
* **`RoundCancelled` (26)** & **`RoundNotCancellable` (27)**: Round cancellation rules.

### 3. User Betting & Precision Commit-Reveal
* **`InvalidBetAmount` (6)**: Non-positive stake.
* **`InsufficientBalance` (9)**: Account balance insufficient.
* **`AlreadyBet` (10)**: One position per user per round.
* **`InvalidMode` (14)** & **`WrongModeForPrediction` (15)**: Mode mismatches.
* **`PrecisionParticipantCapExceeded` (39)**: Max players reached in precision round.
* **`CommitmentNotFound` (45)**, **`AlreadyRevealed` (46)**, **`InvalidRevealWindow` (47)**, **`HashMismatch` (48)**: Commit-reveal security checks.

### 4. Risk, Bounds & Exposure Controls
* **`WindowOutOfRange` (23)** & **`InvalidDuration` (13)**: Ledger timing parameters.
* **`StakeExceedsMax` (28)**: Stake amount cap per bet.
* **`ExposureCapExceeded` (29)**: Cumulative user round exposure cap.
* **`PendingWinningsCapExceeded` (30)**: Max claimable balance cap.
* **`StartPriceTooLow` (31)** & **`StartPriceTooHigh` (32)**: Price bounds on round creation.
* **`InsufficientParticipants` (34)** & **`InvalidMinParticipants` (35)**: Competitive threshold.
* **`InvalidPrecisionParticipantCap` (38)**: Bounds check on precision cap.

### 5. Oracle Liveness & Settlement Verification
* **`StaleOracleData` (18)** & **`InvalidStaleThreshold` (37)**: Freshness rules.
* **`InvalidOracleRound` (19)**: Mismatched round ID.
* **`FutureOracleData` (24)**: Future timestamp rejection.
* **`OracleNonceReused` (33)**: Replay attack protection.
* **`InvalidOracleStatus` (36)**: Liveness status range validation.
* **`InvalidOracleDeviationBps` (40)** & **`OracleDeviationExceeded` (41)**: Settlement price circuit-breaker.
* **`OracleNetworkMismatch` (49)** & **`OracleContractMismatch` (50)**: Cross-network / cross-contract domain separation.

### 6. Storage & Migration Safety
* **`UnsupportedSchemaVersion` (42)**: Unrecognized storage schema.
* **`InvalidMigrationPath` (43)**: Unsupported migration path.
* **`MigrationActiveRound` (44)**: Migration blocked during active round.

---

## Integration Walkthroughs

### 1. Decoding Errors in TypeScript (Soroban SDK / Wallet)

```typescript
export interface ErrorDetail {
  code: number;
  name: string;
  userMessage: string;
}

export const CONTRACT_ERRORS: Record<number, { name: string; userMessage: string }> = {
  1: { name: "AlreadyInitialized", userMessage: "This contract is already initialized." },
  2: { name: "AdminNotSet", userMessage: "Contract admin is not configured." },
  3: { name: "OracleNotSet", userMessage: "Oracle address is not configured." },
  4: { name: "UnauthorizedAdmin", userMessage: "Admin privileges required. Please connect with the admin wallet." },
  5: { name: "UnauthorizedOracle", userMessage: "Oracle privileges required. This action is restricted to the authorized oracle." },
  6: { name: "InvalidBetAmount", userMessage: "Bet amount must be greater than 0." },
  7: { name: "NoActiveRound", userMessage: "There is currently no active round." },
  8: { name: "RoundEnded", userMessage: "Betting for this round has closed." },
  9: { name: "InsufficientBalance", userMessage: "Insufficient vXLM balance to place this bet." },
  10: { name: "AlreadyBet", userMessage: "You have already placed a bet in this round." },
  11: { name: "Overflow", userMessage: "Calculation overflow encountered. Please try a smaller amount." },
  12: { name: "InvalidPrice", userMessage: "Invalid price value provided." },
  13: { name: "InvalidDuration", userMessage: "Invalid round duration configured." },
  14: { name: "InvalidMode", userMessage: "Invalid round mode. Supported modes are Up/Down (0) or Precision (1)." },
  15: { name: "WrongModeForPrediction", userMessage: "Prediction type does not match the active round mode." },
  16: { name: "RoundNotEnded", userMessage: "The round has not reached its end ledger yet." },
  17: { name: "InvalidPriceScale", userMessage: "Price scale is invalid. Price must be scaled to 4 decimal places." },
  18: { name: "StaleOracleData", userMessage: "Oracle data is stale. Please submit fresh price data." },
  19: { name: "InvalidOracleRound", userMessage: "Oracle payload round ID does not match the active round." },
  20: { name: "RoundAlreadyActive", userMessage: "An active round already exists. Settle or cancel it before starting a new round." },
  21: { name: "AdminIsOracle", userMessage: "Admin and Oracle cannot be the same address." },
  22: { name: "ContractPaused", userMessage: "The contract is currently paused for maintenance." },
  23: { name: "WindowOutOfRange", userMessage: "Window duration values exceed allowable limits." },
  24: { name: "FutureOracleData", userMessage: "Oracle timestamp is in the future. Check oracle clock synchronization." },
  25: { name: "PayoutOverflow", userMessage: "Payout calculation overflow. Settlement aborted safely." },
  26: { name: "RoundCancelled", userMessage: "This round has been cancelled and cannot be resolved." },
  27: { name: "RoundNotCancellable", userMessage: "Round cannot be cancelled because no active round exists." },
  28: { name: "StakeExceedsMax", userMessage: "Bet amount exceeds the maximum allowable stake limit." },
  29: { name: "ExposureCapExceeded", userMessage: "Your total stake for this round exceeds the exposure cap." },
  30: { name: "PendingWinningsCapExceeded", userMessage: "Pending winnings limit reached. Claim existing winnings before accumulating more." },
  31: { name: "StartPriceTooLow", userMessage: "Start price is below the minimum allowed price threshold." },
  32: { name: "StartPriceTooHigh", userMessage: "Start price exceeds the maximum allowed price threshold." },
  33: { name: "OracleNonceReused", userMessage: "Oracle nonce has already been used. Please submit with a fresh nonce." },
  34: { name: "InsufficientParticipants", userMessage: "Insufficient participants for competitive settlement; round will be refunded." },
  35: { name: "InvalidMinParticipants", userMessage: "Minimum participants setting must be between 1 and 10,000." },
  36: { name: "InvalidOracleStatus", userMessage: "Invalid oracle heartbeat status code (must be 0, 1, or 2)." },
  37: { name: "InvalidStaleThreshold", userMessage: "Oracle stale threshold must be between 60 and 86,400 seconds (1 min to 24 hrs)." },
  38: { name: "InvalidPrecisionParticipantCap", userMessage: "Precision participant cap must be between 1 and 10,000." },
  39: { name: "PrecisionParticipantCapExceeded", userMessage: "Participant capacity for this precision round is full." },
  40: { name: "InvalidOracleDeviationBps", userMessage: "Oracle max deviation basis points must be greater than 0." },
  41: { name: "OracleDeviationExceeded", userMessage: "Oracle price deviation exceeds allowed limits. Settlement blocked." },
  42: { name: "UnsupportedSchemaVersion", userMessage: "Unsupported storage schema version. Please upgrade contract or client." },
  43: { name: "InvalidMigrationPath", userMessage: "Invalid migration path for the current storage schema version." },
  44: { name: "MigrationActiveRound", userMessage: "Cannot migrate schema while a round is active. Complete or cancel the round first." },
  45: { name: "CommitmentNotFound", userMessage: "No committed prediction found for this account in the current round." },
  46: { name: "AlreadyRevealed", userMessage: "You have already revealed your prediction for this round." },
  47: { name: "InvalidRevealWindow", userMessage: "Reveal period is not open. Predictions can only be revealed during the designated reveal window." },
  48: { name: "HashMismatch", userMessage: "Revealed price and salt do not match your committed hash." },
  49: { name: "OracleNetworkMismatch", userMessage: "Oracle payload network ID does not match the active network." },
  50: { name: "OracleContractMismatch", userMessage: "Oracle payload target contract does not match this contract address." },
};

export function decodeContractError(rawError: unknown): ErrorDetail {
  let code: number | undefined;

  // Extract Soroban error code from RPC / SDK simulation error structure
  if (typeof rawError === "object" && rawError !== null) {
    const errObj = rawError as Record<string, any>;
    code =
      errObj.code ??
      errObj.result?.xdr?.value?.val?.code ??
      errObj.simulation?.error?.code ??
      errObj.error?.code;
  }

  if (typeof code === "number" && CONTRACT_ERRORS[code]) {
    return {
      code,
      name: CONTRACT_ERRORS[code].name,
      userMessage: CONTRACT_ERRORS[code].userMessage,
    };
  }

  return {
    code: typeof code === "number" ? code : 0,
    name: "UnknownError",
    userMessage: "An unexpected transaction error occurred. Please try again.",
  };
}
```

### 2. Displaying User-Friendly Messages in React UI

```tsx
import React from "react";
import { decodeContractError } from "./errorDecoder";

interface ErrorBannerProps {
  error: unknown;
  onDismiss?: () => void;
}

export const ErrorBanner: React.FC<ErrorBannerProps> = ({ error, onDismiss }) => {
  if (!error) return null;

  const { name, userMessage, code } = decodeContractError(error);

  return (
    <div className="rounded-lg border border-red-500/30 bg-red-950/40 p-4 text-red-200">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="font-semibold text-red-100">{userMessage}</span>
          {code > 0 && (
            <span className="rounded bg-red-900/60 px-2 py-0.5 text-xs text-red-300">
              {name} (#{code})
            </span>
          )}
        </div>
        {onDismiss && (
          <button
            onClick={onDismiss}
            className="text-red-400 hover:text-red-100 focus:outline-none"
            aria-label="Dismiss error"
          >
            ✕
          </button>
        )}
      </div>
    </div>
  );
};
```
