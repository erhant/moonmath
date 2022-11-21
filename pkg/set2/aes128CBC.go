package set2

import (
	"crypto/aes"
	"cryptopals/internal/constants"
	"cryptopals/pkg/set1"
)

func AES128CBCEncrypt(pt, iv, key []byte) ([]byte, int, error) {
	const size = 16
	// create block cipher
	if len(key) != size {
		return nil, 0, constants.ErrWrongKeySize
	}
	if len(iv) != size {
		return nil, 0, constants.ErrWrongIVSize
	}
	cipher, err := aes.NewCipher(key)
	if err != nil {
		return nil, 0, err
	}

	// make sure input length is multiple of block length
	// not really caring about PKCS for this challenge
	padding := uint8(size - (len(pt) % size))
	for i := padding; i > 0; i-- {
		pt = append(pt, padding)
	}

	// encrypt
	ct := make([]byte, len(pt))
	prev := make([]byte, size)
	copy(prev, iv)
	for bs, be := 0, size; be <= len(pt); bs, be = bs+size, be+size {
		// xor with prev
		xor, err := set1.XOR(pt[bs:be], prev)
		if err != nil {
			return nil, 0, err
		}
		// encrypt
		cipher.Encrypt(ct[bs:be], xor)
		// update prev
		copy(prev, ct[bs:be])

	}

	return ct[:len(pt)-int(padding)], int(padding), nil
}

func AES128CBCDecrypt(ct, iv, key []byte) ([]byte, int, error) {
	const size = 16
	// create block cipher
	if len(key) != size {
		return nil, 0, constants.ErrWrongKeySize
	}
	if len(iv) != size {
		return nil, 0, constants.ErrWrongIVSize
	}
	cipher, err := aes.NewCipher(key)
	if err != nil {
		return nil, 0, err
	}

	// make sure input length is multiple of block length
	// not really caring about PKCS for this challenge
	padding := size - (len(ct) % size)
	if padding > 0 {
		pad := make([]byte, padding)
		ct = append(ct, pad...)
	}

	// decrypt
	pt := make([]byte, len(ct))
	prev := make([]byte, size)
	copy(prev, iv)
	for bs, be := 0, size; be <= len(ct); bs, be = bs+size, be+size {
		// decrypt
		cipher.Decrypt(pt[bs:be], ct[bs:be])
		// xor with prev
		xor, err := set1.XOR(pt[bs:be], prev)
		if err != nil {
			return nil, 0, err
		}
		copy(pt[bs:be], xor)
		// update prev
		copy(prev, ct[bs:be])
	}

	return pt[:len(ct)-padding], padding, nil
}
