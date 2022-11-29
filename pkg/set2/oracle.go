package set2

import (
	"cryptopals/internal/common"
	"cryptopals/pkg/set1"
	"fmt"
)

// This function creates a random 128-bit key,
// appends & preprends some stuff to the plaintext
// and then encrypts the entire thing. Half of the time,
// it will use ECB mode, otherwise CBC.
func EncryptionOracle(pt []byte) ([]byte, bool, error) {
	const size = 16
	// create random 16-byte key
	key, err := common.RandBytes(size)
	if err != nil {
		return nil, false, err
	}

	// prepend
	if prependBytes, err := common.RandBytes(common.RandInteger(5, 10)); err != nil {
		return nil, false, err
	} else {
		pt = append(prependBytes, pt...)
	}

	// append
	if appendBytes, err := common.RandBytes(common.RandInteger(5, 10)); err != nil {
		return nil, false, err
	} else {
		pt = append(pt, appendBytes...)
	}

	// encrypt
	var ct []byte
	useECB := common.RandBool()
	if useECB {
		// encrypt with ECB
		ct, _, err = set1.AES128ECBEncrypt(pt, key)
		if err != nil {
			return nil, false, err
		}
	} else {
		// generate random iv
		iv, err := common.RandBytes(size)
		if err != nil {
			return nil, false, err
		}
		// encrypt with CBC
		ct, _, err = AES128CBCEncrypt(pt, iv, key)
		if err != nil {
			return nil, false, err
		}
	}

	return ct, useECB, nil
}

// Given a ciphertext, this function returns true if
// the given ciphertext was encrypted using ECB. Returning
// false means that ciphertext was encrypted using CBC.
func DetectionOracle(ct []byte) bool {
	const size = 16
	// similar to previous challenge, try to check for repeating blocks
	s := common.RepeatingBlocks(ct, size)
	fmt.Println("Repeating:", s)
	return s > 0
}
