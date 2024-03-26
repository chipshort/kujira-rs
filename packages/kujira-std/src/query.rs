use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, CustomQuery, Decimal};

use crate::denom::Denom;

/// KujiraQuery is an override of QueryRequest::Custom to access Terra-specific modules
#[cw_serde]
pub enum KujiraQuery {
    Bank(BankQuery),
    Denom(DenomQuery),
    Oracle(OracleQuery),
    Ica(IcaQuery),
}

impl CustomQuery for KujiraQuery {}

#[cw_serde]
pub enum BankQuery {
    Supply { denom: Denom },
}

/// This contains all queries that can be made to the denom module
#[cw_serde]
pub enum DenomQuery {
    /// Given a subdenom minted by a contract via `DenomMsg::MintTokens`,
    /// returns the full denom as used by `BankMsg::Send`.
    FullDenom {
        creator_addr: Addr,
        subdenom: String,
    },
    /// Returns the admin of a denom, if the denom is a Token Factory denom.
    DenomAdmin { subdenom: String },
}

/// This contains all queries that can be made to the oracle module
#[cw_serde]
pub enum OracleQuery {
    // ExchangeRate will return the rate of this denom.
    ExchangeRate { denom: String },
    // ExchangeRates will return the exchange rate between offer denom and all supported asks
    // ExchangeRates { offer: String },
}

/// This contains all queries that can be made to the cw-ica module
#[cw_serde]
pub enum IcaQuery {
    // AccountAddress will return the address of the interchain account.
    AccountAddress {
        owner: Addr,
        connection_id: String,
        account_id: String,
    },
}

/// ExchangeRateResponse is data format returned from OracleRequest::ExchangeRate query
#[cw_serde]
pub struct ExchangeRateResponse {
    pub rate: Decimal,
}

#[cw_serde]
pub struct SupplyResponse {
    pub amount: Coin,
}

#[cw_serde]
pub struct FullDenomResponse {
    pub denom: Denom,
}

#[cw_serde]
pub struct DenomAdminResponse {
    pub admin: Addr,
}

/// AccountAddressResponse is data format returned from CwIcaRequest::AccountAddress query
#[cw_serde]
pub struct AccountAddressResponse {
    pub address: String,
}
