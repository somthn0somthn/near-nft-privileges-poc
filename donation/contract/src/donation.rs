use crate::Contract;
use crate::ContractExt;

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Balance, Gas, Promise, PromiseError,
};

pub const TGAS: u64 = 1_000_000_000_000;
pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

// Validator interface, for cross-contract calls
#[ext_contract(nft_creator)]
trait NftCreator {
    fn get_owner_by_token_id(&self, token_id: TokenId) -> AccountId;
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn privilege_check(&self, token_id: TokenId) -> Promise;
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
    #[payable] 
    pub fn donate(&mut self) -> U128 {
        // Get who is calling the method and how much $NEAR they attached
        let donor: AccountId = env::predecessor_account_id();
        let donation_amount: Balance = env::attached_deposit();

        let mut donated_so_far = self.donations.get(&donor).unwrap_or(0);

        let to_transfer: Balance = if donated_so_far == 0 {
          
            assert!(
                donation_amount > STORAGE_COST,
                "Attach at least {} yoctoNEAR",
                STORAGE_COST
            );

            donation_amount - STORAGE_COST
        } else {
            donation_amount
        };

        // Persist in storage the amount donated so far -- do we need to prevent
        // hitting a max storage amoutnt
        donated_so_far += donation_amount;
        self.donations.insert(&donor, &donated_so_far);

        log!(
            "Thank you {} for donating {}! You donated a total of {}",
            donor.clone(),
            donation_amount,
            donated_so_far
        );

        log!(
            "for reference, this is the transfer amount: {}. 
          You can withdrawal/trasfer it by calling the withdrawal method",
            to_transfer
        );

        U128(donated_so_far)
    }

    //HOW MUCH GAS IS USED ??? check rpc
    pub fn privilege_check(&mut self) -> Promise {
        let promise = nft_creator::ext(self.nft_deployed_account.clone())
            .with_static_gas(Gas(5 * TGAS))
            .get_owner_by_token_id(self.token_id.clone());

        return promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .privilege_check_callback(),
        );
    }

    #[private]
    pub fn privilege_check_callback(
        &mut self,
        #[callback_result] call_result: Result<AccountId, PromiseError>,
    ) {
        if call_result.is_err() {
            log!("There was an error querying the token owner");
            /*  return "Error".to_string(); */
        }

        let account_id: AccountId = call_result.unwrap();
        log!(
            "from privilege_check_callback, token holder account is {} and privileged_account is currently set to {}",
            account_id,
            self.privileged_account
        );
        self.privileged_account = account_id;
        log!(
            "from privilege_check_callback, privileged_account is now set to {}",
            self.privileged_account
        );
    }

    //#[privileged_func_call]
    pub fn withdrawal(&mut self) -> U128 {
        
        let caller = env::predecessor_account_id();
        if self.privileged_account.clone() != caller {
            panic!(
                "method access denied - no NFT in account - trying running privilege check first"
            );
        }

        let mut withdrawal_amount: u128 = 0;

        let values = self.donations.values();

        for d in values {
            let val: u128 = u128::from(d);
            withdrawal_amount += val;
            log!(
                "incrementing withdrawal amount, currently its {}",
                withdrawal_amount
            );
        }

        log!(
            "For reference the folded withdrawal_amount is {}",
            withdrawal_amount
        );

        assert!(
            withdrawal_amount > STORAGE_COST,
            "The donation amount is {}, wait until its more than {}",
            withdrawal_amount,
            STORAGE_COST
        );

        let transfer_amount: Balance = Balance::from(withdrawal_amount);
        log!("for ref, the transfer amount is {}", transfer_amount);

        let beneficiary = self.beneficiary.clone();

        //change to ValidAccountId check????????
        assert!(
            beneficiary == caller,
            "Beneficiary is not the caller, meaning transfer could fail"
        );

        Promise::new(beneficiary).transfer(transfer_amount);

      
        //UnorderedMaps need to be initialized with unique keys (as byte strings)
        //if not, the contract will hang so thats occuring here --
        // you could also apply a sha256 hash to a random value  
        let block_height = env::block_height();
        let bytes = block_height.to_be_bytes();
        self.donations = UnorderedMap::new(bytes.as_ref());
        log!("self.donations is currently {:?}", &self.donations);

        U128(withdrawal_amount)
    }

    pub fn get_donation_for_account(&self, account_id: AccountId) -> Donation {
        Donation {
            account_id: account_id.clone(),
            total_amount: U128(self.donations.get(&account_id).unwrap_or(0)),
        }
    }

    pub fn number_of_donors(&self) -> u64 {
        self.donations.len()
    }

    pub fn get_donations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Donation> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through donation
        self.donations
            .keys()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            .map(|account| self.get_donation_for_account(account))
            //since we turned map into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}