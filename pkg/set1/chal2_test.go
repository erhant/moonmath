package set1_test

import (
	"cryptopals/pkg/set1"
	"encoding/hex"
	"testing"
)

func TestChal2(t *testing.T) {
	a := []byte("1c0111001f010100061a024b53535009181c")
	b := []byte("686974207468652062756c6c277320657965")
	expectedStr := "746865206b696420646f6e277420706c6179"
	output, err := set1.XOR(a, b)
	if err != nil {
		t.Error(err)
	}
	outputStr := hex.EncodeToString(output)
	if outputStr != expectedStr {
		t.Errorf("Wrong output.\nHave: %s\nNeed: %s\n", outputStr, expectedStr)
	}
}
