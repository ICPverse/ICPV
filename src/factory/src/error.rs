use candid::CandidType;
use ic_helpers::factory::error::FactoryError;
use thiserror::Error;

#[derive(Debug, Error, CandidType)]
pub enum TokenFactoryError {
    #[error("the property {0} has invalid value: {0}")]
    InvalidConfiguration(&'static str, &'static str),

    #[error("a token with the same name is already registered")]
    AlreadyExists,

    #[error("failed to create token canister: {0}")]
    CanisterCreateFailed(String),

    #[error(transparent)]
    FactoryError(#[from] FactoryError),
}
