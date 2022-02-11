use crate::types::TxRecord;
use crate::types::Designation;
use candid::{CandidType, Deserialize, Nat, Principal};
use num_traits::ToPrimitive;
use chrono::prelude::*;
use chrono::Duration;

const MAX_HISTORY_LENGTH: usize = 1_000_000;
const HISTORY_REMOVAL_BATCH_SIZE: usize = 10_000;

#[derive(Default, CandidType, Deserialize)]
pub struct Ledger {
    history: Vec<TxRecord>,
    vec_offset: Nat,
}

impl Ledger {
    pub fn len(&self) -> Nat {
        self.vec_offset.clone() + self.history.len()
    }

    fn next_id(&self) -> Nat {
        self.vec_offset.clone() + self.history.len()
    }

    pub fn get(&self, id: &Nat) -> Option<TxRecord> {
        self.history.get(self.get_index(id)?).cloned()
    }

    pub fn get_range(&self, start: &Nat, limit: &Nat) -> Vec<TxRecord> {
        let start = match self.get_index(start) {
            Some(v) => v,
            None => {
                if *start > self.vec_offset.clone() {
                    usize::MAX
                } else {
                    0
                }
            }
        };

        let limit = limit.0.to_usize().unwrap_or(usize::MAX);
        self.history
            .iter()
            .skip(start)
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TxRecord> {
        self.history.iter()
    }

    fn get_index(&self, id: &Nat) -> Option<usize> {
        if *id < self.vec_offset {
            None
        } else {
            let index = id.clone() - self.vec_offset.clone();
            index.0.to_usize()
        }
    }

    pub fn transfer(&mut self, from: Principal, to: Principal, amount: Nat, fee: Nat) -> Nat {
        let id = self.next_id();
        self.push(TxRecord::transfer(id.clone(), from, to, amount, fee));

        id
    }
	
	pub fn transfer_to_investor(&mut self, from: Principal, to: Principal, amount: Nat, fee: Nat) -> Nat {
        let id = self.next_id();
        self.push(TxRecord::transfer(id.clone(), from, to, amount, fee));
		DesignationList.push(Designation{owner: to, role: 'investor', assignment_time: Utc::now()});
        id
    }
	
	pub fn transfer_to_founder(&mut self, from: Principal, to: Principal, amount: Nat, fee: Nat) -> Nat {
        let id = self.next_id();
        self.push(TxRecord::transfer(id.clone(), from, to, amount, fee));
		DesignationList.push(Designation{owner: to, role: 'founder', assignment_time: Utc::now()});
        id
    }
	
	pub fn transfer_to_advisor(&mut self, from: Principal, to: Principal, amount: Nat, fee: Nat) -> Nat {
        let id = self.next_id();
        self.push(TxRecord::transfer(id.clone(), from, to, amount, fee));
		DesignationList.push(Designation{owner: to, role: 'advisor', assignment_time: Utc::now()});
        id
    }

    pub fn transfer_from(
        &mut self,
        caller: Principal,
        from: Principal,
        to: Principal,
        amount: Nat,
        fee: Nat,
    ) -> Nat {
        let id = self.next_id();
        self.push(TxRecord::transfer_from(
            id.clone(),
            caller,
            from,
            to,
            amount,
            fee,
        ));

        id
    }

    pub fn approve(&mut self, from: Principal, to: Principal, amount: Nat, fee: Nat) -> Nat {
        let id = self.next_id();
        self.push(TxRecord::approve(id.clone(), from, to, amount, fee));

        id
    }

    pub fn mint(&mut self, from: Principal, to: Principal, amount: Nat) -> Nat {
        let id = self.len();
        self.push(TxRecord::mint(id.clone(), from, to, amount));
		init_dl();
		
        id
    }

    pub fn burn(&mut self, caller: Principal, amount: Nat) -> Nat {
        let id = self.next_id();
        self.push(TxRecord::burn(id.clone(), caller, amount));

        id
    }

    pub fn auction(&mut self, to: Principal, amount: Nat) {
        let id = self.next_id();
        self.push(TxRecord::auction(id, to, amount))
    }

    fn push(&mut self, record: TxRecord) {
        self.history.push(record);
        if self.len() > MAX_HISTORY_LENGTH + HISTORY_REMOVAL_BATCH_SIZE {
            // We remove first `HISTORY_REMOVAL_BATCH_SIZE` from the history at one go, to prevent
            // often relocation of the history vec.
            // This removal code can later be changed to moving old history records into another
            // storage.
            self.history = self.history[HISTORY_REMOVAL_BATCH_SIZE..].into();
            self.vec_offset += HISTORY_REMOVAL_BATCH_SIZE;
        }
    }
}
