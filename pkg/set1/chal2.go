package set1

import (
	"cryptopals/internal/common"
	"encoding/hex"
)

func FixedXOR(a, b []byte) ([]byte, error) {
	// check lengths
	aD := make([]byte, hex.DecodedLen(len(a)))
	bD := make([]byte, hex.DecodedLen(len(b)))
	if len(aD) != len(bD) {
		return nil, common.ErrLenMismatch
	}

	// decode
	_, err1 := hex.Decode(aD, a)
	if err1 != nil {
		return nil, err1
	}
	_, err2 := hex.Decode(bD, b)
	if err2 != nil {
		return nil, err2
	}

	// xor
	ans := make([]byte, len(aD))
	for i := 0; i < len(aD); i++ {
		ans[i] = aD[i] ^ bD[i]
	}
	return ans, nil
}
