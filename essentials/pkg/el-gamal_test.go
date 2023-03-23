package essentials_test

import (
	"bytes"
	"crypto/rand"
	"math/big"
	"testing"

	common "cryptogoraphy/pkg/common"
)

func TestElGamal(t *testing.T) {
	// key generation
	p, err := rand.Prime(rand.Reader, 256)
	if err != nil {
		t.Error(err)
	}
	g := big.NewInt(2)
	x, err := rand.Int(rand.Reader, p)
	if err != nil {
		t.Error(err)
	}
	y := new(big.Int).Exp(g, x, p)

	// encryption
	m := []byte("very secret")
	k, err := rand.Int(rand.Reader, p)
	if err != nil {
		t.Error(err)
	}
	a := new(big.Int).Exp(g, k, p)
	b := new(big.Int).Exp(y, k, p)
	b.Mul(b, new(big.Int).SetBytes(m))
	b.Mod(b, p)

	// decryption
	s := new(big.Int).Exp(a, x, p)
	s.ModInverse(s, p)
	b.Mul(b, s)
	b.Mod(b, p)

	// fmt.Printf("encryption (a, b): (%s, %s)\n", a, b)
	if !bytes.Equal(b.Bytes(), m) {
		t.Error(common.ErrCorrectness)
	}

}
