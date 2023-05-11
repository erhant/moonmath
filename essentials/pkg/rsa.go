package essentials

import (
	"crypto/rand"
	"math/big"
)

type RSAEncryption struct {
	p *big.Int // randomly chosen prime
	q *big.Int // randomly chosen prime
	d *big.Int // private key
	n *big.Int // public key, n = pq
	e *big.Int // public, constant
}

// Generate keys, security parameter is given by the user.
// Public exponent is constant.
func NewRSA(λ int) (*RSAEncryption, error) {
	p, err := rand.Prime(rand.Reader, λ)
	if err != nil {
		return nil, err
	}
	q, err := rand.Prime(rand.Reader, λ)
	if err != nil {
		return nil, err
	}
	n := new(big.Int).Mul(p, q)
	phi := new(big.Int).Mul(new(big.Int).Sub(p, big.NewInt(1)), new(big.Int).Sub(q, big.NewInt(1)))
	e := big.NewInt(65537)
	d := new(big.Int).ModInverse(e, phi)

	return &RSAEncryption{
		p, q, d, n, e,
	}, nil
}

func (rsa *RSAEncryption) Encrypt(pt []byte) *big.Int {
	ct := new(big.Int).Exp(new(big.Int).SetBytes(pt), rsa.e, rsa.n)
	return ct
}

func (rsa *RSAEncryption) Decrypt(ct *big.Int) []byte {
	pt := new(big.Int).Exp(ct, rsa.d, rsa.n)
	return pt.Bytes()
}
