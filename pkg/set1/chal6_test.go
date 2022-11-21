package set1_test

import (
	"cryptopals/internal/common"
	"cryptopals/pkg/set1"
	"encoding/base64"
	"io/ioutil"
	"testing"
)

func TestChal6(t *testing.T) {
	// test hamming distance
	{
		s1 := []byte("this is a test")
		s2 := []byte("wokka wokka!!!")
		dist, err := common.HammingDistance(s1, s2)
		if err != nil {
			t.Error(err)
		}
		expectedDist := 37
		if dist != expectedDist {
			t.Errorf("Wrong output.\nHave: %d\nNeed: %d\n", dist, expectedDist)
		}
	}

	// read file (base64 encoded)
	fileb64, err := ioutil.ReadFile("../../res/set1/6.txt")
	if err != nil {
		t.Error(err)
	}

	// decode
	ct := make([]byte, base64.StdEncoding.DecodedLen(len(fileb64)))
	base64.StdEncoding.Decode(ct, fileb64)
	pt, key, err := set1.RepeatingKeyXORDecipher(ct)
	if err != nil {
		t.Error(err)
	}
	t.Log("KEY:", string(key))
	t.Log("PT:", string(pt))

}
