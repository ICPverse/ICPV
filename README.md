Special Thanks to:
1. Maximkaaa (Max) from the Infinity Swap team, majority of whose code we are working on
2. glinuxdev for helping with valuable debugging and other suggestions

The token $ICPV for ICPverse, which will adhere to the IS20 standard, which is used for $ICP projects.

For vesting, changes in following files have been done without changing the existing code:
1. src\token\src\ledger.rs
2. src\token\src\api\dip20_transactions.rs
3. src\token\src\types.rs

A new struct called Designation has been created to keep a track of Founder, Advisor, Marketer, Investor wallets, considering these are expected to hold some locked tokens for a while.

The token standard remains IS20, but with added functionality to implement vesting of tokens, and more detailed tracking of important wallets: like those of the founders, investors, the marketing and treasury wallets of the project, and so on. The following methods have been added and exposed:
sizeDl : to check the size of the DesignationList
transferForFounder
transferForAdvisor
transferForInvestor
transferForPrivate
transferForPublic
transferForTreasury
transferForMarketing

The original format from the IS team has been left intact, and someone should be able to folk this repository, and run those commands to deploy token_factory, as mentioned on their Official GitHub, and by changing the values of vesting period in our code, anyone can implement vesting for IS20 as per their needs.
For further instructions on how to run the code as it is with no changes, please refer to their repository:
https://github.com/infinity-swap/IS20

Note: While fully functional code, this repository is defunct and has been replaced by the IV20 repository as the reference code from the ICPverse Team.
