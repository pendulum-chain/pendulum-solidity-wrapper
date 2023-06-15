# Solidity Sample contract

This directory contains a sample contract written in solidity that can be deployed on the pendulum chain.
It is supposed to be used for testing the cross-contract communication between ink! and solidity contracts.

## Compiling the solidity contract

### Install solang

```bash
brew install hyperledger/hyperledger/solang
```

### Compile the contract

We need to specify the importmap for the openzeppelin contracts, because solang does not support the solidity import
syntax.

```bash
solang compile --target substrate --output solang/contracts erc20-test.sol --importmap @openzeppelin=openzeppelin-contracts/
```

As a result you will get a file called `Erc20Test.contract` in the `solang/contracts/` directory.
You can now deploy this contract to a parachain using Contracts UI. 