

use crate::utils::helper::{append_0x, self};

use crate::abi::{self};
use crate::pb::erc20::types::v1::{
    BalanceOfStorageChange, Block as Erc20Block, Erc20Token
};
use abi::erc20::{
    functions::Transfer as TransferFun,
    functions::TransferFrom as TransferFromFun
};
use substreams::errors::Error;
use substreams::Hex;
use substreams::store::{StoreNew, StoreSetIfNotExistsProto, StoreSetIfNotExists};
use substreams_ethereum::pb::eth::v2::Block;



#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Erc20Block, Error> {
    let storage_changes = map_balance_of(block);

    Ok(Erc20Block {
        storage_changes
    })
}

#[substreams::handlers::store]
pub fn store_tokens(i0: Erc20Block, o: StoreSetIfNotExistsProto<Erc20Token>) {
    for storage_change in i0.storage_changes {
        o.set_if_not_exists(
            0,
            &storage_change.address,
            &helper::get_erc20_token(storage_change.address.clone()).unwrap(),
        );
    }
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
            // let address = storage_change.address.as_ref();
            // let decimals = Decimals::call(&self::Decimals {}, address).unwrap();
            // let name = NameFun::call(&self::NameFun {}, address).unwrap();
            storage_changes.push(BalanceOfStorageChange {
                // contract address
                address: append_0x(&Hex(&storage_change.address).to_string()),
                method: Hex::encode(&input[0..4]),
                // name: name,
                // decimals: decimals.to_string(),
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

