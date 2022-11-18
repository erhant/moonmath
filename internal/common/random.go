package common

import "crypto/rand"

// Generates random bytes of the given length.
func RandBytes(numBytes int) ([]byte, error) {
	bytes := make([]byte, numBytes)
	_, err := rand.Read(bytes)
	if err != nil {
		return nil, err
	}
	return bytes, nil
}
