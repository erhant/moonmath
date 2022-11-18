package set1

import (
	"cryptopals/internal/common"
	"encoding/hex"
	"math"
)

// Deciphers a ciphertext that has been encrypted with a single byte XOR.
// Returns in order: plaintext, key, score, error
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
		// xor everything
		pt := make([]byte, len)
		for i := 0; i < len; i++ {
			pt[i] = ctDec[i] ^ byte(b)
		}

		// find the score w.r.t letter frequencies
		s := common.FittingQuotinent(common.LetterFreqs(pt))

		// update scores
		if s <= score {
			ans = pt
			key = byte(b)
			score = s
			// fmt.Println("Better:", string(xor), "\nKey:", key, "\nScore:", score)
		}

	}

	return ans, key, score, nil
}
