package set1

import (
	"cryptopals/internal/common"
	"log"
	"math"
)

// Given a byte array, XOR the bytes with a repeating key.
// This is also known as Repeating-Key XOR.
func VigenereCipher(pt []byte, key []byte) []byte {
	ct := make([]byte, len(pt))
	for i := 0; i < len(pt); i++ {
		ct[i] = pt[i] ^ key[i%len(key)]
	}
	return ct
}

// Given a byte array, finds the possible key used and the corresponding plaintext.
func VigenereDecipher(ct []byte) ([]byte, []byte, error) {
	const KEYSIZE_MIN = 2
	const KEYSIZE_MAX = 40

	// first, find the keysize via edit distance of consecutive blocks
	var keySize int
	{
		minDist := math.MaxFloat64
		for ks := KEYSIZE_MIN; ks < KEYSIZE_MAX; ks++ {
			// find average normalized edit distance for N consecutive blocks
			dist := float64(0)
			numBlocks := 8
			for b := 0; b < numBlocks; b++ {
				d, err := common.HammingDistance(ct[b*ks:(b+1)*ks], ct[(b+1)*ks:(b+2)*ks])
				if err != nil {
					return nil, nil, err
				}
				dist += float64(d)
			}
			dist /= float64(numBlocks) // average
			dist /= float64(ks)        // normalize
			// update results
			if dist < minDist {
				minDist = dist
				keySize = ks
			}
		}
	}

	log.Println("Keysize:", keySize)
	// then, break the ciphertext into blocks of keysize length and take every KEYSIZE block separately.
	// bytes b, b+ks, b+2ks, ... are all encrypted with ks[0], a single byte!
	// we can concatenate them and run a single-byte XOR decipher.
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

	// you have the key now, break the code by XORing again (XOR cancels itself)
	pt := VigenereCipher(ct, key)
	return pt, key, nil
}
