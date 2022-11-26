use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::ext_contract;
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, log, near_bindgen, AccountId, BorshStorageKey, Gas, Promise};
use std::convert::TryInto;

pub const TGAS: u64 = 1_000_000_000_000;
pub type TokenId = String;
pub type AccountAndTokenId = String;

#[ext_contract(shares)]
pub trait Shares {
    fn create(
        &mut self,
        nft_contract_address: AccountId,
        nft_token_id: TokenId,
        owner_id: AccountId,
        shares_count: U128,
        decimals: u8,
        share_price: U128,
        share_holders: Vec<AccountId>,
        n_shares: Vec<U128>,
    ) -> Self;
}

#[ext_contract(non_fungible_token)]
pub trait NonFungibleTokenCore {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<U64>,
        memo: Option<String>,
    );
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKeyEnum {
    NftToSharesAddress,
    SharesToNftAddress,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Fractose {
    pub nft_to_shares_address: LookupMap<AccountAndTokenId, AccountId>,
    pub shares_to_nft_address: LookupMap<AccountId, AccountAndTokenId>,
    pub sharess_holders: UnorderedMap<AccountId, U128>,
}

impl Default for Fractose {
    fn default() -> Self {
        Self {
            nft_to_shares_address: LookupMap::<AccountAndTokenId, AccountId>::new(
                StorageKeyEnum::NftToSharesAddress,
            ),
            shares_to_nft_address: LookupMap::<AccountId, AccountAndTokenId>::new(
                StorageKeyEnum::SharesToNftAddress,
            ),
            sharess_holders: UnorderedMap::new(b"share_holders".to_vec()),
        }
    }
}

#[near_bindgen]
impl Fractose {
    /// Securitize an approved NFT into shares
    ///
    /// # Parameters
    ///
    /// - `nft_contract_address`: Address of NFT contract
    /// - `nft_token_id`: Address of the NFT to be securitized
    /// - `shares_count`: Number of fungible shares to be created
    /// - `decimals`: Number of decimal places in share fungible tokens
    /// - `exit_price`: Underlying NFT can be retrieved by paying the exit price
    #[payable]
    pub fn securitize(
        &mut self,
        nft_contract_address: String,
        nft_token_id: TokenId,
        shares_count: U128,
        decimals: u8,
        exit_price: U128,
    ) {
        log!(
            "Securitizing token {} from contract {}",
            nft_token_id,
            nft_contract_address
        );

        // Check whether parameters are valid
        assert!(exit_price.0 > 0, "invalid exit price");
        assert!(shares_count.0 > 0, "invalid shares count");
        assert!(
            exit_price.0 % shares_count.0 == 0,
            "share price cannot be fractional"
        );

        let share_price = exit_price.0 / shares_count.0;
        log!("Share price: {}", share_price);

        // Include NFT ID
        let shares_contract =
            get_shares_contract_name(nft_contract_address.clone(), nft_token_id.clone());
        
        log!("{}", shares_contract);
        // Deploy shares contract
        Promise::new(shares_contract.clone().parse().unwrap())
            .create_account()
            .transfer(25_00000000000000000000000)
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(include_bytes!("../../shares/res/nft_shares.wasm").to_vec());

        
        let owner: AccountId = env::signer_account_id().try_into().unwrap();
        
        // Call shares contract constructor
        
        shares::ext(shares_contract.clone().try_into().unwrap())
            .with_static_gas(Gas(2 * TGAS))
            .create(
                nft_contract_address.clone().try_into().unwrap(),
                nft_token_id.clone(),
                owner,
                shares_count,
                decimals,
                share_price.into(),
                self.sharess_holders.keys_as_vector().to_vec(),
                self.sharess_holders.values_as_vector().to_vec(),
            );

        // Save metadata
        let nft_address = get_nft_address(
            nft_contract_address.try_into().unwrap(),
            nft_token_id.clone(),
        );

        self.nft_to_shares_address
            .insert(&nft_address, &shares_contract.clone().try_into().unwrap());
        self.shares_to_nft_address
            .insert(&shares_contract.clone().try_into().unwrap(), &nft_address);

        non_fungible_token::ext("nft-lightency.testnet".to_string().try_into().unwrap())
            .with_attached_deposit(1)
            .with_static_gas(Gas(2 * TGAS))
            .nft_transfer(
                shares_contract.clone().try_into().unwrap(),
                nft_token_id.clone(),
                None,
                None,
            );
    }

    pub fn fill_share_holders(&mut self, sh_id: AccountId, nshares: U128) {
        self.sharess_holders.insert(&sh_id, &nshares);
    }
}

fn get_shares_contract_name(_target: String, token_id: TokenId) -> String {
    let prefix = _target.replace(".", "-");
    format!("{}-{}.{}", prefix, token_id, env::current_account_id())
}

fn get_nft_address(contract_address: AccountId, token_id: TokenId) -> String {
    format!("{}/{}", contract_address, token_id)
}