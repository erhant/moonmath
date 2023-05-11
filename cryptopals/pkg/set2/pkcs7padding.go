package set2

import (
	"cryptopals/internal/constants"
)

func PadPKCS7(buf []byte, size int) []byte {
	// calculate amount of padding required
	padding := byte(size - (len(buf) % size))
	if padding == 0 {
		padding = 16 // pad an entire block
	}
	// pad
	for i := padding; i > 0; i-- {
		buf = append(buf, padding)
	}
	return buf
}

// Returns the unpadded buffer, along with the padding amount.
func UnpadPKCS7(buf []byte) ([]byte, int, error) {
	// edge case
	len_padded := len(buf)
	if len_padded == 0 {
		return buf, 0, constants.ErrEmptyArray
	}

	// the last byte b should appear b many times
	b := buf[len_padded-1]
	len_unpadded := len_padded - int(b)
	for i := len_padded - 1; i > len_unpadded; i-- {
		if buf[i] != b || i < 0 {
			return buf, int(b), constants.ErrInvalidPKCS
		}
	}
	// log.Println(buf[:len_unpadded])
	return buf[:len_unpadded], int(b), nil
}
