// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

// Import Solmate ERC721.
import "../lib/solmate/ERC721.sol";

// Import renderer interface.
import "./Interfaces/IRenderer.sol";

// Import Character Interface
import "./Interfaces/ICharacter.sol";

/// @title Character.
/// @notice The ERC721 token for CharacterDAO.
contract Character is ICharacter, ERC721("CharacterDAO", "CRDD") {
    /// =========================== ///
    /// ======== Variables ======== ///
    /// =========================== ///

    /// @dev The latest token id, incremented with every mint.
    uint256 private tokenID;

    /// @dev The address for the auction house which has the permission to mint.
    address private auctionHouse;

    /// @dev The address for the DAO governor.
    address private DAO;

    /// @dev The renderer for token metadata.
    IRenderer private renderer;

    /// ========================== ///
    /// ======== Mappings ======== ///
    /// ========================== ///

    /// @dev The mapping from token ID to Traits.
    mapping(uint256 => Traits) private properties;

    /// =========================== ///
    /// ======== Modifiers ======== ///
    /// =========================== ///

    /// @dev May only be called by auction house.
    modifier onlyAuctionHouse() {
        require(
            msg.sender == auctionHouse,
            "CharacterDAO::Character: Function may only be called by the Auction House."
        );
        _;
    }

    /// @dev May only be called by the DAO.
    modifier onlyDAO() {
        require(
            msg.sender == DAO,
            "CharacterDAO::Character: Function may only be called by the DAO."
        );
        _;
    }

    /// ============================= ///
    /// ======== Constructor ======== ///
    /// ============================= ///

    /// @param _auctionHouse The address for the auction house.
    /// @param _DAO The address for the DAO wallet.
    /// @param _renderer The address for renderer.
    constructor(
        address _auctionHouse,
        address _DAO,
        address _renderer
    ) {
        // Set auction house.
        auctionHouse = _auctionHouse;

        // Set DAO address.
        DAO = _DAO;

        // Set renderer.
        renderer = IRenderer(_renderer);
    }

    /// ========================================== ///
    /// ======== State-Changing Functions ======== ///
    /// ========================================== ///

    /// @notice Change the DAO address. May only be called by the DAO.
    /// @param _DAO The new DAO address.
    function migrateDAO(address _DAO) external onlyDAO {
        emit DaoMigrated(DAO, _DAO);
        DAO = _DAO;
    }

    /// @notice Change the Auction House address. May only be called by the DAO.
    /// @param _auctionHouse The new Auction House address.
    function migrateAuctionHouse(address _auctionHouse) external onlyDAO {
        emit AuctionHouseMigrated(auctionHouse, _auctionHouse);
        auctionHouse = _auctionHouse;
    }

    /// @notice Allows the auction house to mint a new Character.
    /// @param _seed The seed of the Character to mint.
    function mint(string calldata _seed) external onlyAuctionHouse {
        
        uint256[] memory emptyArr;

        // Set properties of the token.
        properties[tokenID] = Traits({
            seed: _seed,
            votedProposals: emptyArr
        });

        _mint(auctionHouse, tokenID);

        tokenID++;
    }

    /// @notice Changes the proposals a Character has voted on. May only be called by the governor.
    /// @param _tokenID The Character that voted on the proposal.
    /// @param _proposalID The proposal id of the proposal Character voted on.
    function votedOnProposal(
        uint256 _tokenID,
        uint256 _proposalID
    ) external onlyDAO {
        properties[_tokenID].votedProposals.push(_proposalID);
    }

    /// ================================ ///
    /// ======== View-Functions ======== ///
    /// ================================ ///

    /// @notice Returns token data.
    /// @param _tokenID Input token ID.
    /// @return Token metadata.
    function tokenURI(uint256 _tokenID)
        public
        view
        override
        returns (string memory)
    {
        require(_tokenID == 3);
        return
            renderer.render(["de473c", "f5877f", "bd837e"], "3b92a3", "8dc0c9");
    }

    /// @notice Returns true if _addr owns _tokenID.
    /// @param _tokenID The Character ID.
    /// @param _addr The address.
    /// This function is an alternative to access ownerOf
    /// mapping. Otherwise it is unnecessary.
    function owns(uint256 _tokenID, address _addr) external view returns (bool) {
        return ownerOf[_tokenID] == _addr;
    }

    /// @notice Returns total supply.
    function totalSupply() external view returns (uint256) {
        return tokenID;
    }
}
