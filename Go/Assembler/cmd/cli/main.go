package main

import (
	"encoding/hex"
	"flag"
	"fmt"
	"io/ioutil"
	"math/big"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/bbayazit16/mnemo/internal/ethutil"
	"github.com/bbayazit16/mnemo/pkg/compiler"
)

var Version string

func compileHelp() {
	fmt.Println("compile <file>")
	fmt.Println("		Compiles opcode mnemonic. If no file extension is provided, the file is assumed to have .evm extension.")
}

func deployHelp() {
	fmt.Println("deploy --bytecode <> --rpc <> --private-key <>")
	fmt.Println("		Deploys a new contract with bytecode. Note that this function actually sends a transaction.")
	fmt.Println("		Tip: Fork Ethereum Mainnet using Ganache and use Ganache RPC to deploy, or use mnemo quick to test locally with mainnet fork.")

	fmt.Println("Required:")
	fmt.Println("		--bytecode or --file	Bytecode starting with 0x to deploy or file name to compile and deploy.")
	fmt.Println("		--rpc			The rpc url. If not prefixed with http or https, http prefix is automatically added.")
	fmt.Println("		--private-key		Private key of the deployer.")

	fmt.Println("Options:")
	fmt.Println("		--value 		Default: 0")
	fmt.Println("		--gas 			Default: 5,000,000")
	fmt.Println("		--gas-price 		Default: 10 gwei")
	fmt.Println("		--init-code 		Default: <60><08>8060<09>3d393df3")
}

func quickHelp() {
	fmt.Println("quick <file>")
	fmt.Println("		Uses Ganache to fork Ethereum Mainnet, automatically compiles file and executes bytecode.")
	fmt.Println("		Requires Ganache to be installed. npm i -g ganache")

	fmt.Println("Options:")
	fmt.Println("		--deploy-value 		Default: 0")
	fmt.Println("		--call-value 		Default: 0")
	fmt.Println("		--rpc			RPC to be forked. Default: https://rpc.ankr.com/eth")
	fmt.Println("		--calldata		Default: 0x")

}

func help() {
	fmt.Println()
	fmt.Println("Available commands:")
	fmt.Println()
	compileHelp()
	fmt.Println()
	deployHelp()
	fmt.Println()
	quickHelp()
	fmt.Println()
}

// func callHelp() {
// 	fmt.Println("Usage: call <address> <data> <rpc> <private key>")
// 	fmt.Println("	Call a contract.")
// 	fmt.Println("Options:")
// 	fmt.Println("	--value 		Default: 0")
// 	fmt.Println("	--gas 			Default: 5,000,000")
// 	fmt.Println("	--gas-price 		Default: 10 gwei")
// }

func main() {

	if len(os.Args) < 2 {

		help()

		os.Exit(0)
	}

	// Parse the command.
	switch os.Args[1] {

	case "version":

		fmt.Println(Version)

	case "compile":

		if len(os.Args) != 3 {
			compileHelp()
			os.Exit(0)
		}

		fname := os.Args[2]

		if filepath.Ext(fname) == "" {
			fname += ".evm"
		}

		contents, err := ioutil.ReadFile(fname)

		if err != nil {
			fmt.Println("Error reading file:", err)
			os.Exit(0)
		}

		val, err := compiler.Compile(strings.Split(string(contents), "\n"))

		if err != nil {
			fmt.Println("Error compiling file:", err)
			os.Exit(0)
		}

		fmt.Println(val)
		os.Exit(0)

	case "deploy":

		if len(os.Args) < 7 {
			deployHelp()
			os.Exit(0)
		}

		deploy := flag.NewFlagSet("deploy", flag.ExitOnError)

		bytecode := deploy.String("bytecode", "", "")
		rpc := deploy.String("rpc", "", "")
		priv := deploy.String("private-key", "", "")
		fname := deploy.String("file", "", "")

		valueStr := deploy.String("value", "0", "")
		gas := deploy.Uint64("gas", 5_000_000, "")
		gasPriceStr := deploy.String("gas-price", "10000000000", "")
		initcode := deploy.String("init-code", "", "")

		deploy.Parse(os.Args[2:])

		f := *fname

		if f != "" && *bytecode != "" {
			deployHelp()
			os.Exit(0)
		}

		if !(*bytecode == "" || f == "") || *rpc == "" || *priv == "" {
			deployHelp()
			os.Exit(0)
		}

		if f != "" {

			if filepath.Ext(f) == "" {
				f += ".evm"
			}

			contents, err := ioutil.ReadFile(f)

			if err != nil {
				fmt.Println("Error reading file:", err)
				os.Exit(0)
			}

			val, err := compiler.Compile(strings.Split(string(contents), "\n"))

			if err != nil {
				fmt.Println("Error compiling file:", err)
				os.Exit(0)
			}

			*bytecode = val

		}

		value := big.NewInt(0)
		value.SetString(*valueStr, 10)

		gasPrice := big.NewInt(0)
		gasPrice.SetString(*gasPriceStr, 10)

		updates := make(chan string, 2)
		errs := make(chan error, 1)

		go ethutil.Deploy(updates, errs, *bytecode, *initcode, value, *gas, gasPrice, *rpc, *priv)

		for i := 0; i < 2; i++ {

			select {

			case msg := <-updates:

				fmt.Println(msg)

			case err := <-errs:

				fmt.Println(err)

				os.Exit(0)

			case <-time.After(time.Minute):

				fmt.Println("Transaction timed out")

				os.Exit(0)
			}

		}

	case "quick":

		if len(os.Args) < 3 {

			quickHelp()

			os.Exit(0)
		}

		fname := os.Args[2]

		quick := flag.NewFlagSet("quick", flag.ExitOnError)
		deployValue := quick.String("deploy-value", "0", "")
		callValue := quick.String("call-value", "0", "")
		rpc := quick.String("rpc", "https://rpc.ankr.com/eth", "")
		calldata := quick.String("calldata", "0x", "")

		quick.Parse(os.Args[3:])

		dv := big.NewInt(0)
		dv, ok := dv.SetString(*deployValue, 10)

		if !ok {
			fmt.Println("Invalid deployment value entered.")
		}

		cv := big.NewInt(0)
		cv, ok = cv.SetString(*callValue, 10)

		if !ok {
			fmt.Println("Invalid callvalue entered.")
		}

		if !strings.Contains(fname, ".") {
			fname += ".evm"
		}

		contents, err := ioutil.ReadFile(fname)

		if err != nil {
			fmt.Println("Error reading file:", err)
			os.Exit(0)
		}

		bytcode, err := compiler.Compile(strings.Split(string(contents), "\n"))

		if err != nil {
			fmt.Println("Error compiling file:", err)
			os.Exit(0)
		}

		if strings.HasPrefix(strings.ToLower(*calldata), "0x") {
			*calldata = (*calldata)[2:]
		}

		data, err := hex.DecodeString(*calldata)

		if err != nil {
			fmt.Println("Invalid calldata.")
		}

		ret, err := ethutil.Quick(bytcode, *rpc, dv, cv, data)

		if err != nil {
			fmt.Println("Error running bytecode:", err)
			os.Exit(0)
		}

		fmt.Println(ret)

	default:
		help()

		/*

			See https://github.com/gakonst/foundry/tree/master/cast

		*/
		// case "call":

		// 	if len(os.Args) < 6 {
		// 		callHelp()
		// 		os.Exit(0)
		// 	}

		// 	address := os.Args[2]
		// 	dataStr := os.Args[3]
		// 	rpc := os.Args[4]
		// 	priv := os.Args[5]

		// 	if strings.HasPrefix(dataStr, "0x") {
		// 		dataStr = dataStr[2:]
		// 	}

		// 	call := flag.NewFlagSet("call", flag.ExitOnError)
		// 	valueStr := call.String("value", "0", "")
		// 	gas := call.Uint64("gas", 5_000_000, "")
		// 	gasPriceStr := call.String("gas-price", "10000000000", "")

		// 	call.Parse(os.Args[6:])

		// 	value := big.NewInt(0)
		// 	value.SetString(*valueStr, 10)

		// 	gasPrice := big.NewInt(0)
		// 	gasPrice.SetString(*gasPriceStr, 10)

		// 	data, err := hex.DecodeString(dataStr)

		// 	if err != nil {
		// 		fmt.Println("Invalid calldata.")
		// 		os.Exit(0)
		// 	}

		// 	updates := make(chan string, 2)
		// 	errs := make(chan error, 1)

		// 	ethutil.Call(updates, errs, address, value, data, *gas, gasPrice, rpc, priv)

		// 	for ____ {
		// 		select {

		// 		case msg := <-updates:

		// 			if msg == "terminate" {
		// 				os.Exit(0)
		// 			}

		// 			fmt.Println(msg)

		// 		case err := <-errs:

		// 			fmt.Println(err)
		// 			os.Exit(0)

		// 		case <-time.After(time.Minute):

		// 			fmt.Println("Transaction timed out")
		// 			os.Exit(0)

		// 		}
		// 	}
	}
}
