use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, ext_contract, log, near_bindgen, AccountId};

mod donation;

#[ext_contract(nft_creator)]
trait NftCreator {
    fn get_owner_by_token_id(&self, token_id: TokenId) -> AccountId;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    //person receiving donation
    pub beneficiary: AccountId,
    pub donations: UnorderedMap<AccountId, u128>,
    //extract into constant
    pub nft_deployed_account: AccountId,
    pub privileged_account: AccountId,
    pub token_id: TokenId,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            beneficiary: "v1.faucet.nonofficial.testnet".parse().unwrap(),
            donations: UnorderedMap::new(b"d"),
            nft_deployed_account: AccountId::new_unchecked(
                "nftcreator.lukemlabs.testnet".to_string(),
            ),
            privileged_account: AccountId::new_unchecked("lukemlabs.testnet".to_string()),
            token_id: "0".to_string(),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn init(
        beneficiary: AccountId,
        nft_deployed_account: AccountId,
        token_id: TokenId,
    ) -> Self {
        let holder = env::predecessor_account_id().clone();
        Self {
            beneficiary,
            donations: UnorderedMap::new(b"d"),
            nft_deployed_account,
            privileged_account: holder,
            token_id,
        }
    }

    //getters and setters

    pub fn get_beneficiary(&self) -> AccountId {
        self.beneficiary.clone()
    }

    //#[privileged_func_call]
    pub fn change_beneficiary(&mut self, beneficiary: AccountId) {
        let caller = env::predecessor_account_id();
        log!(
        "called chang_beneficiary (privileged function), calling_account is {}, and privileged_account is currently {:?}",
        &caller,
        &self.privileged_account
    );
        if self.privileged_account.clone() != caller {
            panic!(
                "method access denied - no NFT in account - trying running privilege check first"
            );
        }
        log!(
            "from change_ben, at the beginngin self.ben is {} and ben is {}",
            self.beneficiary,
            beneficiary
        );
        self.beneficiary = beneficiary;
    }

    pub fn get_nft_deployed_account(&self) -> AccountId {
        self.nft_deployed_account.clone()
    }

    pub fn get_privileged_account(&self) -> AccountId {
        self.privileged_account.clone()
    }
}