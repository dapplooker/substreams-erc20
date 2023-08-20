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

    for event in block.transfers {
        let id = format!("{}-{}", event.block_index, event.transaction);

        tables
            .create_row("Transfer", id)
            // contract address
            .set("address", event.address)
            // event payload
            .set("from", event.from)
            .set("to", event.to)
            .set("value", event.value)
            // trace information
            .set("transaction", event.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    for event in block.approvals {
        let id = format!("{}-{}", event.block_index, event.transaction);

        tables
            .create_row("Approval", id)
            // contract address
            .set("address", event.address)
            // event payload
            .set("owner", event.owner)
            .set("spender", event.spender)
            .set("value", event.value)
            // trace information
            .set("transaction", event.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    for storage_change in block.storage_changes {
        let id = format!("{}:{}", storage_change.address, storage_change.owner);

        tables
            .create_row("BalanceOf", id)
            // contract address
            .set("address", storage_change.address)
            .set("method", storage_change.method)
            // storage change
            .set("owner", storage_change.owner)
            .set("balance", storage_change.balance)
            // trace information
            .set("transaction", storage_change.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    Ok(tables.to_entity_changes())
}

// #[substreams::handlers::map]
// pub fn prom_out(block: Erc20Block) -> Result<PrometheusOperations, Error> {
//     let mut prom_out = PrometheusOperations::default();

//     // for storage_change in block.storage_changes {
//     //     let id = format!("{}:{}", storage_change.address, storage_change.owner);

//     //     // U256::from
//     //     let balance = U256::from_str(&storage_change.balance).unwrap().low_u128() as f64;

//     //     log::info!("{}:{}:{}", storage_change.address, storage_change.transaction, balance);
//     //     prom_out.push(Gauge::from(&id).set(balance));
//     // }

//     for storage_change in block.storage_changes {
//         let id = format!("{}:{}", storage_change.address, storage_change.owner);

//         // U256::from
//         let balance = U256::from_str(&storage_change.balance).unwrap().div(10_i32.pow(18));

//         log::info!("{} {} {}", id, storage_change.transaction, balance);
//         prom_out.push(Gauge::from(&id).set(1.0));
//     }

//     Ok(prom_out)
// }


// #[substreams::handlers::map]
// pub fn kv_out(block: Erc20Block) -> Result<PrometheusOperations, Error> {
//     let mut prom_out = PrometheusOperations::default();

//     // // for storage_change in block.storage_changes {
//     // //     let id = format!("{}:{}", storage_change.address, storage_change.owner);

//     // //     // U256::from
//     // //     let balance = U256::from_str(&storage_change.balance).unwrap().low_u128() as f64;

//     // //     log::info!("{}:{}:{}", storage_change.address, storage_change.transaction, balance);
//     // //     prom_out.push(Gauge::from(&id).set(balance));
//     // // }

//     // for storage_change in block.storage_changes {
//     //     let id = format!("{}:{}", storage_change.address, storage_change.owner);

//     //     // U256::from
//     //     let balance = U256::from_str(&storage_change.balance).unwrap().div(10_i32.pow(18));

//     //     log::info!("{} {} {}", id, storage_change.transaction, balance);
//     //     prom_out.push(Gauge::from(&id).set(1.0));
//     // }

//     Ok(prom_out)
// }

#[substreams::handlers::map]
pub fn kv_out(block: Erc20Block) -> Result<KvOperations, Error> {
    let mut kv_out = KvOperations::default();

    for storage_change in block.storage_changes {
        let id = format!("{}:{}", storage_change.address, storage_change.owner);

        // U256::from
        let balance = U256::from_str(&storage_change.balance).unwrap();

        log::info!("{} {} {} {}", id, storage_change.transaction, balance, storage_change.balance);
        kv_out.push_new(id, storage_change.balance, 1);
    }

    Ok(kv_out)
}