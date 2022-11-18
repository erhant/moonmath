package set1

import "cryptopals/internal/constants"

func XOR(a, b []byte) ([]byte, error) {
	if len(a) != len(b) {
		return nil, constants.ErrLenMismatch
	}

	ans := make([]byte, len(a))
	for i := 0; i < len(a); i++ {
		ans[i] = a[i] ^ b[i]
	}
	return ans, nil
}
