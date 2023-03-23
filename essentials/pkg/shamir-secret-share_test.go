package essentials_test

import (
	"crypto/rand"
	"fmt"
	"math/big"
	"testing"
)

func TestShamirSecretShare(t *testing.T) {
	// The secret value we want to share
	secret := big.NewInt(123456789)

	// Number of shares to create
	n := 5

	// Minimum number of shares required to reconstruct secret
	k := 3

	// Generate random coefficients for the polynomial
	coeffs := make([]*big.Int, k)
	coeffs[0] = secret
	for i := 1; i < k; i++ {
		coeff, err := rand.Int(rand.Reader, big.NewInt(256))
		if err != nil {
			panic(err)
		}
		coeffs[i] = coeff
	}

	// Generate shares by evaluating the polynomial at different x values
	shares := make([][]*big.Int, n)
	for i := 0; i < n; i++ {
		x := big.NewInt(int64(i + 1))
		shares[i] = make([]*big.Int, 2)
		shares[i][0] = x
		shares[i][1] = evaluatePolynomial(coeffs, x)
	}

	// Print out the shares
	fmt.Println("Shares:")
	for _, share := range shares {
		fmt.Printf("(%v, %v)\n", share[0], share[1])
	}

	// Reconstruct the secret using k shares
	reconstructed := reconstructSecret(shares[:k])
	fmt.Printf("Reconstructed secret: %v\n", reconstructed)
}

// Evaluate the polynomial defined by coeffs at the given x value
func evaluatePolynomial(coeffs []*big.Int, x *big.Int) *big.Int {
	result := big.NewInt(0)
	for i := len(coeffs) - 1; i >= 0; i-- {
		result.Mul(result, x)
		result.Add(result, coeffs[i])
	}
	return result
}

// Reconstruct the secret using the given shares
func reconstructSecret(shares [][]*big.Int) *big.Int {
	result := big.NewInt(0)
	for i := 0; i < len(shares); i++ {
		// Calculate the Lagrange coefficient for this share
		lagrange := big.NewInt(1)
		for j := 0; j < len(shares); j++ {
			if i == j {
				continue
			}
			numerator := big.NewInt(0).Sub(shares[j][0], big.NewInt(0).Set(shares[i][0]))
			denominator := big.NewInt(0).Sub(shares[j][0], big.NewInt(0).Set(shares[i][0]))
			lagrange.Mul(lagrange, numerator)
			lagrange.Mod(lagrange, prime)
			lagrange.Mul(lagrange, denominator.ModInverse(denominator, prime))
			lagrange.Mod(lagrange, prime)
		}

		// Multiply this share's value by its Lagrange coefficient and add to the result
		value := shares[i][1]
		value.Mul(value, lagrange)
		value.Mod(value, prime)
		result.Add(result, value)
	}
	return result
}

// A large prime number to use for calculations
var prime, _ = big.NewInt(0).SetString("16798108731015832284940804142231733909759579603404752749028378864165570215949", 10)
