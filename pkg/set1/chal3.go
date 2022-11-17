package set1

import (
	"encoding/hex"
	"fmt"
)

// as per the ETAOIN SHRDLU joke on the question.
// see also: https://en.wikipedia.org/wiki/Etaoin_shrdlu
var weights = map[byte]int{
	byte('U'): 2,
	byte('u'): 2,
	byte('L'): 3,
	byte('l'): 3,
	byte('D'): 4,
	byte('d'): 4,
	byte('R'): 5,
	byte('r'): 5,
	byte('H'): 6,
	byte('h'): 6,
	byte('S'): 7,
	byte('s'): 7,
	byte(' '): 8,
	byte('N'): 9,
	byte('n'): 9,
	byte('I'): 10,
	byte('i'): 10,
	byte('O'): 11,
	byte('o'): 11,
	byte('A'): 12,
	byte('a'): 12,
	byte('T'): 13,
	byte('t'): 13,
	byte('E'): 14,
	byte('e'): 14,
}

func SingleByteXORCipher(chal []byte) ([]byte, byte, int, error) {
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
	var score int
	for b := 0; b < 256; b++ {
		// xor everything & calculate score
		s := 0
		xor := make([]byte, len)
		for i := 0; i < len; i++ {
			c := chal[i] ^ byte(b)
			xor[i] = c
			s += weights[c]

		}

		// update results
		if s > score {
			ans = xor
			key = byte(b)
			score = s
		}

		fmt.Println("Score:", s)
	}

	fmt.Println(weights[byte('E')])

	return ans, key, score, nil
}
