# Mnemo

[![Build and Test](https://github.com/bbayazit16/mnemo/actions/workflows/go.yml/badge.svg)](https://github.com/bbayazit16/mnemo/actions/workflows/go.yml) [![Go Report Card](https://goreportcard.com/badge/github.com/bbayazit16/mnemo)](https://goreportcard.com/report/github.com/bbayazit16/mnemo)

Mnemo is a command line tool for:
- Compiling opcode mnemonics into bytecode
- Testing them against Ethereum Mainnet fork using Ganache
- Deploying them

written in Go.

# Usage

Mnemo comes with three modes: compile, deploy and quick.

Compile compiles opcode mnemonic in file to bytecode. Preferred extension of the file is .evm, though you can use any extension you want..

Deploy deploys any bytecode to the chain using given rpc url and private key.

Quick mode compiles opcode mnemonic in file, forks Ethereum Mainnet using Ganache in the background, then deploys and calls the contract and prints the returned value. Fork RPC URL, values and calldata can be specified using flags (See ~$ mnemo help). Requires Ganache to be installed (npm i -g ganache).

# Examples

hello.evm:
```solidity
// Opcode Mnemonic that retrieves
// USDC total supply.


// Supports comments
// in both # and //
// But # is discouraged

PUSH32 0x18160ddd00000000000000000000000000000000000000000000000000000000
RETURNDATASIZE

// Opcodes or any value can be directly
// written in bytecode if surrounded with
// "" or ''

// So this is equivalent to MSTORE

"52"

MSIZE // return size (32 bytes)
RETURNDATASIZE // return offset
PUSH1 4 // args size
RETURNDATASIZE // args offset
PUSH20 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 // USDC

// Automatically converts int to hex
// All hex must be prefixed with 0x
PUSH2 12000 // gas for staticcall

// Capitalization doesn't matter
// but it is discouraged...
staticcall

POP

RETURNDATASIZE
CALLVALUE
RETURN
```

Run locally by forking Ethereum Mainnet using:
```console
~$ mnemo quick hello.evm
>> 00000000000000000000000000000000000000000000000000a07d59ac0b52ce # USDC total supply
```


Compile to bytecode using:
```console
~$ mnemo compile hello.evm
>> 0x7f18160ddd000000000000000000000000000000000000000000000000000000003d52593d60043d73a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48612ee0fa503d34f3
```

Deploy to any rpc using:

```console
~$ mnemo deploy --bytecode 0x7f18160ddd000000000000000000000000000000000000000000000000000000003d52593d60043d73a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48612ee0fa503d34f3 --rpc $MUMBAI_RPC --private-key $PRIVATE_KEY

>> Transaction sent: 0x523498bc1ac5f2882b88c8a22886cf749525a51c3007cdc5126276a096a3ac67
>> Contract deployed to: 0xfb052b297757d880ea95Be02d7073f3DB5417cB1
```

or

```console
~$ mnemo deploy --file hello.evm --rpc $MUMBAI_RPC --private-key $PRIVATE_KEY
```

Mnemo:
- Accepts PUSH values in both integer or hex format with 0x prefix
- Allows comments in # and // (# is discouraged)
- Accepts mixed/lower/uppercase opcodes
- Allows bytecode to be directly written if surrounded with "" or ''
- Pads PUSH values. Eg PUSH4 0x20 == 0x0020


Mnemo warns if:
- PUSH opcode doesn't have a value
- Invalid hex or unknown opcode
- PUSH opcode is incapable of pushing the given value
- Integer/Hex bigger than 32 bytes
- Odd-length opcode written in "", '' syntax


# License

[See license notice](test/)

[GNU AGPL 3.0](LICENSE)