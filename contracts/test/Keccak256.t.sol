// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";


contract Keccak256Test is Test {

    struct BalanceABI {
        bytes32 assetName;
        int256 balance;
        int256 maxWithdrawAmount;
    }

    struct PositionABI {
        bytes32 symbolName;
        PositionItemABI[] positionItems;
    }

    struct PositionItemABI {
        int256 positionAmount;
        int256 entryPrice;
        uint64 leverage;
        int256 unrealizedPnl;
        int256 returnOnEquity;
    }

    struct UserInfo1 {
        address addr1;
        BalanceABI[] balances;
        PositionABI[] positions;
    }


    function setUp() public {

    }

    function createUserInfo1() public pure returns (UserInfo1 memory) {
        // 构造 BalanceABI 对象
        BalanceABI memory balance = BalanceABI({
            assetName: bytes32(0),
            balance: 0,
            maxWithdrawAmount: 0
        });

        // 构造 PositionABI 对象
        PositionABI memory position = PositionABI({
            symbolName: bytes32(0),
            positionItems: new PositionItemABI[](0)
        });

        // 构造 UserInfo1 对象
        UserInfo1 memory userInfo = UserInfo1({
            addr1: address(0x1111111111111111111111111111111111111111),
            balances: new BalanceABI[](0),
            positions: new PositionABI[](0)
        });

//        userInfo.balances[0] = balance;
//        userInfo.positions[0] = position;

        return userInfo;
    }

    // Test that the Fibonacci proof verification passes with real proof data
    function testValidFibonacciProof() public view {
        bytes memory encode = abi.encode(createUserInfo1());
        console.logBytes(encode);
    }
}
