package essentials_test

import (
	"bytes"
	"testing"

	essentials "cryptogoraphy/pkg"
	common "cryptogoraphy/pkg/common"
)

func TestRSAEncryption(t *testing.T) {
	// generate keys
	rsa, err := essentials.NewRSA(1024)
	if err != nil {
		t.Error(err)
	}

	// encrypt
	m := []byte("herro worl")
	ct := rsa.Encrypt(m)

	// decrypt
	pt := rsa.Decrypt(ct)

	// verify that the decrypted message matches the original
	if !bytes.Equal(pt, m) {
		t.Error(common.ErrCorrectness)
	}
}
