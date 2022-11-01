package compiler

import (
	"encoding/hex"
	"errors"
	"fmt"
	"math"
	"math/big"
	"strings"
)

// Reference: https://ethereum.org/en/developers/docs/evm/opcodes/
var opcodeMap = map[string]string{
	"STOP":           "00",
	"ADD":            "01",
	"MUL":            "02",
	"SUB":            "03",
	"DIV":            "04",
	"SDIV":           "05",
	"MOD":            "06",
	"SMOD":           "07",
	"ADDMOD":         "08",
	"MULMOD":         "09",
	"EXP":            "0A",
	"SIGNEXTEND":     "0B",
	"LT":             "10",
	"GT":             "11",
	"SLT":            "12",
	"SGT":            "13",
	"EQ":             "14",
	"ISZERO":         "15",
	"AND":            "16",
	"OR":             "17",
	"XOR":            "18",
	"NOT":            "19",
	"BYTE":           "1A",
	"SHL":            "1B",
	"SHR":            "1C",
	"SAR":            "1D",
	"SHA3":           "20",
	"ADDRESS":        "30",
	"BALANCE":        "31",
	"ORIGIN":         "32",
	"CALLER":         "33",
	"CALLVALUE":      "34",
	"CALLDATALOAD":   "35",
	"CALLDATASIZE":   "36",
	"CALLDATACOPY":   "37",
	"CODESIZE":       "38",
	"CODECOPY":       "39",
	"GASPRICE":       "3A",
	"EXTCODESIZE":    "3B",
	"EXTCODECOPY":    "3C",
	"RETURNDATASIZE": "3D",
	"RETURNDATACOPY": "3E",
	"EXTCODEHASH":    "3F",
	"BLOCKHASH":      "40",
	"COINBASE":       "41",
	"TIMESTAMP":      "42",
	"NUMBER":         "43",
	"DIFFICULTY":     "44",
	"GASLIMIT":       "45",
	"CHAINID":        "46",
	"SELFBALANCE":    "47",
	"BASEFEE":        "48",
	"POP":            "50",
	"MLOAD":          "51",
	"MSTORE":         "52",
	"MSTORE8":        "53",
	"SLOAD":          "54",
	"SSTORE":         "55",
	"JUMP":           "56",
	"JUMPI":          "57",
	"PC":             "58",
	"MSIZE":          "59",
	"GAS":            "5A",
	"JUMPDEST":       "5B",
	"PUSH1":          "60",
	"PUSH2":          "61",
	"PUSH3":          "62",
	"PUSH4":          "63",
	"PUSH5":          "64",
	"PUSH6":          "65",
	"PUSH7":          "66",
	"PUSH8":          "67",
	"PUSH9":          "68",
	"PUSH10":         "69",
	"PUSH11":         "6A",
	"PUSH12":         "6B",
	"PUSH13":         "6C",
	"PUSH14":         "6D",
	"PUSH15":         "6E",
	"PUSH16":         "6F",
	"PUSH17":         "70",
	"PUSH18":         "71",
	"PUSH19":         "72",
	"PUSH20":         "73",
	"PUSH21":         "74",
	"PUSH22":         "75",
	"PUSH23":         "76",
	"PUSH24":         "77",
	"PUSH25":         "78",
	"PUSH26":         "79",
	"PUSH27":         "7A",
	"PUSH28":         "7B",
	"PUSH29":         "7C",
	"PUSH30":         "7D",
	"PUSH31":         "7E",
	"PUSH32":         "7F",
	"DUP1":           "80",
	"DUP2":           "81",
	"DUP3":           "82",
	"DUP4":           "83",
	"DUP5":           "84",
	"DUP6":           "85",
	"DUP7":           "86",
	"DUP8":           "87",
	"DUP9":           "88",
	"DUP10":          "89",
	"DUP11":          "8A",
	"DUP12":          "8B",
	"DUP13":          "8C",
	"DUP14":          "8D",
	"DUP15":          "8E",
	"DUP16":          "8F",
	"SWAP1":          "90",
	"SWAP2":          "91",
	"SWAP3":          "92",
	"SWAP4":          "93",
	"SWAP5":          "94",
	"SWAP6":          "95",
	"SWAP7":          "96",
	"SWAP8":          "97",
	"SWAP9":          "98",
	"SWAP10":         "99",
	"SWAP11":         "9A",
	"SWAP12":         "9B",
	"SWAP13":         "9C",
	"SWAP14":         "9D",
	"SWAP15":         "9E",
	"SWAP16":         "9F",
	"LOG0":           "A0",
	"LOG1":           "A1",
	"LOG2":           "A2",
	"LOG3":           "A3",
	"LOG4":           "A4",
	"CREATE":         "F0",
	"CALL":           "F1",
	"CALLCODE":       "F2",
	"RETURN":         "F3",
	"DELEGATECALL":   "F4",
	"CREATE2":        "F5",
	"STATICCALL":     "FA",
	"REVERT":         "FD",
	"INVALID":        "FE",
	"SELFDESTRUCT":   "FF",
}

// PUSH opcodes to the amount of bytes they are able to push.
var pushmax = map[string]int{
	"PUSH1":  1,
	"PUSH2":  2,
	"PUSH3":  3,
	"PUSH4":  4,
	"PUSH5":  5,
	"PUSH6":  6,
	"PUSH7":  7,
	"PUSH8":  8,
	"PUSH9":  9,
	"PUSH10": 10,
	"PUSH11": 11,
	"PUSH12": 12,
	"PUSH13": 13,
	"PUSH14": 14,
	"PUSH15": 15,
	"PUSH16": 16,
	"PUSH17": 17,
	"PUSH18": 18,
	"PUSH19": 19,
	"PUSH20": 20,
	"PUSH21": 21,
	"PUSH22": 22,
	"PUSH23": 23,
	"PUSH24": 24,
	"PUSH25": 25,
	"PUSH26": 26,
	"PUSH27": 27,
	"PUSH28": 28,
	"PUSH29": 29,
	"PUSH30": 30,
	"PUSH31": 31,
	"PUSH32": 32,
}

// Returns true if the opcoode is a push instruction.
// It is slightly faster to use a switch statement to check
// for push values than cutting the string and checking
// validity. Also used by GETH. See
// https://github.com/ethereum/go-ethereum/blob/127dc5982e3484406eae0631326bbc356f914749/core/vm/opcodes.go#L27
func isPush(opcode string) bool {
	switch opcode {
	case "PUSH1", "PUSH2", "PUSH3", "PUSH4", "PUSH5", "PUSH6", "PUSH7", "PUSH8", "PUSH9", "PUSH10", "PUSH11", "PUSH12", "PUSH13", "PUSH14", "PUSH15", "PUSH16", "PUSH17", "PUSH18", "PUSH19", "PUSH20", "PUSH21", "PUSH22", "PUSH23", "PUSH24", "PUSH25", "PUSH26", "PUSH27", "PUSH28", "PUSH29", "PUSH30", "PUSH31", "PUSH32":
		return true
	}
	return false
}

// Returns true if the input is a valid hexadecimal.
// Used to check for push values.
func isValidHex(value string) bool {

	if value == "" {
		return false
	}

	_, err := hex.DecodeString(value)

	// The error for invalid byte is always returned before
	// odd length error (which can be ignored, since push values are padded later).
	return err == nil || errors.Is(err, hex.ErrLength)
}

// Returns whether the push instruction is capable of pushing the value.
// Accepts hex without 0x prefix. Assumes that hex is valid.
func isValidPushSize(pushcode string, val string) (int, int, bool) {

	bitsize := len(val) * 4 // Each hex is 4 bit

	pushbits := pushmax[pushcode] * 8

	return bitsize, pushbits, pushbits >= bitsize
}

// Pads the hex value to fit the PUSH instruction.
// PUSH4 0x20 => PUSH4 0x0020.
func pushPad(pushcode string, val string) string {
	return strings.Repeat("0", (pushmax[pushcode]*2)-len(val)) + val
}

// Removes comments ("//" or #) and trailing/leading whitespace.
func prettify(line string) string {
	// Remove comments.
	line = strings.Split(line, "//")[0]
	line = strings.Split(line, "#")[0]

	// Replace '' with ""
	line = strings.ReplaceAll(line, "'", "\"")

	// Remove leading and trailing whitespace.
	line = strings.TrimSpace(line)

	// Convert line to uppercase.
	line = strings.ToUpper(line)

	return line
}

// Compiles mnemonic source code to bytecode.
// Accepts slice of mnemoic code, split line by line.
func Compile(code []string) (string, error) {

	var bdr strings.Builder

	for index, line := range code {

		line = prettify(line)

		// Line is just comment or empty, skip.
		if line == "" {
			continue
		}

		// If instruction is push:
		if inst := strings.Split(line, " "); isPush(inst[0]) {

			// Push must have 2 arguments, return if argument length is incorrect.
			if l := len(inst); l != 2 {

				spec := "few"
				if l > 2 {
					spec = "many"
				}

				return "", errors.New(fmt.Sprintf("Line %d: Too %s arguments for push instruction.", index+1, spec))
			}

			// Write push instruction to string builder.
			bdr.WriteString(opcodeMap[inst[0]])

			// If the push argument has 0x prefix and is valid hex, append it to bytecode.
			if strings.HasPrefix(inst[1], "0X") {

				if !isValidHex(inst[1][2:]) {
					return "", errors.New(fmt.Sprintf("Line %d: Invalid hex value 0x%s.", index+1, inst[1][2:]))
				}

				// Remove trailing zeros, 0x001 = 0x1.
				hexvalue := strings.TrimLeft(inst[1][2:], "0")

				// If there is insufficient push size:
				if bitsize, pushMax, ok := isValidPushSize(inst[0], hexvalue); !ok {

					if bitsize > 256 {
						return "", errors.New(fmt.Sprintf("Line %d: The EVM can only work with values <=32 bytes. Use a smaller value.", index+1))
					}

					// Minimum PUSH instruction capable to push the given value.
					cap := int(math.Ceil(float64(bitsize) / 8))
					return "", errors.New(fmt.Sprintf("Line %d: %s can only push up to %d byte(s), requested %d bits push. Consider using PUSH%d.", index+1, inst[0], pushMax/8, bitsize, cap))
				}

				bdr.WriteString(pushPad(inst[0], hexvalue))

				// Otherwise convert int to hex and append to bytecode.
			} else {

				b := big.NewInt(0)

				b, ok := b.SetString(inst[1], 10)

				if !ok {
					return "", errors.New(fmt.Sprintf("Line %d: Unknown value %s. Push arguments only accept integers or hex values with 0x prefix.", index+1, inst[1]))
				}

				// Remove trailing zeros, 0x001 = 0x1.
				hexvalue := strings.TrimLeft(fmt.Sprintf("%x", b), "0")

				// insufficient push size:
				if bitsize, pushMax, ok := isValidPushSize(inst[0], hexvalue); !ok {

					if bitsize > 256 {
						return "", errors.New(fmt.Sprintf("Line %d: The EVM can only work with values <=32 bytes. Use a smaller value.", index+1))
					}

					// Minimum PUSH instruction capable to push the given value.
					cap := int(math.Ceil(float64(bitsize) / 8))
					return "", errors.New(fmt.Sprintf("Line %d: %s can only push up to %d byte(s), requested %d bits push. Consider using PUSH%d.", index+1, inst[0], pushMax/8, bitsize, cap))
				}

				bdr.WriteString(pushPad(inst[0], hexvalue))
			}

			// If the instruction is not push:
		} else {

			// Takes no inputs.
			if len(inst) != 1 {

				val := opcodeMap[inst[0]]

				// Unknown opcode.
				if val == "" {
					return "", errors.New(fmt.Sprintf("Line %d: Unknown opcode \"%s\".", index+1, inst[0]))
				}

				return "", errors.New(fmt.Sprintf("Line %d: Too many arguments for %s.", index+1, inst[0]))
			}

			quotDelim := '"'
			// Contains "..", raw opcode.
			if strings.ContainsRune(inst[0], quotDelim) {

				firstRune := rune(inst[0][0])
				lastRune := rune(inst[0][len(inst[0])-1])

				if firstRune != quotDelim || lastRune != quotDelim {
					return "", errors.New(fmt.Sprintf("Line %d: Unclosed opcode literal or unknown extra string.", index+1))
				}

				op := inst[0][1 : len(inst[0])-1]

				if len(op) > 2 {
					if op[:2] == "0x" {
						op = strings.Replace(op, "0x", "", 1)
					}
				}

				if !isValidHex(op) {
					return "", errors.New(fmt.Sprintf("Line %d: Opcode is an invalid hex value.", index+1))
				}

				// isValidHex() ignores odd length error.
				if len(op)%2 == 1 {
					return "", errors.New(fmt.Sprintf("Line %d: Opcode is odd-length.", index+1))
				}

				bdr.WriteString(op)

				// Just a regular opcode.
			} else {

				val := opcodeMap[inst[0]]

				// Unknown opcode.
				if val == "" {
					return "", errors.New(fmt.Sprintf("Line %d: Unknown opcode \"%s\".", index+1, inst[0]))
				}

				bdr.WriteString(val)

			}
		}

	}

	return "0x" + strings.ToLower(bdr.String()), nil

}
