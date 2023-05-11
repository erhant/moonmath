package common

import (
	"cryptopals/internal/constants"
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
		copy(v0, v1)

	}

	return v0[n]
}

// Finds the hamming distance, which is the total number of differing bits.
// We simply XOR bit by bit, and count the set bits in the result.
func HammingDistance(a, b []byte) (int, error) {
	if len(a) != len(b) {
		return 0, constants.ErrLenMismatch
	}
	dist := 0
	for i := 0; i < len(a); i++ {
		dist += NumOfSetBits(a[i] ^ b[i])
	}
	return dist, nil
}

// Returns the number of set bits (1) within a byte.
func NumOfSetBits(b byte) int {
	count := 0
	for b != 0 {
		if b&1 == 1 {
			count++
		}
		b >>= 1
	}
	return count
}

func RepeatingBlocks(bytes []byte, size int) int {
	repeats := make(map[string]int)
	s := 0
	for bs, be := 0, size; be <= len(bytes); bs, be = bs+size, be+size {
		repeats[string(bytes[bs:be])]++
	}
	for _, r := range repeats {
		s += r - 1 // minus 1 to ignore single-occurences
	}
	return s
}
