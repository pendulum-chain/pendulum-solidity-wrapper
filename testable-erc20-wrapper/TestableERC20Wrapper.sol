// This file does not define a pragma version because it is meant to be compiled with Solang and Solang ignores
// pragma definitions, see [here](https://solang.readthedocs.io/en/latest/language/pragmas.html).

import "polkadot";
import {ERC20Wrapper} from "../erc20-wrapper/ERC20Wrapper.sol";

contract TestableERC20Wrapper is ERC20Wrapper {
    /**
     * @dev Emitted when `value` tokens are minted to one account
     *
     */
    event Mint(address indexed to, uint256 value);

    /**
     * @dev Emitted when `value` tokens are burned from one account
     *
     */
    event Burn(address indexed from, uint256 value);

    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        bytes1 variant_,
        bytes1 index_,
        bytes12 code_,
        bytes32 issuer_
    ) {
        ERC20Wrapper(name_, symbol_, decimals_, variant_, index_, code_, issuer_);
    }

    function mint(address _to, uint256 _amount) public returns (bool) {
        bytes currency = createCurrencyId();
        bytes to = abi.encode(_to);

        uint128 amountU128 = convertU256toU128(_amount);
        bytes amount = abi.encode(amountU128);

        bytes input = abi.encodePacked(currency, to, amount);
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(
            1107,
            input
        );
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        // If the call to chain_extension was successful, the raw_data will contain only `0`s
        bool success = isBytesAllZeros(raw_data);

        emit Mint(_to, _amount);
        return success;
    }

    function burn(address _from, uint256 _amount) public returns (bool) {
        bytes currency = createCurrencyId();
        bytes from = abi.encode(_from);

        uint128 amountU128 = convertU256toU128(_amount);
        bytes amount = abi.encode(amountU128);

        bytes input = abi.encodePacked(currency, from, amount);
        (uint32 result_chain_ext, bytes raw_data) = chain_extension(
            1108,
            input
        );
        require(result_chain_ext == 0, "Call to chain_extension failed.");

        // If the call to chain_extension was successful, the raw_data will contain only `0`s
        bool success = isBytesAllZeros(raw_data);

        emit Burn(_from, _amount);
        return success;
    }
}
