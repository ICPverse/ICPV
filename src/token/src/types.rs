use common::types::Metadata;
use std::collections::{HashMap, HashSet};
use candid::{CandidType, Deserialize, Nat, Principal};
use chrono::prelude::*;


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
pub struct Designation{
	pub owner: Principal,
	pub role: String,  //the building block for our role and address list
	pub assignment_time: i64,
	pub tokens: Nat,
}
pub static mut DesignationList: Option<Vec<Designation>> = None;


pub fn init_dl() {
    unsafe {
        DesignationList = Some(Vec::new());
    }
}



pub fn find_designation(wallet: Principal, dlist: Vec<Designation>) -> Designation {
	let mut i: usize = 0;
	while i < dlist.len(){
		if dlist[i].owner == wallet{
            break;
        }
			
		else {
            i += 1;
        }   
			
	}
	if i == dlist.len()
    {
        Ok(Designation{owner: Principal, role: "NA".to_string(), assignment_time: Utc::now().timestamp(),tokens: (1 as candid::Nat)});
    }
	else{
        Ok(dlist[i]);
    }
		
}

pub fn remainder_limit(des: Designation, dlist: Vec<Designation>) -> Nat {
	if des.role == "NA"{
        return 0;
    }
		
	else {
		let mut time_elapsed = Utc::now().timestamp() - des.assignment_time;
		let mut days_elapsed = time_elapsed/(60*60*24);
		match days_elapsed {
			0..=90 => return des.tokens,
			90..=180 => return 0.75*(des.tokens.to_f32()),
			180..=270 => return 0.5*(des.tokens.to_f32()),
			_ => return 0,
		
		}
		
	}
}	

