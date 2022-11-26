#!/bin/bash

# How to run: ADDRESS=thamerdridi.testnet ./demo.sh

NFT_ID=token-$RANDOM
NFT_CONTRACT=dev-1618440176640-7650905
FRACTOSE_CONTRACT=nft_lightency.testnet
SHARES_CONTRACT=dev-1618440176640-7650905-$NFT_ID.$FRACTOSE_CONTRACT
ADDRESS=nft_lightency.testnet

echo "1. Minting NFT with ID $NFT_ID ---------------------"
near call $NFT_CONTRACT nft_mint \
  '{
     "token_id": "'$NFT_ID'",
     "metadata": {
       "media": "https://near.org/wp-content/themes/near-19/assets/img/neue/kats-header.svg"
     }
  }' --accountId $ADDRESS --amount 1

echo "2. Granting escrow access to fractose contract $FRACTOSE_CONTRACT ---------------------"
near call $NFT_CONTRACT nft_approve \
  '{
    "token_id": "'$NFT_ID'",
    "account_id": "'$FRACTOSE_CONTRACT'"
  }' --accountId $ADDRESS --amount 1

near call nft_lightency.testnet fill_share_holders '{"sh_id":"firas.testnet", "nshares":"1000"}' --accountId $ADDRESS

near call nft_lightency.testnet fill_share_holders '{"sh_id":"alaaa.testnet", "nshares":"1000"}' --accountId $ADDRESS

near call nft_lightency.testnet fill_share_holders '{"sh_id":"thamerdridi.testnet", "nshares":"1000"}' --accountId $ADDRESS


# echo "3. Fractionalizing. NFT will be transferred and a shares contract will be created ---------------------"
near call $FRACTOSE_CONTRACT securitize '{"nft_contract_address": "'$NFT_CONTRACT'", "nft_token_id": "'$NFT_ID'", "shares_count": "3000", "decimals": 4, "exit_price": "30000" }' --accountId $ADDRESS --amount 5 --gas 300000000000000

# echo "4. The new NFT owner is ---------------------"
near view $NFT_CONTRACT nft_token '{ "token_id": "'$NFT_ID'"}' --accountId $ADDRESS


# echo "5. thamerdridi.testnet now own these fungible shares ---------------------"
near view $SHARES_CONTRACT ft_balance_of '{"account_id": "alaaa.testnet"}' --accountId $ADDRESS

# echo "6. thamer.testnet now own these fungible shares ---------------------"
near view $SHARES_CONTRACT ft_balance_of '{"account_id": "firas.testnet"}' --accountId $ADDRESS

near view $SHARES_CONTRACT ft_balance_of '{"account_id": "thamerdridi.testnet"}' --accountId $ADDRESS

