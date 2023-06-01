# price_feed

a contract that wraps the dia chain extension which provides a price feed for various assets.

## Setup 

When testing locally, you'll likely have no oracle data on your chain. In that case, you'll have to first call the extrinsic: `diaOracleModule::setUpdatedCoinInfos` to add some price data.

## Using

Call the contract's `get_coin_info` function with the same keys corresponding to the oracle data you added to the chain.