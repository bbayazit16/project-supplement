// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

// Import SSTORE2 library, used for writing SVG as a bytecode
// instead of directly writing to the storage, which reduces
// gas costs for storage.
import "../lib/solmate/SSTORE2.sol";

// Import Renderer interface.
import "./Interfaces/IRenderer.sol";

/// @title Renderer.
/// @notice The SVG renderer for Character metadata.
contract Renderer is IRenderer {
    /// ======================================== ///
    /// ======== Variables and Mappings ======== ///
    /// ======================================== ///

    bool private bodySet;

    /// @dev SVG string for body parts, set in the constructor.
    /// Points to the address to read from SSTORE2.
    mapping(string => address) private parts;

    /// ============================= ///
    /// ======== Constructor ======== ///
    /// ============================= ///

    /// @notice Constructor that sets the base SVG templates for the tokens.
    /// @dev Parts must be given without the <svg> open and close tags.
    /// @param _eyes SVG eyes parts of Characters.
    /// @param _mouth SVG mouth parts of Characters.
    constructor(
        bytes memory _eyes,
        bytes memory _mouth
    ) {
        parts["eyes"] = SSTORE2.write(_eyes);
        parts["mouth"] = SSTORE2.write(_mouth);
    }

    /// @param _body SVG body parts of Characters.
    function setBody(bytes memory _body) external {
        require(!bodySet, "CharacterDao::Renderer: Body has already been set.");
        bodySet = true;
        parts["body"] = SSTORE2.write(_body);
    }

    /// ==================================== ///
    /// ======== External Functions ======== ///
    /// ==================================== ///

    /// @dev Renders a metadata using given colors.
    /// @param _bodyColors Colors of the body.
    /// @param _eyeColor Eye color.
    /// @param _mouthColor Mouth color.
    function render(
        string[3] memory _bodyColors,
        string memory _eyeColor,
        string memory _mouthColor
    ) external view returns (string memory) {
        return
            string(
                abi.encodePacked(
                    '<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024" xml:space="preserve">',
                    _renderBody(_bodyColors),
                    _renderEyes(_eyeColor),
                    _renderMouth(_mouthColor),
                    "</svg>"
                )
            );
    }

    /// ==================================== ///
    /// ======== Internal Functions ======== ///
    /// ==================================== ///

    /// @dev Renders the body part only.
    /// @param _colors The colors of the body.
    /// @return Rendered SVG group with color.
    function _renderBody(string[3] memory _colors)
        internal
        view
        returns (string memory)
    {
        return
            string(
                abi.encodePacked(
                    "<g>",
                    SSTORE2.read(parts["body"]),
                    "<style>",
                    string(
                        abi.encodePacked(
                            ".body0{fill:#",
                            _colors[0],
                            "}.body1{fill:#",
                            _colors[1],
                            "}.body2{fill:#",
                            _colors[2],
                            "}"
                        )
                    ),
                    "</style>"
                    "</g>"
                )
            );
    }

    /// @dev Renders the eyes only.
    /// @param _color The color of eyes.
    /// @return Rendered SVG group with color.
    function _renderEyes(string memory _color)
        internal
        view
        returns (string memory)
    {
        return
            string(
                abi.encodePacked(
                    "<g>",
                    SSTORE2.read(parts["eyes"]),
                    "<style>",
                    string(abi.encodePacked(".eyes0{fill:#", _color, "}")),
                    "</style>"
                    "</g>"
                )
            );
    }

    /// @dev Renders the mouth only.
    /// @param _color The color of mouth.
    /// @return Rendered SVG group with color.
    function _renderMouth(string memory _color)
        internal
        view
        returns (string memory)
    {
        return
            string(
                abi.encodePacked(
                    "<g>",
                    SSTORE2.read(parts["mouth"]),
                    "<style>",
                    string(abi.encodePacked(".mouth0{fill:#", _color, "}")),
                    "</style>"
                    "</g>"
                )
            );
    }

}
