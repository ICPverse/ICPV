The token $ICPV for ICPverse, which will adhere to the IS20 standard, which is used for $ICP projects.

For vesting, changes in following files have been done without changing the existing code:
1. src\token\src\ledger.rs
2. src\token\src\api\dip20_transactions.rs
3. src\token\src\types.rs

A new struct called Designation has been created to keep a track of Founder, Advisor, Marketer, Investor wallets, considering these are expected to hold some locked tokens for a while.
