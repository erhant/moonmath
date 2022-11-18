package set1_test

import (
	"cryptopals/internal/constants"
	"cryptopals/pkg/set1"
	"testing"
)

func TestChal3(t *testing.T) {
	chal := []byte("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
	res, key, _, err := set1.SingleByteXORCipher(chal)
	if err != nil {
		t.Error(err)
	}
	// there are two answers!
	// key 88  --> "Cooking MC's like a pound of bacon" (score: 0.72218305)
	// key 120 --> "cOOKINGmcSLIKEAPOUNDOFBACON"			  (score: 0.72218287)
	if string(res) != "cOOKINGmcSLIKEAPOUNDOFBACON" && key != 120 {
		t.Error(constants.ErrWrongResult)
	}
}
