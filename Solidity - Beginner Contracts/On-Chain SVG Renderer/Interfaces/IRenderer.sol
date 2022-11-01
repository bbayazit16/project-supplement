// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

/// @title IRenderer
/// @notice The interface of SVG renderer for Character metadata.

interface IRenderer {
    function render(
        string[3] memory _bodyColors,
        string memory _eyeColor,
        string memory _mouthColor
    ) external view returns (string memory);
}
