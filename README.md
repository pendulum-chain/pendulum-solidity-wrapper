# Pendulum-Solidity Wrappers

A collection of contracts written in Solidity and compiled to WASM using Solang compiler for deployment on the
Contracts pallet.

These contracts implement well know interfaces (`IERC20`, `IPriceOracleGetter`), but the novel feature
is the ability to call Pendulum's chain extensions which allows them to interact with 
the logic of the pallets. Notably, fetching latest price of assets from the chain and interactions with the tokens pallet.

Please see 'docs' for a more information.

For information about compiling the contracts, see the `README` file on each contract directory.




