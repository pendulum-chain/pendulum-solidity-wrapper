# psp_pendulum

PSP Pendulum is a contract designed to provide access to a Pendulum asset (CurrencyId) via chain extensions. This allows native tokens to be wrapped and used in the smart contract world. 

Note that for Solidity contracts to see this as an `IERC20` token just like any other `IERC20` token, we have a Solidity contract that wraps this one, `native-token-wrapper-solidity` providing the usable implementation of the interface.

## Setup

Before an asset can be used with the chain extensions, it needs to first be enabled via the extrinsic: `tokenAllowance:addAllowedCurrencies`

## ERC20 Standard

From the ERC20 standard at: https://ethereum.org/en/developers/docs/standards/tokens/erc-20/

Methods:

```
function name() public view returns (string)
function symbol() public view returns (string)
function decimals() public view returns (uint8)
function totalSupply() public view returns (uint256)
function balanceOf(address _owner) public view returns (uint256 balance)
function transfer(address _to, uint256 _value) public returns (bool success)
function transferFrom(address _from, address _to, uint256 _value) public returns (bool success)
function approve(address _spender, uint256 _value) public returns (bool success)
function allowance(address _owner, address _spender) public view returns (uint256 remaining)
```

Events:

```
event Transfer(address indexed _from, address indexed _to, uint256 _value)
event Approval(address indexed _owner, address indexed _spender, uint256 _value)
```

## Compatibility with Solidity ERC20 Standard

Selectors that map ink! functions to the appropriate ERC20 function are obtained like this:

Install the tool: `cargo install keccak256-cli`

```
echo "name()" | keccak256 --method-id
0x06fdde03
echo "symbol()" | keccak256 --method-id
0x95d89b41
echo "decimals()" | keccak256 --method-id
0x313ce567
echo "totalSupply()" | keccak256 --method-id
0x18160ddd
echo "balanceOf(address)" | keccak256 --method-id
0x70a08231
echo "transfer(address,uint256)" | keccak256 --method-id
0xa9059cbb
echo "transferFrom(address,address,uint256)" | keccak256 --method-id
0x23b872dd
echo "approve(address,uint256)" | keccak256 --method-id
0x095ea7b3
echo "allowance(address,address)" | keccak256 --method-id
0xdd62ed3e
```