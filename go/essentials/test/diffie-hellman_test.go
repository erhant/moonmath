package essentials_test

import (
	"testing"

	essentials "cryptogoraphy/pkg"
	common "cryptogoraphy/pkg/common"
)

func TestDiffieHellman(t *testing.T) {
	// generate private and public keys for Alice
	alicePrivateKey, alicePublicKey, err := essentials.DHKeyGen()
	if err != nil {
		t.Error(err)
	}

	// generate private and public keys for Bob
	bobPrivateKey, bobPublicKey, err := essentials.DHKeyGen()
	if err != nil {
		t.Error(err)
	}

	// Alice and Bob exchange public keys
	sharedKey1, err := essentials.DHKeyShare(alicePrivateKey, bobPublicKey)
	if err != nil {
		t.Error(err)
	}
	sharedKey2, err := essentials.DHKeyShare(bobPrivateKey, alicePublicKey)
	if err != nil {
		t.Error(err)
	}

	// compare shared keys
	if sharedKey1.Cmp(sharedKey2) != 0 {
		t.Error(common.ErrCorrectness)
	}
}
