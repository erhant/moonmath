package set1

func RepeatingKeyXOR(pt []byte, key []byte) []byte {
	ct := make([]byte, len(pt))
	keylen := len(key)
	for i := 0; i < len(pt); i++ {
		ct[i] = pt[i] ^ key[i%keylen]
	}
	return ct
}
