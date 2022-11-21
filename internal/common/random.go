package common

import (
	cryptoRand "crypto/rand"
	mathRand "math/rand"
)

// Generates random bytes of the given length.
func RandBytes(numBytes int) ([]byte, error) {
	bytes := make([]byte, numBytes)
	_, err := cryptoRand.Read(bytes)
	if err != nil {
		return nil, err
	}
	return bytes, nil
}

// Generates an integer in range [min, max]
func RandInteger(min int, max int) int {
	return mathRand.Intn(max-min+1) + min
}

// Generates a boolean
func RandBool() bool {
	return mathRand.Intn(2) == 0
}
