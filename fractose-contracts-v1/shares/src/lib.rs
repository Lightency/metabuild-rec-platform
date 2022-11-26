use std::convert::TryInto;

use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::{
    env, AccountId, Balance, PromiseOrValue, Promise,
    BorshStorageKey, PanicOnDefault, log,
    near_bindgen, ext_contract,
    collections::LazyOption,
    json_types::{ValidAccountId, U64, U128},
    borsh::{self, BorshDeserialize, BorshSerialize}
};

mod shares_metadata;
use shares_metadata::{SharesMetadata, SharesMetadataProvider, SHARES_FT_METADATA_SPEC};

near_sdk::setup_alloc!();

pub type TokenId = String;

#[ext_contract]
pub trait NonFungibleTokenCore {
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<U64>,
        memo: Option<String>,
    );
}

#[ext_contract]
pub trait Shares {
    fn cleanup(&mut self);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Shares {
    token: FungibleToken,
    metadata: LazyOption<SharesMetadata>
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FungibleToken,
    Metadata,
}

#[near_bindgen]

impl Shares {
    #[init]
    pub fn create(nft_contract_address: AccountId, nft_token_id: TokenId, owner_id: ValidAccountId, shares_count: U128, decimals: u8, share_price: U128, share_holders: Vec<ValidAccountId>, n_shares: Vec<U128>) -> Self {
        // TODO allow payment in NEP-141 fungible tokens
        assert!(!env::state_exists(), "Already initialized");

        let metadata = SharesMetadata {
            spec: SHARES_FT_METADATA_SPEC.to_string(),
            name: "Example NEAR fungible token".to_string(),
            symbol: "EXAMPLE".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            reference: None,
            reference_hash: None,
            decimals,

            // Shares FT specific metadata
            nft_contract_address: nft_contract_address.clone(),
            nft_token_id: nft_token_id.clone(),
            share_price,
            released: false
        };
        metadata.assert_valid();

        let mut this = Self {
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        };
        
        this.token.internal_register_account(&owner_id.clone().try_into().unwrap());
        this.token.internal_deposit(&owner_id.clone().try_into().unwrap(), shares_count.0);
        
        for i in 0..share_holders.len(){
        this.token.internal_register_account(&share_holders[i].clone().try_into().unwrap());
        this.token.internal_transfer(&owner_id.clone().try_into().unwrap(),&share_holders[i].clone().try_into().unwrap(), n_shares[i].0, Some("".to_string()));
        }
        

        // Emit event
        this.on_securitize(owner_id.to_string(), nft_contract_address, nft_token_id);

        this
    }

    /// Exit price in Near to redeem underlying NFT
    pub fn exit_price(&self) -> U128 {
        (self.ft_total_supply().0 * self.ft_metadata().share_price.0).into()
    }

    /// Near tokens required by a user in addition to held shares to redeem NFT
    pub fn redeem_amount_of(&self, from: ValidAccountId) -> U128 {
        let SharesMetadata { released, share_price, .. } = self.ft_metadata();
        assert!(!released, "token already redeemed");

        let user_shares = self.ft_balance_of(from);

        (self.exit_price().0 - user_shares.0 * share_price.0).into()
    }

    /// Returns balance Near tokens in vault
    /// NFTs can be redeemed by paying Near. These tokens are the new backing for shares
    pub fn vault_balance(&self) -> U128 {
        let SharesMetadata { released, share_price, .. } = self.ft_metadata();
        let balance = if !released {
            0
        } else {
            self.ft_total_supply().0 * share_price.0
        };

        balance.into()
    }

    /// Once NFT is redeemed by paying exit price, remaining shareholders get a
    /// share of the deposited Near tokens in proportion of their owned shares
    pub fn vault_balance_of(&self, from: ValidAccountId) -> U128 {
        let SharesMetadata { released, share_price, .. } = self.ft_metadata();
        let balance = if !released {
            0
        } else {
            let user_shares = self.ft_balance_of(from);
            user_shares.0 * share_price.0
        };

        balance.into()
    }

    /// Redeem NFT through owned shares or NEAR payment
    #[payable]
    pub fn redeem(&mut self) {
        let SharesMetadata { released, nft_token_id, nft_contract_address, .. } = self.ft_metadata();
        assert!(!released, "token already redeemed");

        let user_account = env::signer_account_id();

        let user_account_object: ValidAccountId = (user_account.clone()).try_into().unwrap();

        let payment_amount = env::attached_deposit();
        let redeem_amount = self.redeem_amount_of(user_account_object.clone()).0;

        // TODO allow payment in NEP-141 fungible tokens
        assert!(payment_amount >= redeem_amount, "insufficient payment amount");

        // Return change amount to redeemer
        let change_amount = payment_amount - redeem_amount;
        Promise::new(user_account.clone()).transfer(
            change_amount
        );

        // Set as redeemed
        let mut new_metadata = self.ft_metadata();
        new_metadata.set_as_released();

        self.metadata.replace(&new_metadata);

        // Burn shares
        let user_shares = self.ft_balance_of(user_account_object.clone());
        self.token.accounts.insert(&user_account, &0);
        self.token.total_supply -= user_shares.0;
        self.on_tokens_burned(user_account.clone(), user_shares.0);

        // Transfer NFT to redeemer
        non_fungible_token_core::nft_transfer(
            user_account_object.clone(),
            nft_token_id.clone(),
            None,
            None,
            &nft_contract_address,
            1,
            env::prepaid_gas() / 2
        );

        // Emit event
        self.on_redeem(user_account, nft_contract_address, nft_token_id.clone());

        // Cleanup
        self.cleanup();
    }

    /// Once NFT is redeemed by paying NEAR tokens, remaining shareholders can claim their share of NEAR in vault
    pub fn claim(&mut self) {
        let SharesMetadata { released,  nft_contract_address, nft_token_id, .. } = self.ft_metadata();
        assert!(released, "token not redeemed");

        let user_account = env::signer_account_id();
        let user_account_object: ValidAccountId = user_account.clone().try_into().unwrap();

        let user_shares = self.ft_balance_of(user_account_object.clone());
        assert!(user_shares.0 > 0, "nothing to claim");

        let claim_amount = self.vault_balance_of(user_account_object.clone());
        assert!(claim_amount.0 > 0, "balance has already been claimed");

        // Burn tokens- TODO check correctness
        self.token.accounts.insert(&user_account, &0);
        self.token.total_supply -= user_shares.0;
        self.on_tokens_burned(user_account.clone(), user_shares.0);

        // Emit event
        self.on_claim(user_account.clone(), nft_contract_address, nft_token_id, user_shares);

        // Transfer NEAR to user
        Promise::new(user_account.clone()).transfer(
            claim_amount.0
        ).then(shares::cleanup(
            &env::current_account_id(),
            0,
            env::prepaid_gas() / 2
        )); // TODO allow payment in NEP-141 fungible tokens

        // self.cleanup();
    }


    fn cleanup(&mut self) {
        // Emit event

        let shares_left = self.ft_total_supply();
        if shares_left.0 == 0 {
            // TODO Remove current contract address Fractose contract

            // Delete contract if all shares have been burnt
            Promise::new(env::current_account_id()).delete_account(
                // "system".into()
                env::signer_account_id() // Transfer any leftover NEAR tokens to redeemer
            );
        }
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

    fn on_securitize(&self, owner_address: AccountId, nft_contract_address: AccountId, nft_token_id: TokenId) {
        log!("Securitize({}, {}, {}, {})", owner_address, nft_contract_address, nft_token_id, env::current_account_id());
        // log!("Account @{} securitized NFT #{} on contract {}", owner_address, nft_token_id, nft_contract_address);
    }

    fn on_redeem(&mut self, redeemer_address: AccountId, nft_contract_address: AccountId, nft_token_id: TokenId) {
        log!("Redeem({}, {}, {}, {})", redeemer_address, nft_contract_address, nft_token_id, env::current_account_id());
    }

    fn on_claim(&mut self, claimant_address: AccountId, nft_contract_address: AccountId, nft_token_id: TokenId, shares_count: U128) {
        log!("Securitize({}, {}, {}, {}, {})", claimant_address, nft_contract_address, nft_token_id, env::current_account_id(), shares_count.0);
    }
}

near_contract_standards::impl_fungible_token_core!(Shares, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Shares, token, on_account_closed);

#[near_bindgen]
impl SharesMetadataProvider for Shares {
    fn ft_metadata(&self) -> SharesMetadata {
        self.metadata.get().unwrap()
    }
}