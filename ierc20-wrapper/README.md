# Native Token Wrapper Solidity

This contract implements openzeppelin's `IERC20` and `IERC20Metadata` intefaces. Then the functions call into our `native-token-wrapper-ink` contract which in turn call's the chain extensions on the blockchain.

This purpose of this contract is to allow our wrapped tokens to satisfy `IERC20` so that other Solidity contracts can use them the same way as any other `IERC20` token.

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