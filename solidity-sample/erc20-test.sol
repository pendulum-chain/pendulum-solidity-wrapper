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

    function totalSupplyWithHexAndBytes() external returns (bytes memory) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSelector(hex"18160ddd"));
        require(success, "call failed");
        return result;
    }

    function totalSupplyWithHexAndBytesRemoved() external returns (uint256) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSelector(hex"18160ddd"));
        require(success, "call failed");
        // remove first 2 bytes of result before decoding to u256
        return abi.decode(removeFirstTwoBytes(result), (uint256));

        // Slicing doesn't work with Solang apparently
        //        return abi.decode(result[4 :], (uint256));
    }

    function removeFirstTwoBytes(bytes memory data) public pure returns (bytes memory) {
        uint256 newDataSize = data.length - 2;
        bytes memory newData = new bytes(newDataSize);
        for (uint256 i = 2; i < data.length; i++) {
            newData[i - 2] = data[i];
        }
        return newData;
    }

    function removeFirstFourBytes(bytes memory data) public pure returns (bytes memory) {
        uint256 newDataSize = data.length - 4;
        bytes memory newData = new bytes(newDataSize);
        for (uint256 i = 4; i < data.length; i++) {
            newData[i - 4] = data[i];
        }
        return newData;
    }


    function decodeHardcodedValue() external pure returns (string memory) {
        // This is the result of calling name() on the token contract (the first 2 or 4 bytes are the function selector probably)
        bytes memory result = hex"280020546573744e616d65";
        // TODO try to decode result directly without a cross-contract call

        return "";
    }

    function name() external view returns (string memory) {
        return tokenContract.name();
    }

    function nameWithHex() external returns (string memory) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSelector(hex"06fdde03"));
        require(success, "call failed");
        return abi.decode(result, (string));
    }

    function symbol() external view returns (string memory) {
        return tokenContract.symbol();
    }

    function decimals() external view returns (uint8) {
        return tokenContract.decimals();
    }
}