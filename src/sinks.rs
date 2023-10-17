use std::ops::Div;
use std::str::FromStr;

use crate::pb::erc20::types::v1::Block as Erc20Block;
use ethabi::ethereum_types::U256;
use substreams::log;
use substreams_sink_prometheus::{PrometheusOperations, Counter, Gauge};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;


#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Erc20Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();


    for storage_change in block.storage_changes {
        let id = format!("{}:{}", storage_change.address, storage_change.owner);

        let balance = U256::from_str(&storage_change.balance).unwrap();
        tables
            .create_row("Account", storage_change.owner.clone());
          
        tables
            .create_row("Balance", id)
            // contract address
            .set("address", storage_change.address)
            .set("method", storage_change.method)
            // storage change
            .set("owner", storage_change.owner)
            .set("balance", balance.to_string())
            // trace information
            .set("transaction", storage_change.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    Ok(tables.to_entity_changes())
}
