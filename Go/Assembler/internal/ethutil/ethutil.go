package ethutil

import (
	"context"
	"crypto/ecdsa"
	"errors"
	"fmt"
	"math"
	"math/big"
	"os/exec"
	"strconv"
	"strings"
	"time"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
)

// Pads the hex value to fit the PUSH instruction.
// PUSH4 0x20 => PUSH4 0x0020.
func zeropad(pushmax int, val string) string {
	return strings.Repeat("0", (pushmax*2)-len(val)) + val
}

func waitTillDone(client *ethclient.Client, hash common.Hash) error {
	_, pending, err := client.TransactionByHash(context.Background(), hash)

	if err != nil {
		return err
	}

	if !pending {
		return nil
	}

	time.Sleep(time.Second)
	err = waitTillDone(client, hash)

	if err != nil {
		return err
	}

	return nil
}

// Build and return init code.
func initCode(bytecode string) string {
	bclen := strconv.FormatInt(int64(len(bytecode)/2), 16)

	bchexlen := len(bclen)

	pushsize := int(math.Ceil(float64(bchexlen) / 2))

	pushcode := strconv.FormatInt(int64(95+pushsize), 16)

	// There is no need to increment push opcode in
	// the offset. Maximum push size used in the first
	// opcode would be (see below) PUSH32, which has
	// 33 bytes offset. 33 + 8 <= 255, so the offset
	// should always remain as PUSH1.
	offset := strconv.FormatInt(int64(8+pushsize), 16)

	// PUSH1 SIZE
	// DUP1
	// PUSH1 OFFSET, 0x9 if PUSH1 else increment by 1
	// RETURNDATASIZE
	// CODECOPY
	// RETURNDATASIZE
	// RETURN
	// 60088060093d393df3

	return pushcode + zeropad(pushsize, bclen) + "8060" + zeropad(1, offset) + "3d393df3"
}

// Sign deployment transaction without to field.
func deployCode(bytecode []byte, value *big.Int, gas uint64, gasPrice *big.Int, pk string, client *ethclient.Client) (*types.Transaction, error) {

	priv, err := crypto.HexToECDSA(pk)

	if err != nil {
		return nil, err
	}

	pubKey := priv.Public()
	pub, ok := pubKey.(*ecdsa.PublicKey)

	if !ok {
		return nil, errors.New("Unable to cast public key to ECDSA.")
	}

	from := crypto.PubkeyToAddress(*pub)

	nonce, err := client.PendingNonceAt(context.Background(), from)

	if err != nil {
		return nil, err
	}

	chainId, err := client.NetworkID(context.Background())

	if err != nil {
		return nil, err
	}

	signer := types.NewEIP155Signer(chainId)

	tx := types.NewContractCreation(nonce, value, gas, gasPrice, bytecode)

	signed, err := types.SignTx(tx, signer, priv)

	if err != nil {
		return nil, err
	}

	return signed, nil
}

// Sign regular transaction with to field.
func signTx(to string, value *big.Int, data []byte, gas uint64, gasPrice *big.Int, pk string, client *ethclient.Client) (*types.Transaction, error) {

	priv, err := crypto.HexToECDSA(pk)

	if err != nil {
		return nil, err
	}

	pubKey := priv.Public()
	pub, ok := pubKey.(*ecdsa.PublicKey)

	if !ok {
		return nil, errors.New("Unable to cast public key to ECDSA.")
	}

	from := crypto.PubkeyToAddress(*pub)

	toAddress := common.HexToAddress(to)

	nonce, err := client.PendingNonceAt(context.Background(), from)

	if err != nil {
		return nil, err
	}

	chainId, err := client.NetworkID(context.Background())

	if err != nil {
		return nil, err
	}

	signer := types.NewEIP155Signer(chainId)

	tx := types.NewTransaction(nonce, toAddress, value, gas, gasPrice, data)

	signed, err := types.SignTx(tx, signer, priv)

	if err != nil {
		return nil, err
	}

	return signed, nil
}

// Sends *types.Transaction to client.
func sendTx(tx *types.Transaction, client *ethclient.Client) (common.Hash, error) {
	err := client.SendTransaction(context.Background(), tx)

	if err != nil {
		return common.Hash{}, err
	}

	return tx.Hash(), nil
}

// Deploys bytecode to rpc url with private key.
func Deploy(updates chan string, errs chan error, bytecode string, initcode string, value *big.Int, gas uint64, gasPrice *big.Int, rpc string, priv string) {

	if strings.HasPrefix(strings.ToLower(priv), "0x") {
		priv = priv[2:]
	}

	if strings.HasPrefix(strings.ToLower(bytecode), "0x") {
		bytecode = bytecode[2:]
	}

	if len(rpc) > 4 {
		if rpc[:4] != "http" {
			rpc = "http://" + rpc
		}
	}

	client, err := ethclient.Dial(rpc)

	if err != nil {
		errs <- errors.New("Unable to connect to rpc.")
		time.Sleep(time.Second)
	}

	init := initcode
	if initcode == "" {
		init = initCode(bytecode)
	}

	tx, err := deployCode(common.Hex2Bytes(init+bytecode), value, gas, gasPrice, priv, client)

	if err != nil {
		errs <- err
		time.Sleep(time.Second)
	}

	hash, err := sendTx(tx, client)

	if err != nil {
		errs <- err
		time.Sleep(time.Second)
	}

	updates <- "Transaction sent: " + hash.Hex()

	waitTillDone(client, hash)

	receipt, err := client.TransactionReceipt(context.Background(), hash)

	if err != nil {
		errs <- err
		time.Sleep(time.Second)
	}

	updates <- "Contract deployed to: " + receipt.ContractAddress.Hex()

}

func Quick(bytecode string, rpc string, deployValue *big.Int, callValue *big.Int, data []byte) (string, error) {

	if strings.HasPrefix(strings.ToLower(bytecode), "0x") {
		bytecode = bytecode[2:]
	}

	if rpc[:4] != "http" {
		rpc = "http://" + rpc
	}

	cmd := exec.Command("ganache", "--fork", rpc, "--port", "8512", "--host", "127.0.0.1", "-s", "1", "--chain.chainId", "1")

	if err := cmd.Start(); err != nil {
		return "", errors.New("Unable to start ganache. Ganache-cli was recently renamed to ganache. Do you have ganache installed? npm i -g ganache.")
	}

	time.Sleep(time.Second)

	client, err := ethclient.Dial("http://127.0.0.1:8512")

	if err != nil {
		cmd.Process.Kill()
		return "", err
	}

	priv := "f0906fd865d515fed0f4563175bfc5da0eb44cce630fac63a8ede30816d2e6ed"

	gp := big.NewInt(0)
	gp.SetString("5000000000000", 10)

	tx, err := deployCode(common.Hex2Bytes(initCode(bytecode)+bytecode), deployValue, 10_000_000, gp, priv, client)

	if err != nil {
		cmd.Process.Kill()
		return "", err
	}

	hash, err := sendTx(tx, client)

	if err != nil {
		cmd.Process.Kill()
		return "", err
	}

	waitTillDone(client, hash)

	receipt, err := client.TransactionReceipt(context.Background(), hash)

	if err != nil {
		cmd.Process.Kill()
		return "", err
	}

	addr := common.HexToAddress("0xA16842b28FF96Ec695008996F0D85BE705A2c4Dd")

	ret, err := client.CallContract(context.Background(), ethereum.CallMsg{Data: data, Value: callValue, From: addr, To: &receipt.ContractAddress}, nil)

	if err != nil {
		cmd.Process.Kill()
		return "", err
	}

	if err := cmd.Process.Kill(); err != nil {
		fmt.Println(err)
		return "", err
	}

	return common.Bytes2Hex(ret), nil
}

// Call a contract at rpc url with private key.
// func Call(updates chan string, errs chan error, to string, value *big.Int, data []byte, gas uint64, gasPrice *big.Int, rpc string, priv string) {

// 	if strings.HasPrefix(strings.ToLower(priv), "0x") {
// 		priv = priv[2:]
// 	}

// 	if rpc[:4] != "http" {
// 		rpc = "http://" + rpc
// 	}

// 	client, err := ethclient.Dial(rpc)

// 	if err != nil {
// 		errs <- err
// 	}

// 	tx, err := signTx(to, value, data, gas, gasPrice, priv, client)

// 	if err != nil {
// 		errs <- err
// 	}

// 	hash, err := sendTx(tx, client)

// 	if err != nil {
// 		errs <- err
// 	}

// 	updates <- "Transaction sent: " + hash.Hex()

// 	waitTillDone(client, hash)

// 	receipt, err := client.TransactionReceipt(context.Background(), hash)

// 	if err != nil {
// 		errs <- err
// 	}

// 	logs := receipt.Logs

// 	if len(logs) == 0 {
// 		updates <- "Logs:\n"
// 		updates <- "[0]: 0x"
// 		updates <- "terminate"
// 	}

// 	for i, log := range logs {
// 		updates <- fmt.Sprintf("[%d]: %s\n", i, hex.EncodeToString(log.Data))
// 	}

// 	updates <- "terminate"
// }
