import "substrate";

contract ERC20Wrapper {

    string private _name;
    string private _symbol;
    uint8 private _decimals;

    bytes1 private _variant;
    bytes1 private _index;

    // `0` indicates OriginType 'caller'
    // everything else indicates OriginType 'address'
    uint8 private _originType;

    constructor(string memory name_, string memory symbol_, uint8 decimals_, uint8 originType_, bytes1 variant_, bytes1 index_) {
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;

        _originType = originType_;
        _variant = variant_;
        _index = index_;
    }

    /**
     * @dev Returns the name of the token.
     */
    function name() public view virtual returns (string memory) {
        return _name;
    }

    /**
     * @dev Returns the symbol of the token, usually a shorter version of the
     * name.
     */
    function symbol() public view virtual returns (string memory) {
        return _symbol;
    }

    function decimals() public view virtual returns (uint8) {
        return _decimals;
    }

    function totalSupply() public returns (uint256) {
        bytes currency = createCurrencyId();
        bytes input = currency;

        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1107, input);
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        uint128 totalSupplyU128 = abi.decode(raw_data, (uint128));
        uint256 totalSupplyU256 = uint256(totalSupplyU128);
        return totalSupplyU256;
    }

    function balanceOf(address _owner) public returns (uint256) {
        // Encode currency and address
        bytes currency = createCurrencyId();
        bytes owner = abi.encode(_owner);
        // Concatenate the already encoded values with abi.encodePacked()
        bytes input = abi.encodePacked(currency, owner);

        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1106, input);
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        uint128 balanceU128 = abi.decode(raw_data, (uint128));
        uint256 balanceU256 = uint256(balanceU128);
        return balanceU256;
    }

    function transfer(address _to, uint256 _amount) public returns (bool) {
        bytes origin = abi.encode(_originType);
        bytes currency = createCurrencyId();
        bytes to = abi.encode(_to);

        uint128 amountU128 = convertU256toU128(_amount);
        bytes amount = abi.encode(amountU128);

        bytes input = abi.encodePacked(origin, currency, to, amount);
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1105, input);
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        // If the call to chain_extension was successful, the raw_data will contain only `0`s
        bool success = isBytesAllZeros(raw_data);
        return success;
    }

    function transferFrom(address _from, address _to, uint256 _amount) public returns (bool) {
        bytes from = abi.encode(_from);
        bytes origin = abi.encode(_originType);
        bytes currency = createCurrencyId();
        bytes to = abi.encode(_to);

        uint128 amountU128 = convertU256toU128(_amount);
        bytes amount = abi.encode(amountU128);

        bytes input = abi.encodePacked(from, origin, currency, to, amount);
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1109, input);
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        // If the call to chain_extension was successful, the raw_data will contain only `0`s
        bool success = isBytesAllZeros(raw_data);
        return success;
    }

    function approve(address _spender, uint256 _amount) public returns (bool) {
        bytes origin = abi.encode(_originType);
        bytes currency = createCurrencyId();
        bytes spender = abi.encode(_spender);

        uint128 amountU128 = convertU256toU128(_amount);
        bytes amount = abi.encode(amountU128);

        bytes input = abi.encodePacked(origin, currency, spender, amount);
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1108, input);

        bool success = result_chain_ext == 0;
        return success;
    }

    function allowance(address _owner, address _spender) public returns (uint256) {
        bytes currency = createCurrencyId();
        bytes owner = abi.encode(_owner);
        bytes spender = abi.encode(_spender);
        bytes input = abi.encodePacked(currency, owner, spender);

        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1110, input);
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        uint128 allowanceU128 = abi.decode(raw_data, (uint128));
        uint256 allowanceU256 = uint256(allowanceU128);
        return allowanceU256;
    }

    function createCurrencyId() public view returns (bytes) {
        bytes memory currency = new bytes(0);
        // We use the knowledge we have about our `CurrencyId` enum to craft the encoding
        if (_variant == 0) {
            // Native
            print("Native");
            currency = abi.encode(_variant);
        } else if (_variant == 1) {
            // XCM(_index)
            print("XCM({})".format(_index));
            currency = abi.encode(_variant, _index);
        } else {
            require(false, "Invalid variant");
            // Unknown
            currency = abi.encode(_name, _symbol);
        }
        return currency;
    }

    function isBytesAllZeros(bytes memory data) private pure returns (bool) {
        for (uint256 i = 0; i < data.length; i++) {
            if (data[i] != 0) {
                return false;
            }
        }
        return true;
    }

    // If we don't use this function to convert from uint256 to uint128,
    // then the chain extensions will just silently use u128.max() as the value instead of erroring
    function convertU256toU128(uint256 value) public pure returns (uint128) {
        require(value <= type(uint128).max, "Value exceeds maximum representable uint128");

        uint128 result = uint128(value);
        return result;
    }
}