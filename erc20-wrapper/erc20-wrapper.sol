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

        print("currency: {}".format(currency));
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1107, input);
        print("result_chain_ext: {}".format(result_chain_ext));
        print("raw_data: {}".format(raw_data));
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        uint128 totalSupplyU128 = abi.decode(raw_data, (uint128));
        print("totalSupply: {}".format(totalSupplyU128));

        uint256 totalSupplyU256 = uint256(totalSupplyU128);
        return totalSupplyU256;
    }

    function balanceOf(address _owner) public returns (uint256) {
        // Encode currency and address
        bytes currency = createCurrencyId();
        bytes owner = abi.encode(_owner);
        // Concatenate the already encoded values with abi.encodePacked()
        bytes input = abi.encodePacked(currency, owner);
        print("input: {}".format(input));

        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1106, input);
        print("result_chain_ext: {}".format(result_chain_ext));
        print("raw_data: {}".format(raw_data));
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        uint128 balanceU128 = abi.decode(raw_data, (uint128));
        print("totalSupply: {}".format(balanceU128));

        uint256 balanceU256 = uint256(balanceU128);
        return balanceU256;
    }

    function transfer(address _to, uint256 _amount) public returns (bool) {
//        bytes from = abi.encode(msg.sender);
        bytes origin = abi.encode(_originType);
        bytes currency = createCurrencyId();
        bytes to = abi.encode(_to);
        bytes amount = abi.encode(_amount);

        bytes input = abi.encodePacked(origin, currency, to, amount);
        print("input: {}".format(input));
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(1105, input);
        print("result_chain_ext: {}".format(result_chain_ext));
        print("raw_data: {}".format(raw_data));

        bool success = result_chain_ext == 0;
//        require(result_chain_ext == 0, "Call to chain_extension failed.");

//        bool success = abi.decode(raw_data, (bool));
        print("success: {}".format(success));
        return success;
    }

    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        return false;
    }

    function approve(address _spender, uint256 _value) public returns (bool success) {
        return false;
    }

    function allowance(address _owner, address _spender) public returns (uint256 remaining) {
        return 0;
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
}