package set1_test

import (
	"cryptopals/pkg/set1"
	"testing"
)

func TestChal3(t *testing.T) {
	chal := []byte("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
	res, b, score, err := set1.SingleByteXORCipher(chal)
	if err != nil {
		t.Error(err)
	}
	t.Log(res, b, score)
}
