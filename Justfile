set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
#set allow-duplicate-recipe
#set positional-arguments
#set dotenv-load
set dotenv-filename := ".env"
# set dotenv-filename := ".env.intel"
set export

mx := "sc-meta"

build:
  {{mx}} all build

wallet:
  {{mx}} wallet new --format pem --outfile ./wallet/hiro.pem