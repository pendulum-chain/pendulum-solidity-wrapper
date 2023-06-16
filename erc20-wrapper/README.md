# ERC 20 Wrapper - Solidity

This contract implements openzeppelin's `IERC20` and `IERC20Metadata` interfaces.
It is able to communicate with the chain extensions directly, without the need to have a second wrapper contract
implemented in ink!.

This purpose of this contract is to allow our wrapped tokens to satisfy `IERC20` so that other Solidity contracts can
use them the same way as any other `IERC20` token.

## Compiling the solidity contract

### Install solang

```bash
brew install hyperledger/hyperledger/solang
```

### Compile the contract

We need to specify the importmap for the openzeppelin contracts, because solang does not support the solidity import
syntax.

```bash
solang compile --target substrate --output solang/contracts erc20-wrapper.sol
```

As a result you will get a file called `ERC20Wrapper.contract` in the `solang/contracts/` directory.
You can now deploy this contract to a parachain using Contracts UI. 