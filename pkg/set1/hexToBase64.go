package set1

import (
	"encoding/base64"
	"encoding/hex"
)

// Converts a hex byte array to base64. The input array is decoded within this function.
func HexToBase64(src []byte) ([]byte, error) {
	// decode hex
	srcD := make([]byte, hex.DecodedLen(len(src)))
	_, err := hex.Decode(srcD, src)
	if err != nil {
		return nil, err
	}

	// encode base64
	b64E := make([]byte, base64.StdEncoding.EncodedLen(len(srcD)))
	base64.StdEncoding.Encode(b64E, srcD)

	return b64E, nil
}
