mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const ACOFEE_TRACKED_CONTRACT: [u8; 20] = hex!("68a99c3e90cf4ccbc933daae4f303cfc05291871");
const PINFT_TRACKED_CONTRACT: [u8; 20] = hex!("b92000f91a80144d03cbd8931095683b7f76405e");
const PIMETH_TRACKED_CONTRACT: [u8; 20] = hex!("e8b5338739ea8358d6cc0f322ec390cd57259bbd");
const PIMARK_TRACKED_CONTRACT: [u8; 20] = hex!("1a28451ff140cf1224706839af44b8de4eb3f019");
const LENDBORROW_TRACKED_CONTRACT: [u8; 20] = hex!("5bb81157eb2b27844d6d6ff69abbb9eda4ef2cdd");
const VALIDATEDNFT_TRACKED_CONTRACT: [u8; 20] = hex!("a9eb3c4160c8963a02c29ca7cdee4359c9c659c9");

fn map_acofee_events(blk: &eth::Block, events: &mut contract::Events) {
    events.acofee_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACOFEE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::acofee_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::AcofeeOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.acofee_set_aconomy_nft_lend_borrow_fees.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACOFEE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::acofee_contract::events::SetAconomyNftLendBorrowFee::match_and_decode(log) {
                        return Some(contract::AcofeeSetAconomyNftLendBorrowFee {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_fee: event.new_fee.to_u64(),
                            old_fee: event.old_fee.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.acofee_set_aconomy_pi_market_fees.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACOFEE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::acofee_contract::events::SetAconomyPiMarketFee::match_and_decode(log) {
                        return Some(contract::AcofeeSetAconomyPiMarketFee {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_fee: event.new_fee.to_u64(),
                            old_fee: event.old_fee.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.acofee_set_aconomy_pool_fees.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ACOFEE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::acofee_contract::events::SetAconomyPoolFee::match_and_decode(log) {
                        return Some(contract::AcofeeSetAconomyPoolFee {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_fee: event.new_fee.to_u64(),
                            old_fee: event.old_fee.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_acofee_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.acofee_call_renounce_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ACOFEE_TRACKED_CONTRACT && abi::acofee_contract::functions::RenounceOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::acofee_contract::functions::RenounceOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::AcofeeRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.acofee_call_set_aconomy_nft_lend_borrow_fees.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ACOFEE_TRACKED_CONTRACT && abi::acofee_contract::functions::SetAconomyNftLendBorrowFee::match_call(call))
                .filter_map(|call| {
                    match abi::acofee_contract::functions::SetAconomyNftLendBorrowFee::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::AcofeeSetAconomyNftLendBorrowFeeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_fee: decoded_call.new_fee.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.acofee_call_set_aconomy_pi_market_fees.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ACOFEE_TRACKED_CONTRACT && abi::acofee_contract::functions::SetAconomyPiMarketFee::match_call(call))
                .filter_map(|call| {
                    match abi::acofee_contract::functions::SetAconomyPiMarketFee::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::AcofeeSetAconomyPiMarketFeeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_fee: decoded_call.new_fee.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.acofee_call_set_aconomy_pool_fees.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ACOFEE_TRACKED_CONTRACT && abi::acofee_contract::functions::SetAconomyPoolFee::match_call(call))
                .filter_map(|call| {
                    match abi::acofee_contract::functions::SetAconomyPoolFee::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::AcofeeSetAconomyPoolFeeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_fee: decoded_call.new_fee.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.acofee_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ACOFEE_TRACKED_CONTRACT && abi::acofee_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::acofee_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::AcofeeTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

fn map_pinft_events(blk: &eth::Block, events: &mut contract::Events) {
    events.pinft_admin_changed_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::AdminChanged1::match_and_decode(log) {
                        return Some(contract::PinftAdminChanged1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_admin_changed_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::AdminChanged2::match_and_decode(log) {
                        return Some(contract::PinftAdminChanged2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::PinftApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            owner: event.owner,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_approval_for_alls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::ApprovalForAll::match_and_decode(log) {
                        return Some(contract::PinftApprovalForAll {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            operator: event.operator,
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_batch_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::BatchMetadataUpdate::match_and_decode(log) {
                        return Some(contract::PinftBatchMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_from_token_id: event.u_from_token_id.to_string(),
                            u_to_token_id: event.u_to_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_beacon_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::BeaconUpgraded1::match_and_decode(log) {
                        return Some(contract::PinftBeaconUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_beacon_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::BeaconUpgraded2::match_and_decode(log) {
                        return Some(contract::PinftBeaconUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::PinftInitialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::MetadataUpdate::match_and_decode(log) {
                        return Some(contract::PinftMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_token_id: event.u_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::PinftOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_pauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::Paused::match_and_decode(log) {
                        return Some(contract::PinftPaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_royalties_set_for_token_ids.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::RoyaltiesSetForTokenId::match_and_decode(log) {
                        return Some(contract::PinftRoyaltiesSetForTokenId {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_royalties_set_for_validators.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::RoyaltiesSetForValidator::match_and_decode(log) {
                        return Some(contract::PinftRoyaltiesSetForValidator {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_token_minteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::TokenMinted::match_and_decode(log) {
                        return Some(contract::PinftTokenMinted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                            uri: event.uri,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::PinftTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_unpauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::Unpaused::match_and_decode(log) {
                        return Some(contract::PinftUnpaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::Upgraded1::match_and_decode(log) {
                        return Some(contract::PinftUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pinft_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PINFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pinft_contract::events::Upgraded2::match_and_decode(log) {
                        return Some(contract::PinftUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_pinft_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.pinft_call_add_trusted_forwarders.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::AddTrustedForwarder::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::AddTrustedForwarder::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftAddTrustedForwarderCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_tf: decoded_call.u_tf,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::Approve::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_delete_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::DeleteNft::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::DeleteNft::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftDeleteNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_delete_validator_royalties.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::DeleteValidatorRoyalties::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::DeleteValidatorRoyalties::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftDeleteValidatorRoyaltiesCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::Initialize::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                tf_gelato: decoded_call.tf_gelato,
                                u_name: decoded_call.u_name,
                                u_pi_nft_methods_address: decoded_call.u_pi_nft_methods_address,
                                u_symbol: decoded_call.u_symbol,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_lazy_mint_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::LazyMintNft::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::LazyMintNft::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::pinft_contract::functions::LazyMintNft::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PinftLazyMintNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0.to_string(),
                                u_to: decoded_call.u_to,
                                u_uri: decoded_call.u_uri,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_mint_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::MintNft::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::MintNft::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::pinft_contract::functions::MintNft::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PinftMintNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0.to_string(),
                                u_to: decoded_call.u_to,
                                u_uri: decoded_call.u_uri,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_pauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::Pause::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::Pause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftPauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_remove_trusted_forwarders.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::RemoveTrustedForwarder::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::RemoveTrustedForwarder::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftRemoveTrustedForwarderCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_tf: decoded_call.u_tf,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_renounce_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::RenounceOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::RenounceOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_safe_transfer_from_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::SafeTransferFrom1::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::SafeTransferFrom1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftSafeTransferFrom1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_safe_transfer_from_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::SafeTransferFrom2::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::SafeTransferFrom2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftSafeTransferFrom2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                from: decoded_call.from,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_set_approval_for_alls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::SetApprovalForAll::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::SetApprovalForAll::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftSetApprovalForAllCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                approved: decoded_call.approved,
                                operator: decoded_call.operator,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_set_royalties_for_validators.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::SetRoyaltiesForValidator::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::SetRoyaltiesForValidator::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftSetRoyaltiesForValidatorCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_commission: decoded_call.u_commission.to_string(),
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::TransferFrom::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_unpauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::Unpause::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::Unpause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftUnpauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_upgrade_tos.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::UpgradeTo::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::UpgradeTo::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftUpgradeToCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pinft_call_upgrade_to_and_calls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PINFT_TRACKED_CONTRACT && abi::pinft_contract::functions::UpgradeToAndCall::match_call(call))
                .filter_map(|call| {
                    match abi::pinft_contract::functions::UpgradeToAndCall::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PinftUpgradeToAndCallCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

fn map_pimeth_events(blk: &eth::Block, events: &mut contract::Events) {
    events.pimeth_admin_changed_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::AdminChanged1::match_and_decode(log) {
                        return Some(contract::PimethAdminChanged1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_admin_changed_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::AdminChanged2::match_and_decode(log) {
                        return Some(contract::PimethAdminChanged2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_beacon_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::BeaconUpgraded1::match_and_decode(log) {
                        return Some(contract::PimethBeaconUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_beacon_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::BeaconUpgraded2::match_and_decode(log) {
                        return Some(contract::PimethBeaconUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_erc20_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::Erc20Added::match_and_decode(log) {
                        return Some(contract::PimethErc20Added {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_address: event.collection_address,
                            erc20_contract: event.erc20_contract,
                            from: event.from,
                            token_id: event.token_id.to_string(),
                            total_balance: event.total_balance.to_string(),
                            uri: event.uri,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_erc20_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::Erc20Transferred::match_and_decode(log) {
                        return Some(contract::PimethErc20Transferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_address: event.collection_address,
                            erc20_contract: event.erc20_contract,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::PimethInitialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::PimethOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_pauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::Paused::match_and_decode(log) {
                        return Some(contract::PimethPaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_pi_nft_burnts.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::PiNftBurnt::match_and_decode(log) {
                        return Some(contract::PimethPiNftBurnt {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_address: event.collection_address,
                            erc20_contract: event.erc20_contract,
                            erc20_receiver: event.erc20_receiver,
                            nft_reciever: event.nft_reciever,
                            token_id: event.token_id.to_string(),
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_pi_nft_redeemeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::PiNftRedeemed::match_and_decode(log) {
                        return Some(contract::PimethPiNftRedeemed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_address: event.collection_address,
                            erc20_contract: event.erc20_contract,
                            nft_reciever: event.nft_reciever,
                            token_id: event.token_id.to_string(),
                            validator_address: event.validator_address,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_unpauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::Unpaused::match_and_decode(log) {
                        return Some(contract::PimethUnpaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::Upgraded1::match_and_decode(log) {
                        return Some(contract::PimethUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::Upgraded2::match_and_decode(log) {
                        return Some(contract::PimethUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_validator_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::ValidatorAdded::match_and_decode(log) {
                        return Some(contract::PimethValidatorAdded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_address: event.collection_address,
                            token_id: event.token_id.to_string(),
                            validator: event.validator,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_validator_funds_repayeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::ValidatorFundsRepayed::match_and_decode(log) {
                        return Some(contract::PimethValidatorFundsRepayed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            collection_address: event.collection_address,
                            erc20_contract: event.erc20_contract,
                            repayer: event.repayer,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimeth_validator_funds_withdrawns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimeth_contract::events::ValidatorFundsWithdrawn::match_and_decode(log) {
                        return Some(contract::PimethValidatorFundsWithdrawn {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            collection_address: event.collection_address,
                            erc20_contract: event.erc20_contract,
                            token_id: event.token_id.to_string(),
                            withdrawer: event.withdrawer,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_pimeth_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.pimeth_call_repays.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::Repay::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::Repay::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethRepayCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_collection_address: decoded_call.u_collection_address,
                                u_erc20_contract: decoded_call.u_erc20_contract,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_add_erc_20s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::AddErc20::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::AddErc20::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethAddErc20call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_collection_address: decoded_call.u_collection_address,
                                u_commission: decoded_call.u_commission.to_string(),
                                u_erc20_contract: decoded_call.u_erc20_contract,
                                u_token_id: decoded_call.u_token_id.to_string(),
                                u_uri: decoded_call.u_uri,
                                u_value: decoded_call.u_value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_add_trusted_forwarders.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::AddTrustedForwarder::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::AddTrustedForwarder::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethAddTrustedForwarderCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_tf: decoded_call.u_tf,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_add_validators.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::AddValidator::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::AddValidator::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethAddValidatorCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_collection_address: decoded_call.u_collection_address,
                                u_token_id: decoded_call.u_token_id.to_string(),
                                u_validator: decoded_call.u_validator,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::Initialize::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                trusted_forwarder: decoded_call.trusted_forwarder,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_lazy_add_validators.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::LazyAddValidator::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::LazyAddValidator::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethLazyAddValidatorCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_collection_address: decoded_call.u_collection_address,
                                u_token_id: decoded_call.u_token_id.to_string(),
                                u_validator: decoded_call.u_validator,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_on_erc721_receiveds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::OnErc721Received::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::OnErc721Received::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::pimeth_contract::functions::OnErc721Received::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PimethOnErc721ReceivedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: Vec::from(output_param0),
                                param0: decoded_call.param0,
                                param1: decoded_call.param1,
                                param2: decoded_call.param2.to_string(),
                                param3: decoded_call.param3,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_paid_commissions.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::PaidCommission::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::PaidCommission::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethPaidCommissionCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_collection: decoded_call.u_collection,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_pauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::Pause::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::Pause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethPauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_redeem_or_burn_pi_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::RedeemOrBurnPiNft::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::RedeemOrBurnPiNft::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethRedeemOrBurnPiNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                burn_nft: decoded_call.burn_nft,
                                u_collection_address: decoded_call.u_collection_address,
                                u_erc20_contract: decoded_call.u_erc20_contract,
                                u_erc20_receiver: decoded_call.u_erc20_receiver,
                                u_nft_receiver: decoded_call.u_nft_receiver,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_remove_trusted_forwarders.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::RemoveTrustedForwarder::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::RemoveTrustedForwarder::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethRemoveTrustedForwarderCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_tf: decoded_call.u_tf,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_renounce_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::RenounceOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::RenounceOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_set_pi_markets.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::SetPiMarket::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::SetPiMarket::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethSetPiMarketCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_pi_market: decoded_call.u_pi_market,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_unpauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::Unpause::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::Unpause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethUnpauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_upgrade_tos.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::UpgradeTo::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::UpgradeTo::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethUpgradeToCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_upgrade_to_and_calls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::UpgradeToAndCall::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::UpgradeToAndCall::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethUpgradeToAndCallCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimeth_call_withdraws.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMETH_TRACKED_CONTRACT && abi::pimeth_contract::functions::Withdraw::match_call(call))
                .filter_map(|call| {
                    match abi::pimeth_contract::functions::Withdraw::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimethWithdrawCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_collection_address: decoded_call.u_collection_address,
                                u_erc20_contract: decoded_call.u_erc20_contract,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

fn map_pimark_events(blk: &eth::Block, events: &mut contract::Events) {
    events.pimark_admin_changed_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::AdminChanged1::match_and_decode(log) {
                        return Some(contract::PimarkAdminChanged1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_admin_changed_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::AdminChanged2::match_and_decode(log) {
                        return Some(contract::PimarkAdminChanged2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_beacon_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::BeaconUpgraded1::match_and_decode(log) {
                        return Some(contract::PimarkBeaconUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_beacon_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::BeaconUpgraded2::match_and_decode(log) {
                        return Some(contract::PimarkBeaconUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_bid_events.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::BidEvent::match_and_decode(log) {
                        return Some(contract::PimarkBidEvent {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            bid_created: event.bid_created,
                            bid_id: event.bid_id.to_string(),
                            collection_address: event.collection_address,
                            sale_id: event.sale_id.to_string(),
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_bid_withdrawns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::BidWithdrawn::match_and_decode(log) {
                        return Some(contract::PimarkBidWithdrawn {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            bid_id: event.bid_id.to_string(),
                            sale_id: event.sale_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::PimarkInitialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_nft_boughts.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::NftBought::match_and_decode(log) {
                        return Some(contract::PimarkNftBought {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_address: event.collection_address,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::PimarkOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_pauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::Paused::match_and_decode(log) {
                        return Some(contract::PimarkPaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_sale_cancelleds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::SaleCancelled::match_and_decode(log) {
                        return Some(contract::PimarkSaleCancelled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sale_id: event.sale_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_sale_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::SaleCreated::match_and_decode(log) {
                        return Some(contract::PimarkSaleCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            bid_time_duration: event.bid_time_duration.to_string(),
                            price: event.price.to_string(),
                            sale_id: event.sale_id.to_string(),
                            token_contract: event.token_contract,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_swap_accepteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::SwapAccepted::match_and_decode(log) {
                        return Some(contract::PimarkSwapAccepted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            swap_id: event.swap_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_swap_cancelleds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::SwapCancelled::match_and_decode(log) {
                        return Some(contract::PimarkSwapCancelled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            swap_id: event.swap_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_swap_proposeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::SwapProposed::match_and_decode(log) {
                        return Some(contract::PimarkSwapProposed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            in_token_id: event.in_token_id.to_string(),
                            in_token_id_address: event.in_token_id_address,
                            out_token_id: event.out_token_id.to_string(),
                            out_token_id_address: event.out_token_id_address,
                            swap_id: event.swap_id.to_string(),
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_unpauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::Unpaused::match_and_decode(log) {
                        return Some(contract::PimarkUnpaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::Upgraded1::match_and_decode(log) {
                        return Some(contract::PimarkUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::Upgraded2::match_and_decode(log) {
                        return Some(contract::PimarkUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pimark_updated_sale_prices.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PIMARK_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pimark_contract::events::UpdatedSalePrice::match_and_decode(log) {
                        return Some(contract::PimarkUpdatedSalePrice {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            duration: event.duration.to_string(),
                            price: event.price.to_string(),
                            sale_id: event.sale_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_pimark_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.pimark_call_bids.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::Bid::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::Bid::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkBidCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_bid_price: decoded_call.u_bid_price.to_string(),
                                u_sale_id: decoded_call.u_sale_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_buy_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::BuyNft::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::BuyNft::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkBuyNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_sale_id: decoded_call.u_sale_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_sell_nft_by_bids.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::SellNftByBid::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::SellNftByBid::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkSellNftByBidCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_bid_time: decoded_call.u_bid_time.to_string(),
                                u_contract_address: decoded_call.u_contract_address,
                                u_currency: decoded_call.u_currency,
                                u_price: decoded_call.u_price.to_string(),
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_accept_swap_requests.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::AcceptSwapRequest::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::AcceptSwapRequest::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkAcceptSwapRequestCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                swap_id: decoded_call.swap_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_cancel_sales.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::CancelSale::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::CancelSale::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkCancelSaleCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_sale_id: decoded_call.u_sale_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_cancel_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::CancelSwap::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::CancelSwap::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkCancelSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_swap_id: decoded_call.u_swap_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_edit_sale_prices.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::EditSalePrice::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::EditSalePrice::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkEditSalePriceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_duration: decoded_call.u_duration.to_string(),
                                u_price: decoded_call.u_price.to_string(),
                                u_sale_id: decoded_call.u_sale_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_execute_bid_orders.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::ExecuteBidOrder::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::ExecuteBidOrder::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkExecuteBidOrderCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_bid_order_id: decoded_call.u_bid_order_id.to_string(),
                                u_sale_id: decoded_call.u_sale_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::Initialize::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_collection_factory_address: decoded_call.u_collection_factory_address,
                                u_fee_address: decoded_call.u_fee_address,
                                u_pi_nf_taddress: decoded_call.u_pi_nf_taddress,
                                u_pi_nft_methods_address: decoded_call.u_pi_nft_methods_address,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_make_swap_requests.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::MakeSwapRequest::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::MakeSwapRequest::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::pimark_contract::functions::MakeSwapRequest::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PimarkMakeSwapRequestCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                contract_address1: decoded_call.contract_address1,
                                contract_address2: decoded_call.contract_address2,
                                output_param0: output_param0.to_string(),
                                token1: decoded_call.token1.to_string(),
                                token2: decoded_call.token2.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_on_erc721_receiveds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::OnErc721Received::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::OnErc721Received::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::pimark_contract::functions::OnErc721Received::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PimarkOnErc721ReceivedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: Vec::from(output_param0),
                                param0: decoded_call.param0,
                                param1: decoded_call.param1,
                                param2: decoded_call.param2.to_string(),
                                param3: decoded_call.param3,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_pauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::Pause::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::Pause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkPauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_renounce_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::RenounceOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::RenounceOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_sell_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::SellNft::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::SellNft::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkSellNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_contract_address: decoded_call.u_contract_address,
                                u_currency: decoded_call.u_currency,
                                u_price: decoded_call.u_price.to_string(),
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_unpauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::Unpause::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::Unpause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkUnpauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_upgrade_tos.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::UpgradeTo::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::UpgradeTo::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkUpgradeToCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_upgrade_to_and_calls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::UpgradeToAndCall::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::UpgradeToAndCall::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkUpgradeToAndCallCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pimark_call_withdraw_bid_monies.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PIMARK_TRACKED_CONTRACT && abi::pimark_contract::functions::WithdrawBidMoney::match_call(call))
                .filter_map(|call| {
                    match abi::pimark_contract::functions::WithdrawBidMoney::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::PimarkWithdrawBidMoneyCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_bid_id: decoded_call.u_bid_id.to_string(),
                                u_sale_id: decoded_call.u_sale_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

fn map_lendborrow_events(blk: &eth::Block, events: &mut contract::Events) {
    events.lendborrow_admin_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == LENDBORROW_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::lendborrow_contract::events::AdminChanged::match_and_decode(log) {
                        return Some(contract::LendborrowAdminChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendborrow_beacon_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == LENDBORROW_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::lendborrow_contract::events::BeaconUpgraded::match_and_decode(log) {
                        return Some(contract::LendborrowBeaconUpgraded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendborrow_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == LENDBORROW_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::lendborrow_contract::events::Upgraded::match_and_decode(log) {
                        return Some(contract::LendborrowUpgraded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_lendborrow_calls(blk: &eth::Block, calls: &mut contract::Calls) {
}

fn map_validatednft_events(blk: &eth::Block, events: &mut contract::Events) {
    events.validatednft_admin_changed_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::AdminChanged1::match_and_decode(log) {
                        return Some(contract::ValidatednftAdminChanged1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_admin_changed_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::AdminChanged2::match_and_decode(log) {
                        return Some(contract::ValidatednftAdminChanged2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::ValidatednftApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            owner: event.owner,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_approval_for_alls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::ApprovalForAll::match_and_decode(log) {
                        return Some(contract::ValidatednftApprovalForAll {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            operator: event.operator,
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_batch_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::BatchMetadataUpdate::match_and_decode(log) {
                        return Some(contract::ValidatednftBatchMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_from_token_id: event.u_from_token_id.to_string(),
                            u_to_token_id: event.u_to_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_beacon_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::BeaconUpgraded1::match_and_decode(log) {
                        return Some(contract::ValidatednftBeaconUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_beacon_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::BeaconUpgraded2::match_and_decode(log) {
                        return Some(contract::ValidatednftBeaconUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::ValidatednftInitialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::MetadataUpdate::match_and_decode(log) {
                        return Some(contract::ValidatednftMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_token_id: event.u_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::ValidatednftOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_pauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::Paused::match_and_decode(log) {
                        return Some(contract::ValidatednftPaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_royalties_set_for_validators.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::RoyaltiesSetForValidator::match_and_decode(log) {
                        return Some(contract::ValidatednftRoyaltiesSetForValidator {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_token_minteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::TokenMinted::match_and_decode(log) {
                        return Some(contract::ValidatednftTokenMinted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                            uri: event.uri,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::ValidatednftTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_unpauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::Unpaused::match_and_decode(log) {
                        return Some(contract::ValidatednftUnpaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::Upgraded1::match_and_decode(log) {
                        return Some(contract::ValidatednftUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.validatednft_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == VALIDATEDNFT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::validatednft_contract::events::Upgraded2::match_and_decode(log) {
                        return Some(contract::ValidatednftUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_validatednft_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.validatednft_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::Approve::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_delete_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::DeleteNft::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::DeleteNft::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftDeleteNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_delete_validator_royalties.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::DeleteValidatorRoyalties::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::DeleteValidatorRoyalties::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftDeleteValidatorRoyaltiesCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::Initialize::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_name: decoded_call.u_name,
                                u_pi_nf_tmethod_address: decoded_call.u_pi_nf_tmethod_address,
                                u_symbol: decoded_call.u_symbol,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_mint_validated_nfts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::MintValidatedNft::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::MintValidatedNft::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::validatednft_contract::functions::MintValidatedNft::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::ValidatednftMintValidatedNftCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0.to_string(),
                                u_to: decoded_call.u_to,
                                u_uri: decoded_call.u_uri,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_on_erc721_receiveds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::OnErc721Received::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::OnErc721Received::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::validatednft_contract::functions::OnErc721Received::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::ValidatednftOnErc721ReceivedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: Vec::from(output_param0),
                                param0: decoded_call.param0,
                                param1: decoded_call.param1,
                                param2: decoded_call.param2.to_string(),
                                param3: decoded_call.param3,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_pauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::Pause::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::Pause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftPauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_renounce_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::RenounceOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::RenounceOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_safe_transfer_from_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::SafeTransferFrom1::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::SafeTransferFrom1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftSafeTransferFrom1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_safe_transfer_from_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::SafeTransferFrom2::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::SafeTransferFrom2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftSafeTransferFrom2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                from: decoded_call.from,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_set_approval_for_alls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::SetApprovalForAll::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::SetApprovalForAll::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftSetApprovalForAllCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                approved: decoded_call.approved,
                                operator: decoded_call.operator,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_set_royalties_for_validators.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::SetRoyaltiesForValidator::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::SetRoyaltiesForValidator::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftSetRoyaltiesForValidatorCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_commission: decoded_call.u_commission.to_string(),
                                u_token_id: decoded_call.u_token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::TransferFrom::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                to: decoded_call.to,
                                token_id: decoded_call.token_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_unpauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::Unpause::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::Unpause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftUnpauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_upgrade_tos.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::UpgradeTo::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::UpgradeTo::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftUpgradeToCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.validatednft_call_upgrade_to_and_calls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == VALIDATEDNFT_TRACKED_CONTRACT && abi::validatednft_contract::functions::UpgradeToAndCall::match_call(call))
                .filter_map(|call| {
                    match abi::validatednft_contract::functions::UpgradeToAndCall::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::ValidatednftUpgradeToAndCallCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_acofee_events(&blk, &mut events);
    map_pinft_events(&blk, &mut events);
    map_pimeth_events(&blk, &mut events);
    map_pimark_events(&blk, &mut events);
    map_lendborrow_events(&blk, &mut events);
    map_validatednft_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
let mut calls = contract::Calls::default();
    map_acofee_calls(&blk, &mut calls);
    map_pinft_calls(&blk, &mut calls);
    map_pimeth_calls(&blk, &mut calls);
    map_pimark_calls(&blk, &mut calls);
    map_lendborrow_calls(&blk, &mut calls);
    map_validatednft_calls(&blk, &mut calls);
    Ok(calls)
}

