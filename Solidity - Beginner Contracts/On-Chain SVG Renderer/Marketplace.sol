// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

// The marketplace was inspired by m1guelpf's lil-opensea, also
// available with the APGL-3.0-only license.
// See https://github.com/m1guelpf/lil-web3/blob/main/src/LilOpenSea.sol

// Import marketplace interface.
import "./Interfaces/IMarketplace.sol";

// Import Character interface.
import "./Interfaces/IERC721.sol";

// Import WETH interface.
import "./Interfaces/IWETH.sol";

/// @title CharacterDAOMarketplace.
/// @notice The feeless marketplace to trade Characters.
contract CharacterDAOMarketplace is ICharacterDAOMarketplace {
    /// ======================================= ///
    /// ======== Variables and Mapping ======== ///
    /// ======================================= ///

    /// @notice Counter for listing ID's.
    uint256 private listingID = 1;

    /// @notice The Character ERC721.
    IERC721 private Character;

    /// @notice Listing ID's mapped to Listing struct.
    mapping(uint256 => Listing) private listings;

    /// ============================= ///
    /// ======== Constructor ======== ///
    /// ============================= ///

    /// @notice Set the Character token.
    constructor(address _CharacterAddress) {
        Character = IERC721(_CharacterAddress);
    }

    /// ========================================== ///
    /// ======== State-Changing Functions ======== ///
    /// ========================================== ///

    /// @notice Allows the sender to list their tokens in the marketplace.
    /// @param _tokenID The token ID to list.
    /// @param _price The sell price.
    /// @return The new listing ID.
    function list(uint256 _tokenID, uint256 _price) external returns (uint256) {
        Character.transferFrom(msg.sender, address(this), _tokenID);

        listings[listingID] = Listing({
            owner: msg.sender,
            tokenID: _tokenID,
            price: _price
        });

        emit NewListing(_tokenID, _price);

        return listingID++;
    }

    /// @notice Allows the sender to change a listing's price.
    /// @param _listingID The ID of listing that is being modified.
    /// @param _newPrice The new ask price for the listing.
    function changePrice(uint256 _listingID, uint256 _newPrice) external {
        Listing memory listing = listings[_listingID];

        require(
            msg.sender == listing.owner,
            "CharacterDAO::Marketplace: Not permitted to modify listing."
        );

        listings[_listingID].price = _newPrice;

        emit PriceChanged(_listingID, _newPrice);
    }

    /// @notice Allows the sender to delete a listing.
    /// @param _listingID The listing ID to be cancelled.
    function removeListing(uint256 _listingID) external {
        Listing memory listing = listings[_listingID];

        require(
            msg.sender == listing.owner,
            "CharacterDAO::Marketplace: Not permitted to remove listing."
        );

        Character.transferFrom(address(this), msg.sender, listing.tokenID);

        delete listings[_listingID];

        emit ListingRemoved(_listingID);
    }

    /// @notice Allows the sender to buy a listing.
    /// @param _listingID The listing ID to be bought.
    function buyListing(uint256 _listingID) external payable {
        Listing memory listing = listings[_listingID];

        require(
            msg.value == listing.price,
            "CharacterDAO::Marketplace: Incorrect amount of funds."
        );
        require(
            listing.owner != address(0),
            "CharacterDAO::Marketplace: Invalid listing."
        );

        _transferETH(listing.owner, msg.value);

        delete listings[_listingID];

        emit ListingBought(_listingID);
    }

    /// ==================================== ///
    /// ======== Internal Functions ======== ///
    /// ==================================== ///

    /// @notice Transfers ETH to another address.
    /// @param _to The address ETH should be sent to.
    /// @param _amount The amount of ETH that should be sent.
    function _transferETH(address _to, uint256 _amount) internal {
        bool success;

        assembly {
            // gas(): gas Returns the amount of available gas.
            // call(gas, to, value, argsOffset, argsSize, returnOffset, returnSize):
            success := call(gas(), _to, _amount, 0, 0, 0, 0)
        }

        require(success, "CharacterDAO::Marketplace: Ether transfer reverted.");
    }
}
