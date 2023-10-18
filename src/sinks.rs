use crate::pb::erc20::types::v1::{ Erc20Token, BalanceChanges};
use crate::utils::helper::{append_0x};
use substreams::scalar::BigInt;
use substreams::store::StoreGetProto;
use substreams::store::StoreGet;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: BalanceChanges, token: StoreGetProto<Erc20Token>) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

   
    for storage_change in block.balance_changes {

        let token_lookup =
            token.get_last(&storage_change.contract);
        let token_found = token_lookup.is_some();

        if token_found {
            let token = token_lookup.unwrap();
            tables
            .create_row("Token", append_0x(&token.address))
            .set("name", token.name)
            .set("decimals", token.decimals)
            .set("symbol", token.symbol);
        }
       
        let id = format!("{}:{}", storage_change.contract, storage_change.owner);
        
        if storage_change.change_type == 0 {
            continue;
        }

        tables
            .create_row("Account", append_0x(&storage_change.owner.clone()));


            tables
            .create_row("Balance", id)
            // contract address
            .set("token", append_0x(&storage_change.contract))
            // storage change
            .set("owner", append_0x(&storage_change.owner)) 
            .set("new_balance", BigInt::try_from(storage_change.new_balance).unwrap_or_default())
            .set("old_balance", BigInt::try_from(storage_change.old_balance).unwrap_or_default())
            // trace information
            .set("transaction_hash_updated", append_0x(&storage_change.transaction))
            .set_bigint("block_num_updated", &block_num)
            .set_bigint("timestamp_updated", &timestamp);
           
       
       
    }

    Ok(tables.to_entity_changes())
}
