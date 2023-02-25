# Test script for Juno Smart Contracts (By @Reecepbcups)
# ./github/workflows/e2e.yml
#
# sh ./e2e/test_e2e.sh
#
# NOTES: anytime you use jq, use `jq -rc` for ASSERT_* functions (-c removes format, -r is raw to remove \" quotes)

# get functions from helpers file 
# -> query_contract, wasm_cmd, mint_cw721, send_nft_to_listing, send_cw20_to_listing
source ./e2e/helpers.sh

CONTAINER_NAME="wasm-reviews"
BINARY="docker exec -i $CONTAINER_NAME junod"
DENOM='ujunox'
JUNOD_CHAIN_ID='testing'
JUNOD_NODE='http://localhost:26657/'
# this should be more like 5mil, testing
JUNOD_COMMAND_ARGS="--gas 5000000 --fees 7000$DENOM -y -b block --chain-id $JUNOD_CHAIN_ID --node $JUNOD_NODE --output json --from test-user"
export KEY_ADDR="juno1hj5fveer5cjtn4wd6wstzugjfdxzl0xps73ftl"


# ========================
# === Contract Uploads ===
# ========================
function upload_review_contract {
    echo "Storing contract..."    
    UPLOAD=$($BINARY tx wasm store /reviews.wasm $JUNOD_COMMAND_ARGS | jq -r '.txhash') && echo $UPLOAD
    BASE_CODE_ID=$($BINARY q tx $UPLOAD --output json | jq -r '.logs[0].events[] | select(.type == "store_code").attributes[] | select(.key == "code_id").value') && echo "Code Id: $BASE_CODE_ID"
    
    TX_HASH=$($BINARY tx wasm instantiate "$BASE_CODE_ID" '{"max_saved":2,"cooldown_blocks":0}' --label "reviews" $JUNOD_COMMAND_ARGS --admin "$KEY_ADDR" | jq -r '.txhash') && echo $TX_HASH

    export CONTRACT=$($BINARY query tx $TX_HASH --output json | jq -r '.logs[0].events[0].attributes[0].value') && echo "CONTRACT: $CONTRACT"    
}

# === COPY ALL ABOVE TO SET ENVIROMENT UP LOCALLY ====

# =============
# === LOGIC ===
# =============

start_docker && compile_and_copy
add_accounts
upload_review_contract


$BINARY q wasm contract-state smart $CONTRACT '{"get_reviews":{}}'

# write a review
$BINARY tx wasm execute $CONTRACT '{"review":{"text":"This is my review, I think joe chain is so great thx Joe"}}' $JUNOD_COMMAND_ARGS
$BINARY tx wasm execute $CONTRACT '{"review":{"text":"2nd review bc its so great later"}}' $JUNOD_COMMAND_ARGS

$BINARY q wasm contract-state smart $CONTRACT '{"get_reviews":{}}'