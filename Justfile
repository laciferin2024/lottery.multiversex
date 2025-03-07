set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

#set allow-duplicate-recipe
#set positional-arguments
#set dotenv-load

set dotenv-filename := ".env"

# set dotenv-filename := ".env.intel"

set export := true

mx := "sc-meta"
Wallet := "wallet/hiro.pem"
MULTIVERSX_PROXY := "https://devnet-gateway.multiversx.com"
CHAIN := "D"
nonce := "0"

build *ARGS:
    {{ mx }} all build {{ ARGS }}

wallet:
    {{ mx }} wallet new --format pem --outfile ./wallet/hiro.pem

deploy *ARGS="":
    mxpy contract deploy --pem {{ Wallet }} --gas-limit=60000000 --bytecode output/lottery.wasm --nonce {{ nonce }} --chain {{ CHAIN }} --proxy {{ MULTIVERSX_PROXY }} {{ ARGS }}

call *ARGS="":
    mxpy contract call erd1qqqqqqqqqqqqqpgq8cke70s93hmd9f68vx0lu6aurn2jppuy48jqj9kgj8 --function=init  --arguments "1"  --pem {{ Wallet }} --nonce {{ nonce }} --chain {{ CHAIN }} --proxy {{ MULTIVERSX_PROXY }} --gas-limit 1 {{ ARGS }}

test:
    cargo test

interactor *ARGS:
    {{ mx }} all snippets {{ ARGS }}
