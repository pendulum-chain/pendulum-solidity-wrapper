psp_pendulum

PSP Pendulum smart contract designed to provide access to any currency on top of substrate pendulum chain via chain extension. 

Here is the conversion from primitive type to currency id: https://github.com/pendulum-chain/pendulum/blob/main/runtime/common/src/chain_ext.rs#L124-L143

Access to new currency id requires deployment of new psp pendulum smart contract instance and initializing the contract with correct type_id, code and issuer to support mapping on the chain side.

ERC20 standard from https://ethereum.org/en/developers/docs/standards/tokens/erc-20/


Methods:

function name() public view returns (string)

function symbol() public view returns (string)

function decimals() public view returns (uint8)

function totalSupply() public view returns (uint256)

function balanceOf(address _owner) public view returns (uint256 balance)

function transfer(address _to, uint256 _value) public returns (bool success)

function transferFrom(address _from, address _to, uint256 _value) public returns (bool success)

function approve(address _spender, uint256 _value) public returns (bool success)

function allowance(address _owner, address _spender) public view returns (uint256 remaining)


Events:

event Transfer(address indexed _from, address indexed _to, uint256 _value)

event Approval(address indexed _owner, address indexed _spender, uint256 _value)


Selectors are obtained like this:

cargo install keccak256-cli

echo "name()" | keccak256 --method-id

echo "symbol()" | keccak256 --method-id

echo "decimals()" | keccak256 --method-id

echo "totalSupply()" | keccak256 --method-id

echo "balanceOf(address)" | keccak256 --method-id

echo "transfer(address,uint256)" | keccak256 --method-id

echo "transferFrom(address,address,uint256)" | keccak256 --method-id

echo "approve(address,uint256)" | keccak256 --method-id

echo "allowance(address,address)" | keccak256 --method-id
