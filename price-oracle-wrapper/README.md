# price-oracle-wrapper

This contract provides the `function getAssetPrice(address asset) external returns (uint256 price)` function that is expected by Nabla's `IPriceOracleGetter` interface.

It calls the `1200` chain extension which fetches a price feed from the chain. The inputs `blockchain` and `symbol` are the keys to query a particular price feed. 

For the purposes of the `getAssetPrice` function, an instance of this contract refers to one particular price feed based on the inputs provided in the constructor. However, the contract also exposes the underlying functions which can query any asset, which is useful for testing and experimentation purposes.

## Compiling the solidity contract

### Install solang

```bash
brew install hyperledger/hyperledger/solang
```

### Compile the contract

```bash
solang compile --target substrate --output solang/contracts price-oracle-wrapper.sol
```

```bash
# Or from root directory
solang compile --target substrate --output target/solang/contracts price-oracle-wrapper/price-oracle-wrapper.sol 
```

### Deploying the contract

As a result you will get a file called `PriceOracleWrapper.contract` in the `solang/contracts/` directory.
You can now deploy this contract to a parachain using Contracts UI.


## Setup 

When testing locally, you'll likely have no oracle data on your chain. In that case, you'll have to first call the extrinsic: `diaOracleModule::setUpdatedCoinInfos` to add some price data.

## Using

Call the contract's `get_coin_info` function with the same keys corresponding to the oracle data you added to the chain.

Note that the price feed has to exist on the chain. They are added/updated via the `diaOracleModule::setUpdatedCoinInfos` extrinsic