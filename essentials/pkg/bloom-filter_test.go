package essentials_test

import (
	"errors"
	"hash/adler32"
	"hash/crc32"
	"hash/fnv"
	"testing"

	common "cryptogoraphy/pkg/common"
)

type BloomFilter struct {
	bitmap []bool
	hashes []func([]byte) uint32
}

// Create a new Bloom Filter of size m bits, with k hash functions
func NewBloomFilter(m, k uint) (*BloomFilter, error) {
	i := 0
	bitmap := make([]bool, m)
	hashes := make([]func([]byte) uint32, k)

	switch k - 1 {
	default:
		return nil, errors.New("Too many hash functions")
	case 5:
		hashes[i] = func(data []byte) uint32 {
			h := fnv.New32()
			h.Write(data)
			return h.Sum32() % uint32(m)
		}
		i++
		fallthrough
	case 4:
		hashes[i] = func(data []byte) uint32 {
			h := fnv.New32a()
			h.Write(data)
			return h.Sum32() % uint32(m)
		}
		i++
		fallthrough
	case 3:
		hashes[i] = func(data []byte) uint32 {
			h := fnv.New64()
			h.Write(data)
			return uint32(h.Sum64()) % uint32(m)
		}
		i++
		fallthrough
	case 2:
		hashes[i] = func(data []byte) uint32 {
			h := fnv.New64a()
			h.Write(data)
			return uint32(h.Sum64()) % uint32(m)
		}
		i++
		fallthrough
	case 1:
		hashes[i] = func(data []byte) uint32 {
			h := adler32.New()
			h.Write(data)
			return h.Sum32() % uint32(m)
		}
		i++
		fallthrough
	case 0:
		hashes[i] = func(data []byte) uint32 {
			h := crc32.NewIEEE()
			h.Write(data)
			return h.Sum32() % uint32(m)
		}
		i++
	}

	return &BloomFilter{bitmap, hashes}, nil
}

// adds an item to the Bloom Filter
func (bf *BloomFilter) Add(data []byte) {
	for _, hash := range bf.hashes {
		h := hash(data)
		bf.bitmap[h] = true
	}
}

// checks if an item is possibly in the Bloom Filter
func (bf *BloomFilter) PossiblyContains(data []byte) bool {
	res := true
	for _, hash := range bf.hashes {
		res = res && bf.bitmap[hash(data)]
	}
	return res
}

func TestBloomFilter(t *testing.T) {
	// create a new Bloom Filter with 10 bits and 2 hash functions
	bf, err := NewBloomFilter(10, 2)
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
