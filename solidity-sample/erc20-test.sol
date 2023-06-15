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

    function balanceOfSignature(address _account) external returns (uint256) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSignature("balanceOf(address)", _account));
        require(success, "call failed");
        return abi.decode(result, (uint256));
    }

    function balanceOfHex(address _account) external returns (uint256) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSelector(hex"70a08231", _account));
        require(success, "call failed");
        return abi.decode(result, (uint256));
    }

    function totalSupply() external view returns (uint256) {
        return tokenContract.totalSupply();
    }

    function totalSupplyWithSignature() external returns (uint256) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSignature("totalSupply()"));
        require(success, "call failed");
        return abi.decode(result, (uint256));
    }

    function totalSupplyWithHex() external returns (uint) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSelector(hex"18160ddd"));
        require(success, "call failed");
        return abi.decode(result, (uint));
    }

    function name() external view returns (string memory) {
        return tokenContract.name();
    }

    function nameWithHex() external returns (bytes memory) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSelector(hex"06fdde03"));
        require(success, "call failed");
        return result;
    }

    function symbol() external view returns (string memory) {
        return tokenContract.symbol();
    }

    function decimals() external view returns (uint8) {
        return tokenContract.decimals();
    }
}