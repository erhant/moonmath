package set1

import (
	"cryptopals/internal/common"
	"encoding/hex"
	"math"
)

func SingleByteXORCipher(chal []byte) ([]byte, byte, float32, error) {
	len := hex.DecodedLen(len(chal))

	// decode hex
	chalDec := make([]byte, len)
	_, err := hex.Decode(chalDec, chal)
	if err != nil {
		return nil, 0, 0, err
	}

	// brute force bytes
	var ans []byte
	var key byte
	var score float32 = math.MaxFloat32
	for b := 0; b < 256; b++ {
		// xor everything & calculate score
		xor := make([]byte, len)
		for i := 0; i < len; i++ {
			xor[i] = chalDec[i] ^ byte(b)
		}

		// update results
		s := common.FittingQuotinent(common.LetterFreqs(xor))
		if s <= score {
			ans = xor
			key = byte(b)
			score = s
			// fmt.Println("Better:", string(xor), "\nKey:", key, "\nScore:", score)
		}

	}

	return ans, key, score, nil
}
