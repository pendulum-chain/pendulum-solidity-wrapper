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

```bash
# Or from root directory
solang compile --target substrate --output target/solang/contracts erc20-wrapper/erc20-wrapper.sol 
```

### Deploying the contract

As a result you will get a file called `ERC20Wrapper.contract` in the `solang/contracts/` directory.
You can now deploy this contract to a parachain using Contracts UI.

#### Constructor arguments
When you deploy the contract, you need to specify the `CurrencyId` of the token that you want to wrap in the
constructor.
The logic is as follows:
- The `variant` determines which element of the top-level enum is used.
- The `index` determines which element of the variant is used.
- In case of `XCM`, the `index` is the index of the `XCM` variant in the top-level enum.
- In case of `Stellar`, the `index` is the index of the `Stellar` variant in the `Asset` enum.
- The `code` and `issuer` are the `code` and `issuer` of the `Asset` enum. They are ignored if the `variant` is
not `Stellar`.

```rust
pub enum CurrencyId {
    Native = 0,
    // _variant
    XCM(_index) = 1,
    // _variant
    Stellar(Asset) = 2, // _variant
}

pub enum Asset {
    StellarNative = 0,
    // _index
    AlphaNum4 { code: Bytes4, issuer: AssetIssuer } = 1,
    // _index
    AlphaNum12 { code: Bytes12, issuer: AssetIssuer } = 2, // _index
}
```

Some examples:
- `CurrencyId::Native` would be `variant: 0, index: 0`
- `CurrencyId::XCM(0)` would be `variant: 1, index: 0`
- `CurrencyId::Stellar(Asset::StellarNative)` would be `variant: 2, index: 0`
- `CurrencyId::Stellar(Asset::AlphaNum4)` would be `variant: 2, index: 1, code: "0xSOMEHEX", issuer: "0xSOMEHEX"`
- `CurrencyId::Stellar(Asset::AlphaNum12)` would be `variant: 2, index: 2, code: "0xSOMEHEX", issuer: "0xSOMEHEX"`
