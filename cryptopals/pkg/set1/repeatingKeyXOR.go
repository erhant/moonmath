package set1

import (
	"cryptopals/internal/common"
	"math"
)

// Given a byte array, XOR the bytes with a repeating key.
// This is also known as Repeating-Key XOR.
func RepeatingKeyXOREncrypt(pt, k []byte) []byte {
	ct := make([]byte, len(pt))
	for i := 0; i < len(pt); i++ {
		ct[i] = pt[i] ^ k[i%len(k)]
	}
	return ct
}

// Decrypting is the same as encrypting.
func RepeatingKeyXORDecrypt(ct, k []byte) []byte {
	return RepeatingKeyXOREncrypt(ct, k)
}

// Given a byte array, finds the possible key used and the corresponding plaintext.
func RepeatingKeyXORDecipher(ct []byte) ([]byte, []byte, error) {
	const KEYSIZE_MIN = 2
	const KEYSIZE_MAX = 40
	const KEYSIZE_COUNT = 2
	const NUM_BLOCKS = 8

	// first, find the keysize via edit distance of consecutive blocks
	var keySizes []int
	{
		keySizeCandidates := make([]int, 0)
		minDist := math.MaxFloat64
		for ks := KEYSIZE_MIN; ks <= KEYSIZE_MAX; ks++ {
			// find average normalized Hamming distance for N consecutive blocks
			dist := float64(0)
			for b := 1; b <= NUM_BLOCKS; b++ {
				d, err := common.HammingDistance(ct[:ks], ct[b*ks:(b+1)*ks])
				if err != nil {
					return nil, nil, err
				}
				dist += float64(d)
			}
			dist /= float64(NUM_BLOCKS) // average
			dist /= float64(ks)         // normalize

			// update results
			if dist <= minDist {
				minDist = dist
				keySizeCandidates = append(keySizeCandidates, ks)
			}
		}
		// take best few good choices
		keySizes = keySizeCandidates[common.Max(0, len(keySizeCandidates)-KEYSIZE_COUNT):]
	}

	// then, break the ciphertext into blocks of keysize length and take every KEYSIZE block separately.
	// bytes b, b+ks, b+2ks, ... are all encrypted with ks[0], a single byte!
	// we can concatenate them and run a single-byte XOR decipher.
	var ans []byte
	var anskey []byte
	var score float32 = math.MaxFloat32
	for _, keySize := range keySizes {
		// create the candidate key
		key := make([]byte, keySize)
		for i := 0; i < keySize; i++ {
			// find how many bytes you will have for that position of the key
			numBytes := int(math.Ceil(float64(len(ct)-i) / float64(keySize)))
			block := make([]byte, numBytes)
			for b := 0; b < numBytes; b++ {
				block[b] = ct[i+b*keySize]
			}
			// single-byte decipher
			_, k, _, err := SingleByteXORDecipher(block)
			if err != nil {
				return nil, nil, err
			}
			key[i] = k
		}

		// decrypt with the key
		pt := RepeatingKeyXORDecrypt(ct, key)

		// find the score w.r.t letter frequencies
		s := common.FittingQuotinent(common.LetterFreqs(pt))

		// update scores
		if s <= score {
			ans = pt
			anskey = key
			score = s
			// fmt.Println("Plaintext:\n", string(pt[:25]), "\nKey:", string(key), "\nScore:", score, "\n---------------------------")
		}
	}

	return ans, anskey, nil
}
