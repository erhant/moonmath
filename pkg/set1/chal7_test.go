package set1_test

import (
	"cryptopals/internal/constants"
	"cryptopals/pkg/set1"
	"encoding/base64"
	"io/ioutil"
	"testing"
)

func TestChal7(t *testing.T) {
	// read file (base64 encoded)
	fileb64, err := ioutil.ReadFile("../../res/set1/7.txt")
	if err != nil {
		t.Error(err)
	}

	// decode
	ct := make([]byte, base64.StdEncoding.DecodedLen(len(fileb64)))
	base64.StdEncoding.Decode(ct, fileb64)

	// decrypt
	key := []byte("YELLOW SUBMARINE")
	pt, _, err := set1.AES128ECBDecrypt(ct, key)
	if err != nil {
		t.Error(err)
	}

	expectedPrefix := "I'm back and I'm ringin' the bell"
	if string(pt)[:len(expectedPrefix)] != expectedPrefix {
		t.Error(constants.ErrWrongResult)
	}
}
