// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

/// @title ICharacterDAOMarketplace.
/// @notice The interface for Character marketplace.
interface ICharacterDAOMarketplace {
    /// ========================= ///
    /// ======== Structs ======== ///
    /// ========================= ///

    /// @notice Stores listing details.
    struct Listing {
        // The owner of the token.
        address owner;
        // Character token ID.
        uint256 tokenID;
        // Ask price for the Character.
        uint256 price;
    }

    /// ======================== ///
    /// ======== Events ======== ///
    /// ======================== ///

    /// @notice Emitted when a new listing is created.
    /// @param tokenID The ID of the Character that was listed.
    /// @param price The price Character was listed for.
    event NewListing(uint256 indexed tokenID, uint256 price);

    /// @notice Emitted when a listing price is modified.
    /// @param listingID The ID of listing that was modified.
    /// @param newPrice The new ask price for the listing.
    event PriceChanged(uint256 indexed listingID, uint256 newPrice);

    /// @notice Emitted when a listing is removed.
    /// @param listingID The ID of listing that was removed.
    event ListingRemoved(uint256 indexed listingID);

    /// @notice Emitted when a listing is bought.
    /// @param listingID The listing that was bought.
    event ListingBought(uint256 indexed listingID);

    /// ========================================== ///
    /// ======== State-Changing Functions ======== ///
    /// ========================================== ///

    /// @notice Allows the sender to list their tokens in the marketplace.
    /// @param _tokenID The token ID to list.
    /// @param _price The sell price.
    /// @return The new listing ID.
    function list(uint256 _tokenID, uint256 _price) external returns (uint256);

    /// @notice Allows the sender to change a listing's price.
    /// @param _listingID The ID of listing that is being modified.
    /// @param _newPrice The new ask price for the listing.
    function changePrice(uint256 _listingID, uint256 _newPrice) external;

    /// @notice Allows the sender to delete a listing.
    /// @param _listingID The listing ID to be cancelled.
    function removeListing(uint256 _listingID) external;

    /// @notice Allows the sender to buy a listing.
    /// @param _listingID The listing ID to be bought.
    function buyListing(uint256 _listingID) external payable;
}
