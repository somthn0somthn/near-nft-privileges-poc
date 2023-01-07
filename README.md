# near-nft-privileges-poc

## NFT privileges POC on the NEAR blockchain

### How to use this POC-demo repo

This project hopes to serve as a proof-of-concept of dApp privileges based on NFT ownership on [NEAR](https://near.org/). NEAR is an asynchronous blockchain that relies on sharding and cross-shard function calls/artifacts for throughput and usability. The simple contracts in this repo are capable of operating independently. However, this project aims to highlight how they may interoperate via the cross-contract functionality built into NEAR. These contracts pull from the example contracts in the NEAR dev documentation available [here](https://github.com/near-examples/NFT).

Briefly, this contract suite consists of a basic NFT collection contract capable of querying a token's owner. This querying is then integrated into a crowdsourcing-like donation contract. This donation allows NEAR holders to donate to a beneficiary -- likely the NFT holder. The NFT can reassign the `beneficiary` account using a `change_beneficiary` function and similarly instigate withdrawals. Privileges are transfered to the new owner when the NFT is transferred to another wallet.

To get started:

1) You'll need to get the NEAR-CLI and initialize a master account on the testnet if you haven't already. See [here](https://docs.near.org/tools/near-cli).

2) Initialize the NFT contract account
```=bash
near create-account <subaccount>.<masteraccount> --masterAccount <masteraccount>
near state <subaccount>.<masteraccount>.testnet
```

3) From the cargo project dir, build the NFT contract using the build script
```=bash
cd ../<NFT project cargo dir>
./scripts/build.sh
```

4) Deploy and initialize the contract
```=bash
near deploy --accountId <deployment account> --wasmFile <path to compiled wasmfile ending in non_fungible_token.wasm>
near call <deployment account> new_default_meta '{"owner_id": "<owner accountId>"}' --accountId <deployment account>
near view <deployment account> nft_metadata
```

5) Mint a token
```=bash
near call <deployment account> nft_mint '{"token_id": "0", "receiver_id": "<accountId>"}' --accountId <accountId> --deposit 0.1
```

6) Create a recipient account -> this will become the privileged account after it receives the NFT
```=bash
near create-account <subaccount>.<masteraccount> --masterAccount <masteraccount>
near call <deployment account> nft_transfer '{"token_id": "0", "receiver_id": "<accountId>", "memo": "enjoy your nft"}' --accountId <deployment account> --depositYocto 1
near call <deployment account> get_owner_by_token_id '{"token_id": "0"}' --accountId <accountId>
```

7) Create an account for the Donation contract, build the contract, and deploy
```=bash
cd ../<donation project dir>/contract
./build.sh
near create-account <donation account> --masterAccount <accountId>
near deploy --accountId <donation account> --wasmFile <wasm path>
```

8) Initialize the contract
```=bash
near call <donation account> init '{"beneficiary": "<accountId>", "nft_deployed_account": "<deployed account>", "token_id": "0"}' --accountId <donation account>
```

9) Check the getter functions
```=bash
near call <donation account> get_beneficiary --accountId <account>
```

10) The so-called `privileged_account` of the donation contract can change the donation beneficiary and instigate a withdrawal action. Initially, the account that deploys the donation contract is set as the `privileged_account`. Calling the `privilege_check` function will designate the `privileged_account` as the NFT holder using a promisary function call to the original NFT contract deployed at first. Any account can call this function.
```=bash
near call <donation account> privelege_check --accountId <account>
near call <donation account> get_priveleged_account --accountId <account>
```

11) Change the donation beneficiary
```=bash
near call <donation account> change_beneficiary '{"beneficiary": "<account>"}' --accountId <priveleged account>
near call <donation account> get_beneficiary --accountId <account>
```

12) Donate to the contract & query the donations
```=bash
near call <donation account> donate --accountId <account> --amount=<token amount>
near call <donation account> get_donations --accountId <account>
```

13) Withdrawal 
```=bash
near call <donation account> withdrawal --accountId <priveleged account>
near call <donation account> get_donations --accountId <account>
```

14) Transfer the NFT, reset the privileges, change the beneficiary, and verify. Note: currently permissions have to be reset manually, but I hope to enforce this automatically moving forward
```=bash
near call <deployment account> nft_transfer '{"token_id": "0", "receiver_id": "<account>", "memo": "enjoy"}' --accountId <receiver acount> --depositYocto 
near call <donation account> privelege_check --accountId <account>
near call <donation account> get_priveleged_account --accountId <account>
near call <donation account> change_beneficiary '{"beneficiary": "<account>"}' --accountId <priveleged account>
near call <donation account> get_beneficiary --accountId <account>
```

15) Redonate and re-withdrawal
```=bash
near call <donation account> donate --accountId <account> --amount=<token amount>
near call <donation account> withdrawal --accountId <priveleged account>
```
