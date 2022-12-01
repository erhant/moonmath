package set1_test

import (
	"cryptopals/internal/common"
	"cryptopals/pkg/set1"
	"encoding/base64"
	"os"
	"testing"
)

func TestChal6(t *testing.T) {
	// t.Skip("skip: test is a bit long")

	// test hamming distance
	{
		s1 := []byte("this is a test")
		s2 := []byte("wokka wokka!!!")
		dist, err := common.HammingDistance(s1, s2)
		if err != nil {
			t.Error(err)
			return
		}
		expectedDist := 37
		if dist != expectedDist {
			t.Errorf("Wrong output.\nHave: %d\nNeed: %d\n", dist, expectedDist)
			return
		}
	}

	// read file (base64 encoded)
	fileb64, err := os.ReadFile("../../res/set1/6.txt")
	if err != nil {
		t.Error(err)
		return
	}

	// decode
	ct := make([]byte, base64.StdEncoding.DecodedLen(len(fileb64)))
	base64.StdEncoding.Decode(ct, fileb64)
	pt, key, err := set1.RepeatingKeyXORDecipher(ct)
	if err != nil {
		t.Error(err)
		return
	}
	t.Log("PLAINTEXT:", string(pt), "\n")

	expectedKey := "Terminator X: Bring the noise"
	// there are two answers if you include capital letters too!
	// key: teRmINaTORx:brINGthENOISE
	// key: Terminator X: Bring the noise
	if string(key) != expectedKey {
		t.Errorf("Wrong output.\nHave: %s\nNeed: %s\n", key, expectedKey)
		return
	}

}
