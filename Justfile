set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

#set allow-duplicate-recipe
#set positional-arguments
#set dotenv-load

set dotenv-filename := ".env"

# set dotenv-filename := ".env.intel"

set export := true

mx := "sc-meta"
Wallet := "wallets/hiro.pem"
MULTIVERSX_PROXY := "https://devnet-gateway.multiversx.com"
CHAIN := "D"
nonce := "0"
WALLET := env("WALLET", "hiro")

build *ARGS:
    {{ mx }} all build {{ ARGS }}

setup:
    {{ mx }} install all

#    installs all test gen deps

wallet:
    {{ mx }} wallet new --format pem --outfile ./wallets/hiro.pem

[group('deploy')]
deploy *ARGS: build
    WALLET={{ WALLET }} just i deploy

[group('deploy')]
deploy1 *ARGS="":
    mxpy contract deploy --pem {{ Wallet }} --gas-limit=60000000 --bytecode output/lottery.wasm --nonce {{ nonce }} --chain {{ CHAIN }} --proxy {{ MULTIVERSX_PROXY }} {{ ARGS }}

call *ARGS="":
    mxpy contract call erd1qqqqqqqqqqqqqpgqej707lk86ap308nyauejqhvvq0sgmpq8d8ss2nmpvf --function=beta_amount  --arguments "1"  --pem {{ Wallet }} --nonce {{ nonce }} --chain {{ CHAIN }} --proxy {{ MULTIVERSX_PROXY }} --gas-limit 1 {{ ARGS }}

[group('test')]
test:
    cargo test

gen-i *ARGS:
    {{ mx }} all snippets {{ ARGS }}

alias i := interactor

interactor cmd *ARGS:
    #!/bin/bash
    cd interactor
    WALLET={{ WALLET }} cargo run {{ cmd }} {{ ARGS }}

TOKEN := "LTRY-94ac38"

token recv_address *ARGS:
    mxpy tx send --recall-nonce \
    --receiver {{ recv_address }} \
    --value 0 \
    --data "ESDTTransfer@4c5452592d393461633338@10f0cf064dd592000000" \
    ---pem {{ Wallet }} --nonce {{ nonce }} --chain {{ CHAIN }} --proxy {{ MULTIVERSX_PROXY }} --gas-limit 1 {{ ARGS }}

build-bin:
    cargo build -p rust-interact --bin lottery --release

ci-clear-cache:
    gh cache list --limit 1000 | awk '{print $1}' | xargs -I {} gh cache delete {}

doc:
    sc-meta test-coverage --output ./coverage.md;
