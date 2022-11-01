// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

/// @title IERC721.
/// @notice Solmate ERC721 token interface.
interface IERC721 {
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 indexed id
    );

    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 indexed id
    );

    event ApprovalForAll(
        address indexed owner,
        address indexed operator,
        bool approved
    );

    function approve(address spender, uint256 id) external;

    function setApprovalForAll(address operator, bool approved) external;

    function transferFrom(
        address from,
        address to,
        uint256 id
    ) external;

    function safeTransferFrom(
        address from,
        address to,
        uint256 id
    ) external;

    function safeTransferFrom(
        address from,
        address to,
        uint256 id,
        bytes memory data
    ) external;

    function supportsInterface(bytes4 interfaceId) external pure returns (bool);

    function tokenURI(uint256 _tokenID) external view returns (string memory);
}
