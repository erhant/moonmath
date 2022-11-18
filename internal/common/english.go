package common

import (
	"cryptopals/internal/constants"
)

func LetterFreqs(src []byte) map[byte]float32 {
	ans := make(map[byte]float32)

	// count letters
	for _, b := range src {
		ans[b] += 1
	}

	// divide by length and normalize to percentage
	n := float32(len(src))
	for i := 0; i < 256; i++ {
		ans[byte(i)] = ans[byte(i)] * 100 / n
	}

	return ans
}

func FittingQuotinent(freqs map[byte]float32) float32 {
	sum := float32(0)
	for i := 0; i < 256; i++ {
		// find absolute difference
		diff := freqs[byte(i)] - constants.EnglishLetterFreqs[byte(i)]
		if diff < 0 {
			diff = -diff
		}

		sum += diff
	}
	sum /= 256
	return sum
}
