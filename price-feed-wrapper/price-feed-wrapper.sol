import "substrate";
import {IPriceOracleGetter} from "./interfaces/IPriceOracleGetter.sol";

/**
 * @title PriceFeedWrapper
 * @notice Price oracle that uses the native chain via chain extension. It stores _asset, _blockchain and _symbol for use by function getAssetPrice() which is called by Nabla. 
 * @dev This contract can be used with the Nabla's Router contract
 */
contract PriceFeedWrapper is IPriceOracleGetter {
    address public _asset;
    string public _blockchain;
    string public _symbol;

    // we store _asset, _blockchain and _symbol for use by function getAssetPrice() which is called by Nabla. 
    // _blockchain and _symbol are the keys used to access a particular price feed from the chain.
    constructor(address asset, string blockchain, string symbol) {
        _asset = asset;
        _blockchain = blockchain;
        _symbol = symbol;
    }

    /**
     * @notice Returns the asset price in USD. This is called by Nabla and expected by their IPriceOracleGetter interface
     * @param asset asset address
     * @return price Asset price in USD
     */
    function getAssetPrice(address asset) external returns (uint256 price) {
        require(_asset == asset, "Asset does not match");
        price = uint256(getAnyAssetPrice(_blockchain, _symbol));
    }

    function getAnyAssetSupply(string blockchain, string symbol) public returns (uint128 result) {
        result = getAnyAsset(blockchain, symbol).supply;
    }
    function getAnyAssetLastUpdateTimestamp(string blockchain, string symbol) public returns (uint64 result) {
        result = getAnyAsset(blockchain, symbol).last_update_timestamp;
    }
    function getAnyAssetPrice(string blockchain, string symbol) public returns (uint128 result) {
        result = getAnyAsset(blockchain, symbol).price;
    }

    /**
     * @notice performs the actual chain extension call to get the price feed. blockchain and symbol are the keys used to access a particular price feed from the chain.
     * @param blockchain input string
     * @param symbol input string
     * @return result the struct cointaining all the coin info
     */
    function getAnyAsset(string blockchain, string symbol) public returns (CoinInfo result) {
        bytes input = abi.encodePacked(stringToBytes32(blockchain), stringToBytes32(symbol));
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1200, input);
        require(result_chain_ext == 0, "Call to chain_extension failed.");
        require(raw_data[0] == 0, "Chain extension call returned an error.");
        (uint8 _, CoinInfo coinInfo) = abi.decode(raw_data, (uint8, CoinInfo));
        require(raw_data[0] == 0, "Chain extension call returned an error.");
        result = coinInfo;
    }
    
    /**
     * @notice converts a string to a bytes of length 32. Will truncate the string or pad with null values to fit the bytes of length 32. This output is the expected format for blockchain and symbol.
     * @param str input string
     * @return result32 a bytes of length 32
     */
    function stringToBytes32(string str) private pure returns (bytes result32){
        bytes input = bytes(str);
        bytes output = new bytes(32);
        for (uint i = 0; i < output.length && i < input.length; i++) {
            output[i] = bytes(input)[i];
        }
        result32 = output;
    }

    // The coin info returned by the chain extension call
    struct CoinInfo {
        bytes symbol;
        bytes name;
        bytes blockchain;
        uint128 supply;
        uint64 last_update_timestamp;
        uint128 price;
    }
}
