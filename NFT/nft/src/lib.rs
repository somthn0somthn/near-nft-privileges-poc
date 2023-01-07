use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}


pub const DEFAULT_TITLE: &str = "This is the default title";
pub const DEFAULT_DESCRIPTION: &str = "This is the default descripton"; 
pub const DEFAULT_MEDIA: &str = "https://ibb.co/cbPxWf3";
pub const DEFAULT_COPIES: u64 = 1;
//for future reference, this is a text-encoded svg, this is a handy tool for encoding
//svg, although you must prepend 'data:image/svg+xml,' ::  https://yoksel.github.io/url-encoder/ 
pub const DATA_IMAGE_SVG_MLABS_ICON: &str = "data:image/svg+xml,%3C%3Fxml version='1.0' standalone='no'%3F%3E%3C!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 20010904//EN' 'http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd'%3E%3Csvg version='1.0' xmlns='http://www.w3.org/2000/svg' width='300.000000pt' height='300.000000pt' viewBox='0 0 300.000000 300.000000' preserveAspectRatio='xMidYMid meet'%3E%3Cg transform='translate(0.000000,300.000000) scale(0.100000,-0.100000)'%0Afill='%23000000' stroke='none'%3E%3Cpath d='M1523 2111 c-73 -40 -140 -91 -193 -146 l-45 -47 60 38 c63 40 276%0A144 294 144 18 0 12 19 -8 30 -30 16 -52 12 -108 -19z'/%3E%3Cpath d='M1880 2115 c-162 -30 -410 -135 -590 -252 -87 -56 -310 -214 -310%0A-219 0 -1 66 29 146 67 236 114 462 182 652 196 l87 6 80 89 c119 133 117 128%0A59 127 -27 0 -83 -7 -124 -14z'/%3E%3Cpath d='M2470 1569 c-58 -23 -100 -85 -100 -146 0 -77 49 -118 183 -157 95%0A-28 123 -74 70 -116 -23 -19 -37 -21 -81 -17 -29 3 -73 17 -100 32 l-47 26%0A-22 -28 c-13 -15 -23 -30 -23 -33 0 -10 69 -49 114 -64 143 -49 266 14 266%0A136 0 72 -38 108 -154 143 -86 26 -126 51 -126 79 0 28 13 47 41 61 39 21 83%0A19 131 -6 l41 -21 23 29 c20 25 21 30 7 45 -35 39 -164 60 -223 37z'/%3E%3Cpath d='M270 1521 c0 -40 0 -40 42 -43 l42 -3 85 -197 c47 -109 89 -198 92%0A-198 3 0 43 87 90 194 l84 194 3 -204 2 -204 40 0 40 0 0 250 0 251 -67 -3%0A-67 -3 -60 -137 c-33 -76 -62 -138 -65 -138 -4 0 -33 63 -66 140 l-60 140 -67%0A0 -68 0 0 -39z'/%3E%3Cpath d='M880 1310 l0 -250 185 0 185 0 0 40 0 40 -145 0 -145 0 0 210 0 210%0A-40 0 -40 0 0 -250z'/%3E%3Cpath d='M1534 1483 c18 -43 47 -111 64 -150 l31 -73 -85 0 -84 0 15 36 c8 20%0A15 38 15 40 0 2 -20 4 -43 4 l-44 0 -51 -122 c-28 -68 -54 -131 -58 -140 -5%0A-15 0 -18 38 -18 l43 0 24 60 24 60 121 0 121 0 24 -60 24 -60 44 0 c23 0 43%0A2 43 4 0 2 -47 113 -103 247 l-104 244 -46 3 -46 3 33 -78z'/%3E%3Cpath d='M1880 1520 l0 -40 139 0 c114 0 143 -3 155 -16 27 -26 24 -71 -6 -96%0A-17 -15 -41 -18 -154 -18 l-134 0 0 -40 0 -40 138 0 c125 0 141 -2 157 -20 24%0A-27 22 -70 -5 -92 -19 -15 -42 -18 -156 -18 l-134 0 0 -40 0 -40 140 0 c155 0%0A178 6 224 60 33 40 36 119 7 164 -19 29 -19 30 0 54 26 34 26 116 -1 159 -33%0A55 -62 63 -225 63 l-145 0 0 -40z'/%3E%3Cpath d='M270 1205 l0 -145 40 0 40 0 0 145 0 145 -40 0 -40 0 0 -145z'/%3E%3C/g%3E%3C/svg%3E%0A";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with default example metadata.
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token by MLabs".to_string(),
                symbol: "MLABS".to_string(),
                icon: Some(DATA_IMAGE_SVG_MLABS_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    pub fn get_owner_by_token_id(&self, token_id: TokenId) -> AccountId {
        let map = &self.tokens.owner_by_id;
        let account_id = map.get(&token_id).unwrap();
        return account_id
    }

    
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        //token_metadata: TokenMetadata,
    ) -> Token {
        //this could be reconfigured or refactored to be parameterized from the function call.
        let meta = TokenMetadata {
            title:Some(DEFAULT_TITLE.to_string()),
            description: Some(DEFAULT_DESCRIPTION.to_string()),
            media: Some(DEFAULT_MEDIA.to_string()),
            media_hash: None,
            copies: Some(DEFAULT_COPIES),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        self.tokens.internal_mint(token_id, receiver_id, Some(meta))
    }
}

//these macros provide basic NFT utilities to the collection
//core needs to be implemented by hadn with the transfer function set to call xxc privilege_check
near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
