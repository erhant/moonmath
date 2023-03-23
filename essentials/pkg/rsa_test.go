package essentials_test

import (
	"crypto/rand"
	"crypto/sha256"
	"math/big"
	"testing"

	common "cryptogoraphy/pkg/common"
)

const bits = 1024     // security parameter
const publicE = 65537 // publicly known exponent

func TestRSA(t *testing.T) {
	// generate public and private keys
	p, err := rand.Prime(rand.Reader, bits)
	if err != nil {
		t.Error(err)
	}
	q, err := rand.Prime(rand.Reader, bits)
	if err != nil {
		t.Error(err)
	}
	n := new(big.Int).Mul(p, q)
	phi := new(big.Int).Mul(new(big.Int).Sub(p, big.NewInt(1)), new(big.Int).Sub(q, big.NewInt(1)))
	e := big.NewInt(publicE)
	d := new(big.Int).ModInverse(e, phi)

	// original message
	message := []byte("herro worl")

	// generate a hash of the message
	hash := sha256.Sum256(message)

	// encrypt the hash using the public key
	encryptedHash := new(big.Int).Exp(new(big.Int).SetBytes(hash[:]), e, n)

	// decrypt the hash using the private key
	decryptedHash := new(big.Int).Exp(encryptedHash, d, n)

	// verify that the decrypted hash matches the original hash
	if string(decryptedHash.Bytes()) != string(hash[:]) {
		t.Error(common.ErrCorrectness)
	}
}
