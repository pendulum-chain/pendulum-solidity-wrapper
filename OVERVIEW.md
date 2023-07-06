Overview

Functional Requirements
Roles
- User (Contract Caller) - This is a user or a smart contract that expects to interact with a contract that implements the ERC-20 interface.

Features:
Use smart contracts to:
- query the chain (e.g. access balances)
- modify the state of the chain (e.g. transfer assets)
- No changes to smart contract implementation required

Use Case:
- Without the need to change the implementation, the solidity smart contract can use the on-chain assets, stored in pallets, by interacting with the wrapper contract.
