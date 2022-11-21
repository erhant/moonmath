package set1_test

import (
	"bytes"
	"cryptopals/pkg/set1"
	"testing"
)

func TestChal1(t *testing.T) {
	input := []byte("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
	expectedOutput := []byte("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
	output, err := set1.HexToBase64(input)
	if err != nil {
		t.Error(err)
		return
	}
	if !bytes.Equal(output, expectedOutput) {
		t.Errorf("Wrong output.\nHave: %s\nNeed: %s\n", output, expectedOutput)
		return
	}
}
