package constants

import "errors"

var (
	ErrLenMismatch   = errors.New("array: length mismatch")
	ErrEmptyArray    = errors.New("array: empty")
	ErrInvalidPKCS   = errors.New("pkcs: invalid padding")
	ErrWrongResult   = errors.New("result: wrong output")
	ErrWrongKeySize  = errors.New("aes: wrong key size")
	ErrWrongIVSize   = errors.New("aes: wrong IV size")
	ErrExpectedError = errors.New("error: expected error")
)
