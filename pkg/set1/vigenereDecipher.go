package set1

func VigenereDecipher(ct []byte) {
	const KEYSIZE_MIN = 2
	const KEYSIZE_MAX = 40

	// first, find the keysize via edit distance of consecutive blocks
	// ...

	// then, break the ciphertext into blocks of keysize length and take every KEYSIZE block separately.
	// for these blocks, they are encrypted with a single key, which we know how to break from previous challenges
	// after finding each byte for the respective blocks, the key is just these bytes together
	// ...
}
