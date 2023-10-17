use crate::abi::{self};
use crate::pb::erc20::types::v1::{
    BalanceOfStorageChange, Block as Erc20Block
};
use abi::erc20::{
    functions::Transfer as TransferFun,
    functions::TransferFrom as TransferFromFun,
};
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

pub fn append_0x(i: &str) -> String {
    format!("0x{}", i)
}

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Erc20Block, Error> {
    let storage_changes = map_balance_of(block);

    Ok(Erc20Block {
        storage_changes
    })
}

pub fn map_balance_of(block: Block) -> Vec<BalanceOfStorageChange> {
    let mut storage_changes = vec![];

    // ETH calls
    for calls in block.calls() {
        // filter only successful calls
        if calls.call.state_reverted {
            continue;
        }

        // filter by calls containing 36 bytes of raw data
        let input = &calls.call.input;
        if input.len() < 36 {
            continue;
        } // skip if not 36 bytes

        // filter by method selector
        if !TransferFun::match_call(calls.call) && !TransferFromFun::match_call(calls.call) {
            continue;
        }

        // Storage changes
        for storage_change in &calls.call.storage_changes {
            storage_changes.push(BalanceOfStorageChange {
                // contract address
                address: append_0x(&Hex::encode(&storage_change.address)),
                method: Hex::encode(&input[0..4]),

                // storage changes
                owner: append_0x(&Hex::encode(&calls.call.caller)),
                balance: Hex::encode(&storage_change.new_value),
                

                // trace information
                transaction: Hex::encode(&calls.transaction.hash),
            })
        }
    }

    storage_changes
}
