// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface IVerifier {
    function verify(
        bytes calldata _proof,
        bytes32[] calldata _publicInputs
    ) external view returns (bool);
}

contract SecretNFTClub {
    IVerifier public immutable verifier;
    bytes32 public immutable secretHash;

    mapping(address => bool) public hasJoined;
    mapping(address => uint256) public memberTokenId;

    uint256 private _nextTokenId;

    event MemberJoined(address indexed member, uint256 indexed tokenId);

    error AlreadyMember();
    error InvalidProof();

    constructor(bytes32 _secretHash, address _verifier) {
        secretHash = _secretHash;
        verifier = IVerifier(_verifier);
    }

    function join(bytes calldata proof) external {
        if (hasJoined[msg.sender]) revert AlreadyMember();

        // Prepare public inputs (just the secret hash)
        bytes32[] memory publicInputs = new bytes32[](1);
        publicInputs[0] = secretHash;

        // Verify the zero-knowledge proof
        if (!verifier.verify(proof, publicInputs)) revert InvalidProof();

        // Proof verified! Grant membership
        uint256 tokenId = _nextTokenId++;
        hasJoined[msg.sender] = true;
        memberTokenId[msg.sender] = tokenId;

        emit MemberJoined(msg.sender, tokenId);
    }

    function isMember(address account) external view returns (bool) {
        return hasJoined[account];
    }

    function totalMembers() external view returns (uint256) {
        return _nextTokenId;
    }
}
