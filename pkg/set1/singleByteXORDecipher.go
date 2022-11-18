package set1

import (
	"cryptopals/internal/common"
	"encoding/hex"
	"math"
)

func SingleByteXORDecipher(ct []byte) ([]byte, byte, float32, error) {
	len := hex.DecodedLen(len(ct))

	// decode hex
	ctDec := make([]byte, len)
	_, err := hex.Decode(ctDec, ct)
	if err != nil {
		return nil, 0, 0, err
	}

	// brute force bytes
	var ans []byte
	var key byte
	var score float32 = math.MaxFloat32
	for b := 0; b < 256; b++ {
		// xor everything & calculate score
		pt := make([]byte, len)
		for i := 0; i < len; i++ {
			pt[i] = ctDec[i] ^ byte(b)
		}

		// update results
		s := common.FittingQuotinent(common.LetterFreqs(pt))
		if s <= score {
			ans = pt
			key = byte(b)
			score = s
			// fmt.Println("Better:", string(xor), "\nKey:", key, "\nScore:", score)
		}

	}

	return ans, key, score, nil
}
