use crate::types::{Operation, TransactionStatus};
use candid::{CandidType, Deserialize, Int, Nat, Principal};
use ic_kit::ic;
use chrono::prelude::*;
use chrono::Duration;

#[derive(Deserialize, CandidType, Debug, Clone)]

pub struct Designation{
	pub owner: Principal,
	pub role: String,  //the building block for our role and address list
	pub assignment_time: chrono::DateTime,
}

static mut DesignationList: Option<Vec<Designation>> = None;

#[init]
pub fn init_dl() {
    unsafe {
        DesignationList = Some(Vec::new());
    }
}



pub fn find_designation(wallet: Principal, dlist: Vec<Designation>) -> Designation {
	mut i: int32 = 0;
	while i < dlist.len(){
		if dlist[i].owner == wallet
			break;
		else 
			i += 1;
	}
	if i == dlist.len()
		ok(Designation{owner: target, role: "NA", assignment_time: Utc::now().timestamp()});
	else
		ok(dlist[i]);
}

pub fn remainder_limit(des: Designation, dlist: Vec<Designation>) -> Nat {
	if des.role == 'NA'
		return 0;
	else {
		let mut time_elapsed = (Utc::now().timestamp() - des.assignment_time);
		let mut days_elapsed = time_elapsed/(60*60*24);
		match days_elapsed {
			0...=90 => return 5%;
			90...=180 => return 2.5%;
			_ => return 0;
		
		}
		
	}
		
}