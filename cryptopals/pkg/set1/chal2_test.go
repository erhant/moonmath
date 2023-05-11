package set1_test

import (
	"cryptopals/pkg/set1"
	"encoding/hex"
	"testing"
)

func TestChal2(t *testing.T) {
	a := []byte("1c0111001f010100061a024b53535009181c")
	b := []byte("686974207468652062756c6c277320657965")

	// decode
	aD := make([]byte, hex.DecodedLen(len(a)))
	_, err1 := hex.Decode(aD, a)
	if err1 != nil {
		t.Error(err1)
		return
	}
	bD := make([]byte, hex.DecodedLen(len(b)))
	_, err2 := hex.Decode(bD, b)
	if err2 != nil {
		t.Error(err2)
		return
	}

	output, err := set1.XOR(aD, bD)
	if err2 != nil {
		t.Error(err)
		return
	}
	outputStr := hex.EncodeToString(output)
	expectedStr := "746865206b696420646f6e277420706c6179"
	if outputStr != expectedStr {
		t.Errorf("Wrong output.\nHave: %s\nNeed: %s\n", outputStr, expectedStr)
		return
	}
}
