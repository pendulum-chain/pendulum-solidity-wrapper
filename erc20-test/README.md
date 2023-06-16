# erc20-test

This contract is used to test if our native-token-wrapper-solidity contract does indeed implement our `IERC20` interface, which is a combination of openzeppelin's `IERC20` and `IERC20Metadata`

When deploying, point to the `native-token-wrapper-solidity` contract (or any other contract you expect implements our `IERC20`)

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