// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.13;

contract PaymentChannel {
    fallback() external payable {
        assembly {
            switch shr(224, calldataload(0))
            // createChannel(address,uint256,uint256)
            case 0x513D144B {
                let receiver := shr(96, calldataload(4))

                let duration := shr(224, calldataload(24))

                let eps := calldataload(28)

                let totalCost := mul(duration, eps)

                if iszero(
                    and(
                        eq(callvalue(), totalCost),
                        eq(div(totalCost, duration), eps)
                    )
                ) {
                    revert(0, 0)
                }

                calldatacopy(0, 4, 56)
                mstore(56, timestamp())

                let receipt := keccak256(0, 60)

                sstore(receipt, shr(32, mload(0)))
                sstore(add(receipt, 1), eps)

                mstore(0, receipt)
                return(0, 32)
            }
            default {
                calldatacopy(0x00, 0x00, calldatasize())
                return(0x00, calldatasize())
            }
        }
    }
}
