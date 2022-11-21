package set1_test

import (
	"cryptopals/pkg/set1"
	"encoding/hex"
	"testing"
)

func TestChal3(t *testing.T) {
	ct := []byte("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")

	// decode hex
	len := hex.DecodedLen(len(ct))
	ctDec := make([]byte, len)
	_, err := hex.Decode(ctDec, ct)
	if err != nil {
		t.Error(err)
		return
	}

	// crack
	res, _, _, err := set1.SingleByteXORDecipher(ctDec)
	if err != nil {
		t.Error(err)
		return
	}

	// there are two answers if you include capital letters too!
	// key 88  --> "Cooking MC's like a pound of bacon" (score: 0.72218305)
	// key 120 --> "cOOKINGmcSLIKEAPOUNDOFBACON"			  (score: 0.72218287)
	expectedRes := "Cooking MC's like a pound of bacon"
	if string(res) != expectedRes {
		t.Errorf("Wrong output.\nHave: %s\nNeed: %s\n", string(res), expectedRes)
		return
	}
}
