contract NativeTokenWrapperSolidity {
    address public token;

    constructor(address _token) {
        token = _token;
    }

    function name() public returns (string memory) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"06fdde03"));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, string);
    }

    function symbol() public returns (string memory) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"95d89b41"));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, string);
    }

    function decimals() public returns (uint8) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"313ce567"));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, uint8);
    }

    function totalSupply() public returns (uint256) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"18160ddd"));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, uint256);
    }

    function balanceOf(address _owner) public returns (uint256 balance) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"70a08231", _owner));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, uint256);
    }

    function transfer(address _to, uint256 _value) public returns (bool success) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"a9059cbb", _to, _value));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, bool);
    }

    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"23b872dd", _from, _to, _value));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, bool);
    }

    function approve(address _spender, uint256 _value) public returns (bool success) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"095ea7b3", _spender, _value));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, bool);
    }

    function allowance(address _owner, address _spender) public returns (uint256 remaining) {
        (bool ok, bytes raw_data) = token.call(abi.encodeWithSelector(hex"dd62ed3e", _owner, _spender));
        require(ok);
        (uint8 result, bytes b) = abi.decode(raw_data, (uint8, bytes));
        require(result == 0);
        return abi.decode(b, uint256);
    }
}