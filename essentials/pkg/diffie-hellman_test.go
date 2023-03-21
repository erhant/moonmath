package essentials_test

import (
	"crypto/elliptic"
	"crypto/rand"
	"math/big"
	"testing"
)

// this example uses P256 curve
var curve = elliptic.P256()

func TestDiffieHellman(t *testing.T) {
	// generate private and public keys for Alice
	alicePrivateKey, alicePublicKey, err := generateKeys()
	if err != nil {
		t.Error(err)
	}

	// generate private and public keys for Bob
	bobPrivateKey, bobPublicKey, err := generateKeys()
	if err != nil {
		t.Error(err)
	}

	// Alice and Bob exchange public keys
	sharedKey1, err := generateSharedKey(alicePrivateKey, bobPublicKey)
	if err != nil {
		t.Error(err)
	}
	sharedKey2, err := generateSharedKey(bobPrivateKey, alicePublicKey)
	if err != nil {
		t.Error(err)
	}

	// compare shared keys
	if sharedKey1.Cmp(sharedKey2) != 0 {
		t.Error(ErrCorrectness)
	}
}

func generateKeys() (*big.Int, CurvePoint, error) {
	// generate private key
	privateKey, err := rand.Int(rand.Reader, curve.Params().N)
	if err != nil {
		return nil, CurvePoint{}, err
	}

	// generate public key
	publicKeyX, publicKeyY := curve.ScalarBaseMult(privateKey.Bytes())
	publicKey := CurvePoint{X: publicKeyX, Y: publicKeyY}

	return privateKey, publicKey, nil
}

func generateSharedKey(privateKey *big.Int, publicKey CurvePoint) (*big.Int, error) {
	// generate shared key, only care about X coordinate
	sharedKeyX, _ := curve.ScalarMult(publicKey.X, publicKey.Y, privateKey.Bytes())
	sharedKey := new(big.Int).SetBytes(sharedKeyX.Bytes())

	return sharedKey, nil
}
