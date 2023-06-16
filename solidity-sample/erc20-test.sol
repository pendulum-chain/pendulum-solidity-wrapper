import "@openzeppelin/interfaces/IERC20.sol";

contract Erc20Test {
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
}