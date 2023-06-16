import "@openzeppelin/interfaces/IERC20.sol";

contract Erc20Test {
    IERC20 public token;

    constructor(address _token) {
        token = IERC20(_token);
    }

    function name() external view returns (string memory) {
        return token.name();
    }
    function symbol() external view returns (string memory) {
        return token.symbol();
    }
    function decimals() external view returns (uint8) {
        return token.decimals();
    }
    function totalSupply() external view returns (uint256) {
        return token.totalSupply();
    }
    function balanceOf(address _owner) public view returns (uint256 balance) {
        return token.balanceOf(_owner);
    }
    function transfer(address _to, uint256 _value) public returns (bool success) {
        return token.transfer(_to, _value);
    }
    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        return token.transferFrom(_from, _to, _value);
    }
    function approve(address _spender, uint256 _value) public returns (bool success) {
        return token.approve(_spender, _value);
    }
    function allowance(address _owner, address _spender) public view returns (uint256 remaining) {
        return token.allowance(_owner, _spender);
    }
}