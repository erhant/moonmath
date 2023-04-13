package essentials

import (
	"crypto/rand"
	"math/big"
)

type ElGamalEncryption struct {
	q *big.Int // public | order of the group
	g *big.Int // public | generator
	h *big.Int // public |
	x *big.Int // secret |
}

// Generates keys, λ is the security parameter such as 256 bits.
func NewElGamal(λ int) (*ElGamalEncryption, error) {
	q, err := rand.Prime(rand.Reader, λ)
	if err != nil {
		return nil, err
	}
	g := big.NewInt(2) // anything coprime to q can be generator
	x, err := rand.Int(rand.Reader, q)
	if err != nil {
		return nil, err
	}
	h := new(big.Int).Exp(g, x, q) // g^x mod q
	return &ElGamalEncryption{q, g, h, x}, nil
}

func (eg *ElGamalEncryption) Encrypt(m []byte) (*big.Int, *big.Int, error) {
	// pick random integer in the group (ephemeral key)
	y, err := rand.Int(rand.Reader, eg.q)
	if err != nil {
		return nil, nil, err
	}

	// generate shared secret
	s := new(big.Int).Exp(eg.h, y, eg.q) // h^y mod q

	// first part of ciphertext, for the shared secret
	c1 := new(big.Int).Exp(eg.g, y, eg.q) // g^y mod q

	// second part of ciphertext, for the message
	c2 := new(big.Int).Mod(s.Mul(s, new(big.Int).SetBytes(m)), eg.q) // s * m mod q
	return c1, c2, nil
}

func (eg *ElGamalEncryption) Decrypt(c1 *big.Int, c2 *big.Int) ([]byte, error) {
	// generate shared secret from c1
	s := new(big.Int).Exp(c1, eg.x, eg.q)

	// compute inverse of s in the group
	s_inv := s.ModInverse(s, eg.q)

	// c_2 * s^{-1} mod q, produces m
	pt := new(big.Int).Mod(c2.Mul(c2, s_inv), eg.q)
	return pt.Bytes(), nil
}
