use common::types::Metadata;
use std::collections::{HashMap, HashSet};
use candid::{CandidType, Deserialize, Nat, Principal};
//use chrono::prelude::*;
use ic_storage::IcStorage;
use ic_cdk_macros::*;
use candid::candid_method;

mod tx_record;
pub use tx_record::*;

pub type Timestamp = u64;

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct StatsData {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Nat,
    pub owner: Principal,
    pub fee: Nat,
    pub fee_to: Principal,
    pub deploy_time: u64,
    pub min_cycles: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct TokenInfo {
    pub metadata: Metadata,
    pub feeTo: Principal,
    pub historySize: Nat,
    pub deployTime: Timestamp,
    pub holderNumber: usize,
    pub cycles: u64,
}

impl Default for StatsData {
    fn default() -> Self {
        StatsData {
            logo: "".to_string(),
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0u8,
            total_supply: Nat::from(0),
            owner: Principal::anonymous(),
            fee: Nat::from(0),
            fee_to: Principal::anonymous(),
            deploy_time: 0,
            min_cycles: 0,
        }
    }
}

pub type Allowances = HashMap<Principal, HashMap<Principal, Nat>>;

#[derive(CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    AmountTooSmall,
    FeeExceededLimit,
    NotificationFailed,
    AlreadyNotified,
    TransactionDoesNotExist,
}

pub type TxReceipt = Result<Nat, TxError>;
pub type PendingNotifications = HashSet<Nat>;

#[derive(CandidType, Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Succeeded,
    Failed,
}

#[derive(CandidType, Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum Operation {
    Approve,
    Mint,
    Transfer,
    TransferFrom,
    Burn,
    Auction,
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq)]
pub struct AuctionInfo {
    pub auction_id: usize,
    pub auction_time: Timestamp,
    pub tokens_distributed: Nat,
    pub cycles_collected: u64,
    pub fee_ratio: f64,
    pub first_transaction_id: Nat,
    pub last_transaction_id: Nat,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct Designation{
	pub owner: Principal,
	pub role: String,  //the building block for our role and address list
	pub assignment_time: Timestamp,
	pub tokens: Nat,
}
pub static mut DESIGNATION_LIST: Vec<Designation> = Vec::new();	
                                    //What if we make this :
                                    //pub static mut DESIGNATION_LIST: Vec<Designation> = Some(Vec::new());

                                    
#[update(name = "init_dl")]
#[candid_method(update)]
pub fn init_dl() {
    unsafe {
        DESIGNATION_LIST = Vec::new();
    }
}


#[query(name = "sizeDl")]
#[candid_method(query,rename = "sizeDl")]
#[ic_cdk_macros::query]
pub fn size_dl() -> usize{
    let mut size = 0;
    unsafe {
	size = DESIGNATION_LIST.len();
        ic_cdk::print(size.to_string());

    }
    size
}



#[ic_cdk_macros::query]
pub fn find_designation(wallet:Principal) -> Designation {
	let mut i: usize = 0;
	ic_cdk::print("designation list of size... \n");
	unsafe{
	ic_cdk::print(DESIGNATION_LIST.len().to_string());
	while i < DESIGNATION_LIST.len(){
		if DESIGNATION_LIST[i].owner == wallet{
            break;
        }
			
		else {
            i += 1;
        }   
			
	}
	if i >= DESIGNATION_LIST.len()
    {   
	ic_cdk::print("New wallet");
        return Designation{owner:wallet,
            role: "NA".to_string(),
            assignment_time: ic_kit::ic::time(),
            tokens: Nat::from(1 as u128),
        };
    }
	else{
        let des: Designation = DESIGNATION_LIST[i].clone();
        return des.clone();
    }
}		
}

pub fn remainder_limit(des: Designation) -> Nat {
	if des.role == "NA"{
	ic_cdk::print((des.assignment_time/1000000000).to_string());
	ic_cdk::print("\n");
        return Nat::from(0 as u32);
    }
		
	else if des.role == "founder" {
		let time_elapsed = (ic_kit::ic::time() - des.assignment_time)/1000000000;
		let days_elapsed = time_elapsed/(60*60*24);
		match days_elapsed {
			0 ..=90 => return des.tokens,
			90 ..=180 => return Nat::from((3 as u128)*des.tokens/(4 as u128) as u128),
			180 ..=270 => return Nat::from(des.tokens/(2 as u128) as u128),
			_ => return Nat::from(0 as u32),
		
		}
		
	}
	else if des.role == "advisor" {
                let time_elapsed = (ic_kit::ic::time() - des.assignment_time)/1000000000;
                let days_elapsed = time_elapsed/(60*60*24);
                match days_elapsed {
                        0 ..=90 => return des.tokens,
			90 ..=180 => return Nat::from((3 as u128)*des.tokens/(4 as u128) as u128),
                        180 ..=270 => return Nat::from(des.tokens/(2 as u128) as u128),

                        __ => return Nat::from(0 as u32),

                }

        }
	else if des.role == "investor" {
                let time_elapsed = (ic_kit::ic::time() - des.assignment_time)/1000000000;
                let days_elapsed = time_elapsed/(60*60*24);
                match days_elapsed {
                        0 ..=90 => return des.tokens,
                        90 ..=180 => return Nat::from((3 as u128)*des.tokens/(4 as u128) as u128),
                        180 ..=270 => return Nat::from(des.tokens/(2 as u128) as u128),

                        _ => return Nat::from(0 as u32),

                }

        }
	else if des.role == "private" {
                let time_elapsed = (ic_kit::ic::time() - des.assignment_time)/1000000000;
                let days_elapsed = time_elapsed/(60*60*24);
                match days_elapsed {
                        0 ..=90 => return des.tokens,
                        90 ..=180 => return Nat::from((3 as u128)*des.tokens/(4 as u128) as u128),
                        180 ..=270 => return Nat::from(des.tokens/(2 as u128) as u128),

                        _ => return Nat::from(0 as u32),

                }

        }
	else if des.role == "marketing" {
                let time_elapsed = (ic_kit::ic::time() - des.assignment_time)/1000000000;
                let days_elapsed = time_elapsed/(60*60*24);
                match days_elapsed {
                        0 ..=90 => return des.tokens,
                        90 ..=180 => return Nat::from((3 as u128)*des.tokens/(4 as u128) as u128),
                        180 ..=270 => return Nat::from(des.tokens/(2 as u128) as u128),

                        _ => return Nat::from(0 as u32),

                }

        }
	else {
		return Nat::from(0 as u32);
	}
}	

