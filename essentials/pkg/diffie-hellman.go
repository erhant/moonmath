package essentials

import (
	"crypto/elliptic"
	"crypto/rand"
	"math/big"

	common "cryptogoraphy/pkg/common"
)

// this example uses P256 curve
var curve = elliptic.P256()

// Generates a private key d and a public key Q that is a point in the curve.
func DHKeyGen() (*big.Int, common.CurvePoint, error) {
	// generate private key, a random scalar
	d, err := rand.Int(rand.Reader, curve.Params().N)
	if err != nil {
		return nil, common.CurvePoint{}, err
	}

	// generate public key, a point on the curve
	Qx, Qy := curve.ScalarBaseMult(d.Bytes()) // d * G
	Q := common.CurvePoint{X: Qx, Y: Qy}

	return d, Q, nil
}

// Given d and Q, computes the shared key.
func DHKeyShare(d *big.Int, Q common.CurvePoint) (*big.Int, error) {
	// generate shared key, only care about X coordinate
	xk, _ := curve.ScalarMult(Q.X, Q.Y, d.Bytes()) // d * (Qx, Qy)
	key := new(big.Int).SetBytes(xk.Bytes())

	return key, nil
}
