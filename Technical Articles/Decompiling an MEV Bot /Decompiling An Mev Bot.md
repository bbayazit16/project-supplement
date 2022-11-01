---
title: "Decompiling An MEV Bot"
date: "2022/08/24"
lastEdit: "2022/10/27"
---

As of writing, the total MEV extracted on mainnet has reached US $673,145,738. Yes, it is not thousands, it is in **millions!** [See the numbers yourself.](https://explore.flashbots.net/)

MEV attracts talented coders due to it's rewarding nature. MEV has become a source of income for the masters, whilst being a source of loss for the newcomers.

This vast sum of money, however, is not extracted easily. MEV can be described as the HFT (High-Frequency Trading) of blockchains - it is a zero-sum game, you either win, maybe sometimes get the breadcrumbs, or lose. Winning the game requires a deep understanding of how the EVM works, DeFi, and substantial programming skills, and even if you have all those, you may still encounter edge cases and fall into traps such as the [salmonella poisoned tokens.](https://github.com/Defi-Cartel/salmonella)

How MEV works is out of the scope of this article. I am personally not involved with MEV and recommend that you do your *own* research. I see MEV bots as a gold mine for quality code. The winners not only have highly optimized code, but also highly optimized strategies that push DeFi toit's limits. Investigating how such bots succeeded can help you discover unseen optimizations and strategies, and be a valuable addition to your coding knowledge.

I have discovered and decompiled an eye-catching profitale MEV bot, and identified a few key strategies.

## Discovering The Bot

While exploring some blocks on Etherscan, I came across a block with an unconventional order label, and saw that a bot had sandwiched a Uniswap trade. I clicked on the bot's contract, copied the bytecode, and pasted it to [dedaub decompiler.](https://library.dedaub.com/decompile)

I thought it'd be easy to read the source code after decompiling it. **But the decompiler output was bogus.** There were no functions at all, just a few headers and require statements. I tried using other decompilers such as [panoramix](https://github.com/eveem-org/panoramix), yet got similar outputs. This was the most readable output I could find:

```solidity
// Decompiled at https://library.dedaub.com/decompile

function uniswapV3SwapCallback() public payable { 
}

function onERC721Received() public payable { 
}

function onERC1155Received() public payable { 
}

function onERC1155BatchReceived() public payable { 
}

// Note: The function selector is not present in the original solidity code.
// However, we display it for the sake of completeness.

function __function_selector__(bytes4 function_selector) public payable { 
    if (block.number - msg.value) {
        if (!msg.value) {
            if (0xfa461e33 == function_selector >> 224) {
                uniswapV3SwapCallback(int256,int256,bytes);
            } else if (0xf0b9e5ba == function_selector >> 224) {
                onERC721Received(address,uint256,bytes);
            } else if (0xf23a6e61 == function_selector >> 224) {
                onERC1155Received(address,address,uint256,uint256,bytes);
            } else if (0xbc197c81 == function_selector >> 224) {
                onERC1155BatchReceived(address,address,uint256[],uint256[],bytes);
            } else {
                exit;
            }
        } else {
            exit;
        }
    } else {
        require(!(0xRedactedAddress - msg.sender));
    }
}
```

Cool, at least we have a result. But where is the main entry point to the contract? This code essentially does nothing! The fact that decompilers failed intrigued me; there had to be something all decompilers missed!

## Decompiling Opcodes Manually

Since decompilers failed, the only way to decompile the contract was to read each opcode one by one. Luckily disassemblers like [EtherVM](https://ethervm.io/decompile) or [dedaub](https://library.dedaub.com/decompile) convert opcodes to their human-readable names. This is what disassembled code looks like (as opposed to decompiled bytecode):

```sol
// ...
156  0x10f: JUMPDEST  
157  0x110: PUSH1     0x0
158  0x112: CALLDATALOAD
159  0x113: PUSH1     0xe0
160  0x115: SHR       
// ...
```

I dedicated myself to be the decompiler and read around 600 instructions. The whole process took me a few hours.

## What Does The Bot Do?

To understand how it does what it does, we should first understand what it does.

In summary, it’s a sandwich bot. If you know what that is, feel free to skip this section.

Definition wise, a person who operates an MEV bot or a person who searches for MEV opportunities is called a searcher.

Let’s say Bob is swapping 1 ETH for 10 X tokens. Before the swap is executed, the bot captures the transaction in the mempool and notices that the price of token X would increase significantly after Bob’s purchase. So the bot purchases some amount of X token right before Bob’s purchase, increasing the price of X token, which then reduces the amount Bob receives. Then Bob purchases 1 ETH for 9.9 X tokens, which again increases the price. Finally, the bot sells all of its tokens, making a profit from the increased price.

Such bots use [Flashbots bundles](https://docs.flashbots.net/flashbots-auction/overview) to ensure that the transactions are ordered correctly and atomically in a single block.

## Why Gas Costs are Specifically Important for MEV Bots

Sure, gas costs are important for every contract, but MEV bots have to be extra careful with each unit of gas they spend because they have to pay fees to miners using Flashbots to get their transactions prioritized. This is even more important for sandwich bots since a wrong transaction order would result in a loss.

Let’s take a look at why this bot is different from others:

## The Bytecode Wasn't Generated By A High-Level Language

The bot wasn't written in a language such as Solidity or Vyper, but in a *low-level language* like [Huff](https://huff.sh/). Modern decompilers can decompile almost every high-level contract, however low-level languages are harder to decompile. Low level languages leave the control flow and optimization to the developers' hands, which is exactly what a searcher needs to reduce gas costs.

This alone puts the searcher ahead of most of the competitors. Not only is the code fast, but also obscure and carefully crafted.

## Calldata is Not Zero-Padded

Each parameter for function calls occupy just as many bytes as they need, not more, or not less. High-level EVM languages pad every parameter to 32 bytes. If you want to execute a function that takes only one byte as input, you have to pad the byte with 31 extra zeros in the calldata (at least in high-level languages). This bot doesn't use zero padding, so instead of occupying 32 bytes in the calldata an address just occupies 20 bytes. To learn how this is done, you can read [my tutorial.](https://github.com/bbayazit16/deep-evm/tree/master/Parameter%20Tricks)

## The Bot Has No Function Selectors

This is why I believe the decompilers failed to parse function selectors.

In high-level languages like Solidity, the first 4 bytes of the calldata make up the function signature. During the beginning of contract execution, the first 4 bytes are compared to every available function signature using a switch block to find the offset of the called function.

Instead of using this approach, the bot uses a jump-table-like mechanism, which means it **jumps** to the offset specified in the **first byte of calldata.**

```sol
PUSH1 0x00
CALLDATALOAD
PUSH1 0x08
SHR
// The entire code block above loads the first byte of
// the calldata onto the stack

JUMP // this is where the smart move comes in 
// jump to the offset

STOP
```

This is a really smart alternative to function selectors instead of that makes the code obscure, causing the decompilers to fail (which eliminates competitors!), and allows the code to jump to the correct code block cheaply (at a constant cost!).

Although compilers can also optimize switch statements, function selectors are a special case because they are truncated hashes of function names that do not indicate a specific offset in the bytecode. They are dependent on the function name, so optimizing them this way is not possible.

This approach taken by the searcher however has one issue. The code only reads the first byte from the calldata to extract the jump offset. A byte can at most be 255 (0xff), so any JUMPDEST instruction located further than the 255th offset can't be jumped to. To solve this, the searcher uses what I call *shortcut blocks.*

## Shortcut Blocks

A shortcut is defined as the following:

```sol
JUMPDEST
PUSH2 <destination>
JUMP
```

Shortcuts are simply hard-coded symbolic links to other jump destinations in the code.

If the first byte of the calldata is 0x50, the code jumps to offset 80 0x50, which links to another offset in the bytecode that is bigger than 2 bytes, such as 0x100.

The bytecode has over 60 shortcuts, but only 10 of them lead to unique offsets. The rest of them jump to the same offset, which reverts the execution. To me, it looks like the searcher has intentionally filled the bytecode with reverting shortcuts until 0x100 so that they have space reserved for future function implementations.

This is another smart move to save gas. The searcher could instead choose to read the first 12 bits (3 hex characters) from the calldata, but they read the first byte only (2 hex characters). Each shortcut costs 12 gas to traverse, meanwhile adding 4 more bits to the calldata costs 16 extra gas. It is also worth noting that shortcut blocks significantly increase the deployed bytecode size (around 300 bytes!), hence increasing the deployment costs. But in the end, increasing deployment cost in return for cheaper execution costs puts the searcher in advantage.

## Using CALLVALUE for Uncle Bandit Protection

To protect against [uncle bandit attacks](https://www.mev.wiki/attack-examples/uncle-bandit-attack), the bot has to check **on-chain** if the code is executed in the desired target block. A pseudocode representation would be something like this:

```sol
function arb(uint256 _targetBlockNumber) external { // assume onlyOwner checks passed
    require(block.number == _targetBlockNumber, "Uncle bandit attack!")
    // continue
}
```

One way to pass the target block number to the contract is to use calldata, just like any other parameter. The current block number is around 15,000,000, so the transaction would take up 3-4 extra bytes in the calldata. This means using 64 extra units of gas.

Instead of using this, the searcher sends \<block number\> amount of Ether to the bot. This way no calldata cost is incurred. Moreover, extracting the block number does not require calldataload or bit shifting, simply using CALLVALUE opcode is sufficient.

However, this optimization is not generally applicable in regular contracts. For larger values the Ether sent would be much higher, making the optimization meaningless. Nevertheless, the bot has a withdraw function, so the searcher is not losing a single wei.

## Directly Calling the Swap Function

This is a simple and well-known optimization, but still something worth mentioning. Instead of using the Uniswap router, the bot directly transfers assets to the pair address, which is passed using 20 bytes of calldata.

## Ignoring Small Amounts

Each wei is 10^-18^ Ether, and that's too precise.

To reduce the space used in calldata, the bot multiplies the amount read from calldata by 10^13^ before swapping WETH by calling the pair address. So, to swap 1 WETH for X tokens, the transaction is submitted with 10^5^, 0x0186A0 (3 bytes) of calldata, instead of 10^18^, 0x0de0b6b3a7640000 (9 bytes). This reduces the calldata size by 6 bytes, and it's still precise enough!

## Further (Paranoid and Unnecessary) Optimizations

Each gas counts in MEV bots! It’s sometimes good to be paranoid! Let's change a few things:

## Replacing PUSH Opcodes

The bot frequently pushes 0 onto the stack. The bytecode representation of this is ```6000```, which costs 3 gas for each execution. Provided that 0 is pushed onto the stack before any external contract calls, PUSH1 0x00 can be replaced with RETURNDATASIZE `3D`, which costs only 2 gas during execution.

Similarly, instead of using `6001` to push 1 onto the stack, the bytes can be replaced with the CHAINID opcode `46` (again, costs 2 gas during execution!). 1 is not pushed onto the stack as frequently as zero, but it is still used once per execution for calldataload(1)). This optimization only works on the mainnet and it would break on other chains, but it saves some gas in this case!
