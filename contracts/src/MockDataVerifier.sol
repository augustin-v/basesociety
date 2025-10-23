// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./interfaces/IERC7857DataVerifier.sol";

contract MockDataVerifier is IERC7857DataVerifier {
    function verifyTransferValidity(
        TransferValidityProof[] calldata _proofs
    ) external pure override returns (TransferValidityProofOutput[] memory) {
        TransferValidityProofOutput[] memory outputs = new TransferValidityProofOutput[](_proofs.length);

        for (uint256 i = 0; i < _proofs.length; i++) {
            outputs[i] = TransferValidityProofOutput({
                oldDataHash: _proofs[i].accessProof.oldDataHash,
                newDataHash: _proofs[i].accessProof.newDataHash,
                sealedKey: _proofs[i].ownershipProof.sealedKey,
                encryptedPubKey: _proofs[i].ownershipProof.encryptedPubKey,
                wantedKey: _proofs[i].accessProof.encryptedPubKey, // Assuming wantedKey is encryptedPubKey for mock
                accessAssistant: address(0), // Mock value
                accessProofNonce: _proofs[i].accessProof.nonce,
                ownershipProofNonce: _proofs[i].ownershipProof.nonce
            });
        }
        return outputs;
    }

    // Mock implementation for the newly added mock function in the interface
    function mockVerifyTransferValidity(
        TransferValidityProof[] calldata _proofs
    ) external pure override returns (TransferValidityProofOutput[] memory) {
        TransferValidityProofOutput[] memory outputs = new TransferValidityProofOutput[](_proofs.length);

        for (uint256 i = 0; i < _proofs.length; i++) {
            outputs[i] = TransferValidityProofOutput({
                oldDataHash: _proofs[i].accessProof.oldDataHash,
                newDataHash: _proofs[i].accessProof.newDataHash,
                sealedKey: _proofs[i].ownershipProof.sealedKey,
                encryptedPubKey: _proofs[i].ownershipProof.encryptedPubKey,
                wantedKey: _proofs[i].accessProof.encryptedPubKey, // Assuming wantedKey is encryptedPubKey for mock
                accessAssistant: address(0), // Mock value
                accessProofNonce: _proofs[i].accessProof.nonce,
                ownershipProofNonce: _proofs[i].ownershipProof.nonce
            });
        }
        return outputs;
    }
}