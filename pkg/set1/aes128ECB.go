package set1

import (
	"crypto/aes"
	"cryptopals/internal/constants"
)

const size = 16 // 128 bits = 16 bytes

func AES128ECBEncrypt(pt, key []byte) ([]byte, int, error) {
	// create block cipher
	if len(key) != size {
		return nil, 0, constants.ErrWrongKeySize
	}
	cipher, err := aes.NewCipher(key)
	if err != nil {
		return nil, 0, err
	}

	// make sure input length is multiple of block length
	padding := size - (len(pt) % size)
	if padding > 0 {
		pad := make([]byte, padding)
		// not really caring about PKCS for this challenge
		pt = append(pt, pad...)
	}

	// encrypt
	ct := make([]byte, len(pt)+padding)
	for bs, be := 0, size; be <= len(pt); bs, be = bs+size, be+size {
		cipher.Encrypt(ct[bs:be], pt[bs:be])
	}

	return ct[:len(pt)-padding], padding, nil
}

func AES128ECBDecrypt(ct, key []byte) ([]byte, int, error) {
	// create block cipher
	if len(key) != size {
		return nil, 0, constants.ErrWrongKeySize
	}
	cipher, err := aes.NewCipher(key)
	if err != nil {
		return nil, 0, err
	}

	// make sure input length is multiple of block length
	padding := size - (len(ct) % size)
	if padding > 0 {
		pad := make([]byte, padding)
		// not really caring about PKCS for this challenge
		ct = append(ct, pad...)
	}

	// decrypt
	pt := make([]byte, len(ct))
	for bs, be := 0, size; be <= len(ct); bs, be = bs+size, be+size {
		cipher.Decrypt(pt[bs:be], ct[bs:be])
	}

	return pt[:len(ct)-padding], padding, nil
}
