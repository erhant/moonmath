package common

import (
	"math/big"
)

// A point struct of two big int pointers (X, Y)
type CurvePoint struct {
	X *big.Int
	Y *big.Int
}
