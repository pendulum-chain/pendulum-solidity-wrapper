import "@openzeppelin/interfaces/IERC20.sol";

contract ERC20User {
    address public token;
    IERC20 public tokenContract;

    constructor(address _token) {
        token = _token;
        tokenContract = IERC20(_token);
    }

    function balanceOf(address _account) external view returns (uint256) {
        return tokenContract.balanceOf(_account);
    }

    function totalSupply() external view returns (uint256) {
        return tokenContract.totalSupply();
    }

    function name() external view returns (string memory) {
        return tokenContract.name();
    }

    function symbol() external view returns (string memory) {
        return tokenContract.symbol();
    }

    function decimals() external view returns (uint8) {
        return tokenContract.decimals();
    }

    function transfer(address _to, uint256 _value) external returns (bool) {
        return tokenContract.transfer(_to, _value);
    }

    function transferFrom(address _from, address _to, uint256 _value) external returns (bool) {
        return tokenContract.transferFrom(_from, _to, _value);
    }

    function approve(address _spender, uint256 _value) external returns (bool) {
        return tokenContract.approve(_spender, _value);
    }

    function allowance(address _owner, address _spender) external view returns (uint256) {
        return tokenContract.allowance(_owner, _spender);
    }
}