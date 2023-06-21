# Solidity Sample contract

This directory contains a sample contract written in solidity that can be deployed on the pendulum chain.
It is supposed to be used for testing the cross-contract communication between ink! and solidity contracts.

The sample contract is a simple pool contract that allows users to deposit and withdraw ERC20 tokens.

## Compiling the solidity contract

### Install solang

```bash
brew install hyperledger/hyperledger/solang
```

### Compile the contract

We need to specify the importmap for the openzeppelin contracts, because solang does not support the solidity import
syntax.

```bash
solang compile --target substrate --output solang/contracts pool.sol --importmap @openzeppelin=openzeppelin-contracts/
````

Or from the root directory

```bash
solang compile --target substrate --output target/solang/contracts solidity-sample/pool.sol --importmap @openzeppelin=solidity-sample/openzeppelin-contracts/
```

### Deploying the contract

As a result you will get a file called `Pool.contract` in the `solang/contracts/` directory.
You can now deploy this contract to a parachain using Contracts UI.
Before you deploy the pool contract, you should deploy two different ERC-20 contracts first, e.g. for the `Native` and
the `XCM(0)` Token.
When you deploy the pool contract, you need to specify the addresses of the two ERC-20 contracts as constructor
arguments.

### Interacting with the contract

You can now interact with the contract using the Contracts UI.
Before depositing tokens, you need to approve the pool contract to transfer tokens on your behalf.
In order to do so, you need to call the `approve` function of the two ERC-20 contracts, using the same account that you
want to use for depositing tokens.
Afterwards, you can deposit tokens using the `deposit` function of the pool contract.
You can also withdraw tokens using the `withdraw` function of the pool contract.