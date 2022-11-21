package constants

import "errors"

var (
	ErrLenMismatch  = errors.New("array: length mismatch")
	ErrWrongResult  = errors.New("result: wrong output")
	ErrWrongKeySize = errors.New("key: wrong size")
)
