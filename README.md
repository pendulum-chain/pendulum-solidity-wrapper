PSP Pendulum smart contract designed to provide access to any currency on top of substrate pendulum chain via chain extension. 

Here is the conversion from primitive type to currency id: https://github.com/pendulum-chain/pendulum/blob/main/runtime/common/src/chain_ext.rs#L124-L143

Access to new currency id requires deployment of new psp pendulum smart contract instance and initializing the contract with correct type_id, code and issuer to support mapping on the chain side.



cargo install cargo-contract --version 1.5.0

Contracts

If you have cargo-contract (https://github.com/paritytech/cargo-contract) installed, you can easily build with:

`cargo contract build`

For more info on writing and building ink smart contracts, see: https://github.com/paritytech/ink

To deploy and use: https://contracts-ui.substrate.io/?rpc=ws://127.0.0.1:9944

