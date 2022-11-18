package common

import (
	"cryptopals/internal/constants"
	"fmt"
)

// Calculate byte frequencies in a given byte array. Returns a mapping of
// byte to frequency (percentage)
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

// Compare a given map of frequencies to english alphabet letter frequencies.
// A smaller result means that the given frequencies match to that of the english alphabet.
func FittingQuotinent(freqs map[byte]float32) float32 {
	sum := float32(0)
	for i := 0; i < 256; i++ {
		// find absolute difference
		diff := freqs[byte(i)] - constants.EnglishLetterFreqs[byte(i)]
		if diff < 0 {
			diff = -diff
		}
		// accumulate the differences
		sum += diff
	}
	// normalize to alphabet size (uint8 = 256 unique bytes)
	sum /= 256
	return sum
}

// Find the Levenstein Edit Distance between two byte arrays.
// Implementation of the Wikipedia algorithm using 2 rows
func LevensteinEditDistance(s, t []byte) int {
	m := len(s)
	n := len(t)

	v0 := make([]int, n+1) // first row
	v1 := make([]int, n+1) // second row

	// initialize first row
	for i := range v0 {
		v0[i] = i
	}

	var delCost int // deletion cost
	var insCost int // insertion cost
	var subCost int // substitution cost
	for i := 0; i < m-1; i++ {
		// calculate second row
		v1[0] = i + 1
		for j := 0; j < n-1; j++ {
			delCost = v0[j+1] + 1
			insCost = v1[j] + 1
			if s[i] == t[j] {
				subCost = v0[j]
			} else {
				subCost = v0[j] + 1
			}
			// find minimum cost
			v1[j+1] = Min(delCost, insCost, subCost)
		}

		// copy v1 to v0 for next iteration (pointer swap would work too)
		fmt.Println("V0:", v0, "\nV1:", v1)
		copy(v0, v1)

	}

	// some bug?
	return v0[n]
}
