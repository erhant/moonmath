package essentials_test

import (
	"crypto/rand"
	"math/big"
	"testing"

	common "cryptogoraphy/pkg/common"
)

func TestPedersenCommitment(t *testing.T) {
	// prime modulus
	p, success := new(big.Int).SetString("115792089237316195423570985008687907853269984665640564039457584007913129639319", 10)
	if !success {
		t.Fatal("Could not create prime modulus")
	}
	// generator
	g, success := new(big.Int).SetString("3", 10)
	if !success {
		t.Fatal("Could not create generator")
	}

	// generate a random value to commit to
	value := big.NewInt(1234)

	// generate a random blinding factor
	r, err := rand.Int(rand.Reader, p)
	if err != nil {
		t.Fatal("Could not create blinding factor")
	}

	// calculate the commitment value
	commitment := new(big.Int).Exp(g, value, p)
	commitment.Mul(commitment, new(big.Int).Exp(r, p, p))
	commitment.Mod(commitment, p)

	// verify the commitment by checking that
	// g^value * r^p == commitment
	verification := new(big.Int).Exp(g, value, p)
	verification.Mul(verification, new(big.Int).Exp(r, p, p))
	verification.Mod(verification, p)

	if commitment.Cmp(verification) != 0 {
		t.Error(common.ErrCorrectness)
	}
}
