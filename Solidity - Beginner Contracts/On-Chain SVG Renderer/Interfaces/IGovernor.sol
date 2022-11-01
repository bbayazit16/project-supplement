// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.12;

/// @title ICharacterDaoGovernor
/// @notice The interface for CharacterDAO Governor
interface ICharacterDaoGovernor {

    /// ================================ ///
    /// ======== Struct & Enums ======== ///
    /// ================================ ///

    /// @notice Describes the state of the proposal.
    enum ProposalState {
        PROPOSED,
        DEFEATED,
        SUCCEEDED,
        QUEUED,
        EXECUTED,
        EXPIRED
    }

    /// @notice Details of a proposal.
    struct Proposal {
        // Proposal state.
        ProposalState proposalState;
        // The address that proposed the proposal.
        address proposer;
        // Proposal ID.
        uint256 proposalID;
        // The timestamp proposal was proposed at.
        uint256 proposalDate;
        // The timestamp voting finishes at.
        uint256 votingEndDate;
        // The date proposal may be executed.
        uint256 executionDate;
        // The timestamp proposal expires on.
        uint256 expiryDate;
        // The votes supporting the proposal.
        uint256 votesFor;
        // The votes against the proposal.
        uint256 votesAgainst;
        // The values that'll be passed to the targets.
        uint256[] values;
        // The addresses to call.
        address[] targets;
        // The signatures for functions to call.
        string[] signatures;
        // The actions for the proposal, excluding the function signatures.
        bytes[] data;
        // The packed signatures and data. Signatures and data are stored seperately
        // to increase transparency on the function being executed.
        bytes[] packedExecutionData;
        // Description of the proposal.
        string proposalDetails;
    }

    /// ======================== ///
    /// ======== Events ======== ///
    /// ======================== ///

    /// @notice Emitted when a new proposal has been proposed.
    /// @param ID The proposal ID.
    /// @param proposer The proposer.
    event ProposalProposed(uint256 indexed ID, address indexed proposer);

    /// @notice Emitted when a new vote is casted.
    /// @param ID The proposal ID.
    /// @param votingPower The number of Characters the voter used to vote.
    /// @param voter The voter.
    /// @param support Whether the voter is for/against the proposal.
    event Vote(
        uint256 indexed ID,
        uint256 votingPower,
        address indexed voter,
        bool support
    );

    /// @notice Emitted when a proposal is executed.
    /// @param ID Executed proposal's ID.
    /// @param returnValues Return values from the execution.
    event ProposalExecuted(uint256 indexed ID, bytes[] returnValues);

    /// @notice Emitted when governance params are adjusted.
    /// @param _voteTreshold Characters required to create a proposal.
    /// @param _percentVotesRequired Percentage of votes required for a proposal
    /// to be considered as passed.
    /// @param _votingTime Voting time in seconds.
    /// @param _delayTime The time proposal will be executed after success, in seconds.
    /// @param _expiryTime The time proposal will expire if not executed, in seconds.
    /// @param _maxActions The maximum actions that can be proposed per 
    /// @param _CharacterAddress The address to Character ERC721.
    event GovernanceParamsChanged(
        uint256 _voteTreshold,
        uint256 _percentVotesRequired,
        uint256 _votingTime,
        uint256 _delayTime,
        uint256 _expiryTime,
        uint256 _maxActions,
        address _CharacterAddress
    );

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
    ) external;

    /// @notice Allows a DAO member to vote on a proposal.
    /// @param _proposalID The proposal to vote on.
    /// @param _support Whether the sender is for / against the proposal.
    /// @param _withCharacters The Characters the sender uses to vote.
    function vote(
        uint256 _proposalID,
        bool _support,
        uint256[] calldata _withCharacters
    ) external;

    /// @notice Executes a passed proposal.
    /// @param _proposalID The proposal ID.
    function executeProposal(uint256 _proposalID) external;

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
    ) external;

    /// ================================ ///
    /// ======== View-Functions ======== ///
    /// ================================ ///

    /// @notice Returns proposal details.
    function getProposal(uint256 _proposalID) external view returns (Proposal memory);

    /// @notice Returns all proposals.
    function getProposals() external view returns (Proposal[] memory);

    /// @notice Returns the ID of latest proposal.
    function getLatestProposalID() external view returns (uint256);
}
