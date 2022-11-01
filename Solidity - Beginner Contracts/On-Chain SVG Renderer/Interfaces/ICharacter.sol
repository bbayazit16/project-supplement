// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

/// @title ICharacter.
/// @notice ERC721 token interface for CharacterDAO.
interface ICharacter {

    /// ========================== ///
    /// ======== Structs ======== ///
    /// ========================= ///

    /// @dev Trait properties of each token.
    struct Traits {
        string seed;
        uint256[] votedProposals;
    }

    /// ======================== ///
    /// ======== Events ======== ///
    /// ======================== ///

    /// @notice Indicates that the DAO address was changed.
    /// @param oldAddress The old address of the DAO.
    /// @param newAddress The new address of the DAO.
    event DaoMigrated(address oldAddress, address newAddress);

    /// @notice Indicates that the DAO address was changed.
    /// @param oldAddress The old address of the Auction House.
    /// @param newAddress The new address of the Auction House.
    event AuctionHouseMigrated(address oldAddress, address newAddress);

    /// ========================================== ///
    /// ======== State-Changing Functions ======== ///
    /// ========================================== ///

    /// @notice Change the DAO address. May only be called by the DAO.
    /// @param _DAO The new DAO address.
    function migrateDAO(address _DAO) external;

    /// @notice Change the Auction House address. May only be called by the DAO.
    /// @param _auctionHouse The new Auction House address.
    function migrateAuctionHouse(address _auctionHouse) external;

    /// @notice Allows the auction house to mint a new Character.
    /// @param _seed The seed of the Character to mint.
    function mint(string calldata _seed) external;

    /// @notice Changes the proposals a Character has voted on. May only be called by the governor.
    /// @param _tokenID The Character that voted on the proposal.
    /// @param _proposalID The proposal id of the proposal Character voted on.
    function votedOnProposal(uint256 _tokenID, uint256 _proposalID) external;

    /// @notice Returns true if _addr owns _tokenID.
    /// @param _tokenID The Character ID.
    /// @param _addr The address.
    /// This function is an alternative to access ownerOf
    /// mapping. Otherwise it is unnecessary.
    function owns(uint256 _tokenID, address _addr) external view returns (bool);

    /// @notice Returns total supply.
    function totalSupply() external view returns (uint256);
}
