package bufutils

import (
	"bytes"
	"encoding/hex"
	"math/rand"
	"time"
)

// Receives *bytes.buffer as input and returns hex.
func BufHex(buf *bytes.Buffer) string {
	return hex.EncodeToString(buf.Bytes())

	// obuf := bytes.NewBuffer([]byte{})
	// for _, val := range buf.Bytes() {
	// 	v := strconv.FormatInt(int64(val), 16)
	// 	if len(v) == 1 {
	// 		v = "0" + v
	// 	}
	// 	obuf.WriteString(v)
	// }
	// return obuf.String()
}

// Receives hex string (with or without 0x) and returns
// *bytes.Buffer. Hex string must have even length.
func HexBuf(hexstr string) (*bytes.Buffer, error) {
	if hexstr[:2] == "0x" {
		hexstr = hexstr[2:]
	}

	res, err := hex.DecodeString(hexstr)

	if err != nil {
		return nil, err
	}

	return bytes.NewBuffer(res), nil

	// if len(hexstr)%2 != 0 {
	// 	return nil, errors.New("Hex string must have even length.")
	// }
	// buf := bytes.NewBuffer([]byte{})
	// for i := 0; i < len(hexstr); i += 2 {
	// 	v, err := strconv.ParseInt(hexstr[i:i+2], 16, 64)
	// 	buf.Write([]byte{byte(int8(v))})
	// 	if err != nil {
	// 		return nil, err
	// 	}
	// }
	// return buf, nil
}

// Iterates over *bytes.Buffer, returning int
// to the channel "ch".
func BufIter(buf *bytes.Buffer, ch chan int) {
	for _, b := range buf.Bytes() {
		ch <- int(uint(b))
	}

	defer close(ch)
}

// Size is not set to uint in the functions below
// for simplicity. Never provide negative size value!

// Generates random byte array with "size" elements
// by using current time in UnixNano as seed.
func RandomBytes(size int) []byte {
	b := make([]byte, size)

	rand.Seed(time.Now().UnixNano())

	rand.Read(b)

	return b
}

// Generates random hex string with "length" length
// by using current time in UnixNano as seed.
func RandomHex(length int) string {
	ext := ""

	if length%2 != 0 {
		ext = string(hex.EncodeToString(RandomBytes(2))[0])
	}

	return hex.EncodeToString(RandomBytes(length/2)) + ext
}

// Generates random *bytes.Buffer with "size" elements
// by using current time in UnixNano as seed.
func RandomBuffer(size int) *bytes.Buffer {
	b := RandomBytes(size)

	return bytes.NewBuffer(b)
}

// Removes 0x prefix if there is any, otherwise appends it.
func OX(hexstr *string) {
	if (*hexstr)[:2] == "0x" {
		*hexstr = (*hexstr)[2:]
	} else {
		*hexstr = "0x" + *hexstr
	}
}
