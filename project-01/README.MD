# hello anchor


Lets just straight to anchor installation guide.

https://www.anchor-lang.com/docs/installation

thise guide assumes for now:

```
solana --version
solana-cli 2.1.22 (src:26944979; feat:1416569292, client:Agave)

rustc --version
rustc 1.87.0 (17067e9ac 2025-05-09)

agave-install list  
stable-2694497991ec20761d820f8a39933222a1017bf6
1.18.26
stable-3861dcebbc15c3d6022208b227bdf13e797af941
2.1.0

agave-install list 
stable-2694497991ec20761d820f8a39933222a1017bf6
1.18.26
stable-3861dcebbc15c3d6022208b227bdf13e797af941
2.1.0


avm --version
avm 0.31.1

anchor --version
anchor-cli 0.31.1

node --version
v24.1.0

yarn --version
1.22.22

sw_vers #if you are using macOS
ProductName:		macOS
ProductVersion:		15.5
BuildVersion:		24F74
```



```
solana config set --url localhost

solana-keygen new --no-bip39-passphrase #we dont set up a pw for now, because we are only on localhost


Solana config get

Config File: /Users/$USER/.config/solana/cli/config.yml
RPC URL: http://localhost:8899 
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /Users/$USER/.config/solana/id.json 
Commitment: confirmed

```


```
solana address
GajyV26ZW5LDEzv2oeiBqWBdZeDJTW6DMoqPEB3MsWos ##example address. pls use your own address


#start your own solana test validator in the current working directory. it will always create a test-ledger.
#its already in the git ignore
#feel free to remove that directory if you want to reset the state

test-ledger

Log: test-ledger/validator.log
⠋ Initializing...                                                                                                  Waiting for fees to stabilize 1...
Identity: CaUdYPCrvC34jw4eTA3iQq91Mn6nnuz6KrFiPc83SFTm
Genesis Hash: HvNwXHdohTpFdPKzcKLrJpyWzdXpe5Z3wxg6D7q67ZKW
Version: 2.1.22
Shred Version: 60192
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
WebSocket PubSub URL: ws://127.0.0.1:8900
⠒ 00:00:35 | Processed Slot: 75 | Confirmed Slot: 75 | Finalized Slot: 44 | Full Snapshot Slot: - | Incremental Sna


```

```
solana config set -ul
solana balance
500000000 SOL

solana airdrop 2
Requesting airdrop of 2 SOL

Signature: 4sv34hs1e3egYLg7wPwK72ZR7LsqRpEgLm929Bp6g4HxzTbB4sxmDyo7z4s9PKTHdoDgMkuPDepcmgQFjDK7b7hq

500000002 SOL
```


# now here comes the fun party. 
`anchor init anchor-contracts`
