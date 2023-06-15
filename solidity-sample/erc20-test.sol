import "@openzeppelin/interfaces/IERC20.sol";

contract Erc20Test {
    address public token;
    IERC20 public tokenContract;

    constructor(address _token) {
        token = _token;
        tokenContract = IERC20(_token);
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
        return abi.decode(removeFirstThreeBytes(result), (uint256));

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

    function removeFirstThreeBytes(bytes memory data) public pure returns (bytes memory) {
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

    function decodeHardcodedValue2() external pure returns (string memory) {
        // This is the result of calling name() on the token contract
        // (the first 2 or 4 bytes are the function selector probably)
        bytes memory result = hex"280020546573744e616d65";
        bytes memory trimmedResult = removeFirstTwoBytes(result);
        return abi.decode(trimmedResult, (string));
    }

    function decodeHardcodedValue3() external pure returns (string memory) {
        // This is the result of calling name() on the token contract
        // (the first 2 or 4 bytes are the function selector probably)
        bytes memory result = hex"280020546573744e616d65";
        bytes memory trimmedResult = removeFirstThreeBytes(result);
        return abi.decode(trimmedResult, (string));
    }

    function decodeHardcodedValue4() external pure returns (string memory) {
        // This is the result of calling name() on the token contract
        // (the first 2 or 4 bytes are the function selector probably)
        bytes memory result = hex"280020546573744e616d65";
        bytes memory trimmedResult = removeFirstFourBytes(result);
        return abi.decode(trimmedResult, (string));
    }

    function decodeHardcodedU256() external pure returns (uint256) {
        // This is the result of calling totalSupply() on the token contract
        bytes memory result = hex"84006400000000000000000000000000000000000000000000000000000000000000";
        return abi.decode(result, (uint256));
    }

    function decodeHardcodedU256_2() external pure returns (uint256) {
        // This is the result of calling totalSupply() on the token contract
        bytes memory result = hex"84006400000000000000000000000000000000000000000000000000000000000000";
        bytes memory trimmedResult = removeFirstTwoBytes(result);
        return abi.decode(trimmedResult, (uint256));
    }

    function decodeHardcodedU256_3() external pure returns (uint256) {
        // This is the result of calling totalSupply() on the token contract
        bytes memory result = hex"84006400000000000000000000000000000000000000000000000000000000000000";
        bytes memory trimmedResult = removeFirstThreeBytes(result);
        return abi.decode(trimmedResult, (uint256));
    }

    function decodeHardcodedU256_4() external pure returns (uint256) {
        // This is the result of calling totalSupply() on the token contract
        bytes memory result = hex"84006400000000000000000000000000000000000000000000000000000000000000";
        bytes memory trimmedResult = removeFirstFourBytes(result);
        return abi.decode(trimmedResult, (uint256));
    }

    function decodeHardcodedU256_extracted() external pure returns (uint256) {
        // This is the result of calling totalSupply() on the token contract
        // (the first two or three bytes are some kind of prefix probably)
        bytes memory result = hex"84006400000000000000000000000000000000000000000000000000000000000000";
        (bytes2 prefix, uint256 decoded_result) = abi.decode(result, (bytes2, uint256));
        return decoded_result;
    }

    function nameWithHex() external returns (bytes memory) {
        (bool success, bytes memory result) = token.call(abi.encodeWithSelector(hex"06fdde03"));
        require(success, "call failed");
        bytes memory trimmedResult = removeFirstThreeBytes(result);
        return trimmedResult;
    }

    function name() external view returns (bytes memory) {
        return tokenContract.name();
    }

    function symbol() external view returns (string memory) {
        return tokenContract.symbol();
    }

    function decimals() external view returns (uint8) {
        return tokenContract.decimals();
    }
}