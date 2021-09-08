use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw721::ContractInfoResponse;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoyaltiesInfo {
    pub royalty_payments: bool,
    /// This is how much the minter takes as a cut when sold
    pub royalty_percentage: Option<u32>,
    /// The payment address, may be different to or the same
    /// as the minter addr
    pub royalty_payment_address: Option<Addr>,
}

pub const ROYALTIES_INFO: Item<RoyaltiesInfo> = Item::new("royalties_info");
pub const CONTRACT_INFO: Item<ContractInfoResponse> = Item::new("nft_info");
