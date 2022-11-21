package set1

import (
	"cryptopals/internal/common"
	"math"
)

// Deciphers a ciphertext that has been encrypted with a single byte XOR.
// Returns in order: plaintext, key, score, error
func SingleByteXORDecipher(ct []byte) ([]byte, byte, float32, error) {
	var ans []byte
	var key byte
	var score float32 = math.MaxFloat32
	for b := 0; b < 256; b++ {
		// xor everything
		pt := make([]byte, len(ct))
		for i := 0; i < len(ct); i++ {
			pt[i] = ct[i] ^ byte(b)
		}

		// find the score w.r.t letter frequencies
		s := common.FittingQuotinent(common.LetterFreqs(pt))

		// update scores
		if s <= score {
			ans = pt
			key = byte(b)
			score = s
			// fmt.Println("Better:", string(pt), "\nKey:", key, "\nScore:", score)
		}

	}

	return ans, key, score, nil
}

func SingleByteXOREncrypt(pt []byte, k byte) []byte {
	ct := make([]byte, len(pt))
	for i := 0; i < len(ct); i++ {
		ct[i] = pt[i] ^ k
	}
	return ct
}

// Decrypting is same as encrypting.
func SingleByteXORDecrypt(ct []byte, k byte) []byte {
	return SingleByteXOREncrypt(ct, k)
}
