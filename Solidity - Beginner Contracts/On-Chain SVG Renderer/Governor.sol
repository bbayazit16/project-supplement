// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

// Import Governor Interface
import "./Interfaces/IGovernor.sol";

// Import Character Interface
import "./Interfaces/ICharacter.sol";

/// @title CharacterDAOGovernor.
/// @notice The governor and wallet of Character DAO.
contract CharacterDAOGovernor is ICharacterDaoGovernor {
    /// =========================== ///
    /// ======== Variables ======== ///
    /// =========================== ///

    /// @notice Latest proposal ID, incremented with each proposal.
    uint256 private proposalID = 1;

    /// @notice The Character ERC721.
    ICharacter private Character;

    /// @notice The amount of Characters required to propose a proposal.
    uint256 public VOTE_TRESHOLD;

    /// @notice The percentage of for votes required for a proposal to be
    /// considered success.
    uint256 public PERCENT_VOTES_REQUIRED;

    /// @notice The voting time in seconds.
    uint256 public VOTING_TIME;

    /// @notice The delay time in seconds.
    /// eg. The proposal will be executed X seconds after the proposal succeeds.
    uint256 public DELAY_TIME;

    /// @notice The expiry time in seconds.
    /// eg. The proposal will expire if not executed in X seconds after it succeeds.
    uint256 public EXPIRY_TIME;

    /// @notice Maximum actions that can be specified per proposal.
    uint256 public MAX_ACTIONS;

    /// ========================== ///
    /// ======== Mappings ======== ///
    /// ========================== ///

    /// @notice Mapping from proposal ID to proposal details.
    mapping(uint256 => Proposal) private proposals;

    /// @notice Mapping from Proposal ID to tokenID to vote status.
    mapping(uint256 => mapping(uint256 => bool)) private hasVotedInProposal;

    /// ============================= ///
    /// ======== Constructor ======== ///
    /// ============================= ///

    /// @param _voteTreshold Characters required to create a proposal.
    /// @param _percentVotesRequired Percentage of votes required for a proposal
    /// to be considered as passed.
    /// @param _votingTime Voting time in seconds.
    /// @param _delayTime The time proposal will be executed after success, in seconds.
    /// @param _expiryTime The time proposal will expire if not executed, in seconds.
    /// @param _maxActions The maximum actions that can be proposed per 
    /// @param _CharacterAddress The address to Character ERC721.
    constructor(
        uint256 _voteTreshold,
        uint256 _percentVotesRequired,
        uint256 _votingTime,
        uint256 _delayTime,
        uint256 _expiryTime,
        uint256 _maxActions,
        address _CharacterAddress
    ) {
        VOTE_TRESHOLD = _voteTreshold;

        PERCENT_VOTES_REQUIRED = _percentVotesRequired;

        VOTING_TIME = _votingTime;

        DELAY_TIME = _delayTime;

        EXPIRY_TIME = _expiryTime;

        MAX_ACTIONS = _maxActions;

        Character = ICharacter(_CharacterAddress);
    }

    /// ========================================== ///
    /// ======== State-Changing Functions ======== ///
    /// ========================================== ///

    /// @notice Used to create a proposal.
    /// @param _targets The callees.
    /// @param _values The values.
    /// @param _signatures The function signatures.
    /// @param _data The function data.
    /// @param _withCharacters The Characters the proposer uses to propose.
    /// @param _description The proposal description.
    function propose(
        address[] calldata _targets,
        uint256[] calldata _values,
        string[] calldata _signatures,
        bytes[] calldata _data,
        uint256[] calldata _withCharacters,
        string calldata _description
    ) external {
        uint256 CharactersLength = _withCharacters.length;

        uint256 _latestID = proposalID;

        uint256 targetLength = _targets.length;

        require(
            Character.totalSupply() >= 30,
            "CharacterDAO::Governor: At least 30 Characters are required before proposals can be made."
        );

        require(
            targetLength != 0 &&
            targetLength <= MAX_ACTIONS &&
            targetLength == _values.length &&
            targetLength == _signatures.length &&
            targetLength == _data.length,
            "CharacterDAO::Governor: Invalid proposal actions length."
        );

        require(
            CharactersLength > VOTE_TRESHOLD,
            "CharacterDAO::Governor: Insufficient Character balance to create a proposal."
        );

        uint256 execDate = block.timestamp + VOTING_TIME + DELAY_TIME;

        bytes[] memory packedExecutionData = new bytes[](targetLength);
        for (uint256 i = 0; i < targetLength; i++) {
            packedExecutionData[i] = abi.encodePacked(
                bytes4(keccak256(bytes(_signatures[i]))),
                _data[i]
            );
        }

        proposals[proposalID] = Proposal({
            proposalState: ProposalState.PROPOSED,
            proposer: msg.sender,
            proposalID: _latestID,
            proposalDate: block.timestamp,
            votingEndDate: block.timestamp + VOTING_TIME,
            executionDate: execDate,
            expiryDate: execDate + EXPIRY_TIME,
            votesFor: CharactersLength,
            votesAgainst: 0,
            values: _values,
            targets: _targets,
            signatures: _signatures,
            data: _data,
            packedExecutionData: packedExecutionData,
            proposalDetails: _description
        });

        _checkOwnershipAndVote(proposalID, _withCharacters);

        emit ProposalProposed(proposalID, msg.sender);

        proposalID++;
    }

    /// @notice Allows a DAO member to vote on a proposal.
    /// @param _proposalID The proposal to vote on.
    /// @param _support Whether the sender is for / against the proposal.
    /// @param _withCharacters The Characters the sender uses to vote.
    function vote(
        uint256 _proposalID,
        bool _support,
        uint256[] calldata _withCharacters
    ) external {
        Proposal storage proposal = proposals[_proposalID];

        uint256 CharactersLength = _withCharacters.length;

        require(
            _proposalID != 0 &&
            _proposalID < proposalID &&
            block.timestamp <= proposal.votingEndDate,
            "CharacterDAO::Governor: Invalid proposal."
        );

        if (_support) {
            proposal.votesFor += CharactersLength;
        } else {
            proposal.votesAgainst += CharactersLength;
        }

        _checkOwnershipAndVote(_proposalID, _withCharacters);

        emit Vote(_proposalID, CharactersLength, msg.sender, _support);
    }

    /// @notice Executes a passed proposal.
    /// @param _proposalID The proposal ID.
    function executeProposal(uint256 _proposalID) external {
        Proposal storage proposal = proposals[_proposalID];

        require(
            block.timestamp >= proposal.executionDate &&
            block.timestamp <= proposal.expiryDate &&
            proposal.proposalState == ProposalState.PROPOSED &&
            (proposal.votesFor * PERCENT_VOTES_REQUIRED) / 100 >=
            proposal.votesAgainst,
            "CharacterDAO::Governor: Proposal can not be executed."
        );

        bytes[] memory returnValues = new bytes[](proposal.targets.length);

        for (uint256 i = 0; i < proposal.targets.length; i++) {
            (bool success, bytes memory data) = proposal.targets[i].call{
                value: proposal.values[i]
            }(proposal.packedExecutionData[i]);
            returnValues[i] = data;
            require(
                success,
                "CharacterDAO::Governor: Proposal execution reverted."
            );
        }

        proposal.proposalState = ProposalState.EXECUTED;

        emit ProposalExecuted(proposalID, returnValues);
    }

    /// @notice Allows governance params to be adjusted.
    /// @param _voteTreshold Characters required to create a proposal.
    /// @param _percentVotesRequired Percentage of votes required for a proposal
    /// to be considered as passed.
    /// @param _votingTime Voting time in seconds.
    /// @param _delayTime The time proposal will be executed after success, in seconds.
    /// @param _expiryTime The time proposal will expire if not executed, in seconds.
    /// @param _maxActions The maximum actions that can be proposed per 
    /// @param _CharacterAddress The address to Character ERC721.
    function changeGovernanceParams(
        uint256 _voteTreshold,
        uint256 _percentVotesRequired,
        uint256 _votingTime,
        uint256 _delayTime,
        uint256 _expiryTime,
        uint256 _maxActions,
        address _CharacterAddress
    ) external {
        require(
            msg.sender == address(this),
            "CharacterDAO::Governor: Governance may only be changed with a proposal."
        );

        VOTE_TRESHOLD = _voteTreshold;

        PERCENT_VOTES_REQUIRED = _percentVotesRequired;

        VOTING_TIME = _votingTime;

        DELAY_TIME = _delayTime;

        EXPIRY_TIME = _expiryTime;

        MAX_ACTIONS = _maxActions;

        Character = ICharacter(_CharacterAddress);

        emit GovernanceParamsChanged(
            _voteTreshold,
            _percentVotesRequired,
            _votingTime,
            _delayTime,
            _expiryTime,
            _maxActions,
            _CharacterAddress
        );

    }

    /// @notice Allow the contract to receive ETH.
    receive() external payable{}

    /// ==================================== ///
    /// ======== Internal Functions ======== ///
    /// ==================================== ///

    /// @notice Sets voted proposals and checks for token ownership.
    /// @param _proposalID The proposal ID.
    /// @param _Characters The Characters to vote and check.
    function _checkOwnershipAndVote(
        uint256 _proposalID,
        uint256[] memory _Characters
    ) internal {
        for (uint256 i = 0; i < _Characters.length; i++) {
            uint256 tokenID = _Characters[i];

            require(
                Character.owns(tokenID, msg.sender) &&
                    !hasVotedInProposal[_proposalID][tokenID],
                "CharacterDAO::Governor: Not permitted to vote."
            );

            hasVotedInProposal[_proposalID][tokenID] = true;
            Character.votedOnProposal(tokenID, _proposalID);
        }
    }

    /// ================================================ ///
    /// ======== View-Helper-Internal-Functions ======== ///
    /// ================================================ ///

    /// @notice Returns proposal details.
    /// @param _proposalID The proposal ID.
    function _getProposal(
        uint256 _proposalID
    ) internal view returns (Proposal memory) {
        Proposal memory proposal = proposals[_proposalID];

        if (proposal.proposalState != ProposalState.EXECUTED) {

            if (block.timestamp >= proposal.expiryDate) {
                proposal.proposalState = ProposalState.EXPIRED;
            } else if (block.timestamp >= proposal.votingEndDate) {
                if ((proposal.votesFor * PERCENT_VOTES_REQUIRED) / 100 >= proposal.votesAgainst) {
                    proposal.proposalState = ProposalState.SUCCEEDED;
                } else {
                    proposal.proposalState = ProposalState.DEFEATED;
                }
            }

        }

        return proposal;
    }

    /// ================================ ///
    /// ======== View-Functions ======== ///
    /// ================================ ///

    /// @notice Returns proposal details.
    function getProposal(
        uint256 _proposalID
    ) external view returns (Proposal memory) {
        return _getProposal(_proposalID);
    }

    /// @notice Returns all proposals.
    function getProposals() external view returns (Proposal[] memory) {
        Proposal[] memory _proposals = new Proposal[](proposalID - 1);
        for (uint256 i = 1; i < proposalID; i++) {
            _proposals[i - 1] = _getProposal(i);
        }
        return _proposals;
    }

    /// @notice Returns the ID of latest proposal.
    function getLatestProposalID() external view returns (uint256) {
        return proposalID - 1;
    }
}
