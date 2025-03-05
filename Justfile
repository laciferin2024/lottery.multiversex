set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
#set allow-duplicate-recipe
#set positional-arguments
#set dotenv-load
set dotenv-filename := ".env"
# set dotenv-filename := ".env.intel"
set export

mx := "sc-meta"
Wallet := "wallet/hiro.pem"

MULTIVERSX_PROXY := "https://testnet-gateway.multiversx.com"
CHAIN := "D"
nonce := "0"


build:
  {{mx}} all build

wallet:
  {{mx}} wallet new --format pem --outfile ./wallet/hiro.pem


deploy *ARGS:
  mxpy contract deploy --pem {{Wallet}} --gas-limit=60000000 --bytecode output/lottery.wasm --nonce {{nonce}} --chain {{CHAIN}} --proxy {{MULTIVERSX_PROXY}}