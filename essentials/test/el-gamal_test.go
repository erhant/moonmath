package essentials_test

import (
	"bytes"
	"testing"

	essentials "cryptogoraphy/pkg"
	common "cryptogoraphy/pkg/common"
)

func TestElGamalEncryption(t *testing.T) {
	// keygen
	eg, err := essentials.NewElGamal(256)
	if err != nil {
		t.Error(err)
	}

	// encrypt
	m := []byte("very secret")
	c1, c2, err := eg.Encrypt(m)
	if err != nil {
		t.Error(err)
	}

	// decrypt
	pt, err := eg.Decrypt(c1, c2)
	if err != nil {
		t.Error(err)
	}

	if !bytes.Equal(pt, m) {
		t.Error(common.ErrCorrectness)
	}

}
