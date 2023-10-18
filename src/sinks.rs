use std::str::FromStr;

use crate::pb::erc20::types::v1::{Block as Erc20Block, Erc20Token};
use crate::utils::helper::{append_0x};
use ethabi::ethereum_types::U256;
use substreams::Hex;
use substreams::store::StoreGetProto;
use substreams::store::StoreGet;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Erc20Block, token: StoreGetProto<Erc20Token>) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

   
    for storage_change in block.storage_changes {

        let token_lookup =
            token.get_last(&storage_change.address);
        let token_found = token_lookup.is_some();

        if token_found {
            let token = token_lookup.unwrap();
            tables
            .create_row("Token", token.address)
            .set("name", token.name)
            .set("decimals", token.decimals)
            .set("symbol", token.symbol);
        }
       
        let id = format!("{}:{}", storage_change.address, storage_change.owner);

        let balance = U256::from_str(&storage_change.balance).unwrap();
        let zero = U256::from_str("0").unwrap();
        tables
            .create_row("Account", storage_change.owner.clone());

        if balance == zero {
            tables
            .delete_row("Balance", id);
        } else {
           

            tables
            .create_row("Balance", id)
            // contract address
            .set("token", storage_change.address)
            // storage change
            .set("owner", storage_change.owner)
            .set("balance", balance.to_string())
            // trace information
            .set("transaction_hash_updated", storage_change.transaction)
            .set_bigint("block_num_updated", &block_num)
            .set_bigint("timestamp_updated", &timestamp);
        }
       
    }

    Ok(tables.to_entity_changes())
}
