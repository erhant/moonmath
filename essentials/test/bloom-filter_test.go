package essentials_test

import (
	"testing"

	essentials "cryptogoraphy/pkg"
	common "cryptogoraphy/pkg/common"
)

func TestBloomFilter(t *testing.T) {
	bf, err := essentials.NewBloomFilter(32, 4)
	if err != nil {
		t.Fatal(err)
	}

	// add some data to the Bloom Filter
	bf.Add([]byte("hello"))
	bf.Add([]byte("world"))

	// check if some items are possibly in the Bloom Filter
	if !(bf.PossiblyContains([]byte("hello")) && bf.PossiblyContains([]byte("world"))) {
		t.Error(common.ErrCorrectness)
	}

	if bf.PossiblyContains([]byte("foo")) || bf.PossiblyContains([]byte("bar")) {
		t.Error(common.ErrCorrectness)
	}
}
