package set1_test

import (
	"bufio"
	"cryptopals/pkg/set1"
	"encoding/hex"
	"math"
	"os"
	"testing"
)

func TestChal4(t *testing.T) {
	t.Skip("skip: test is a bit long")
	// open file
	file, err := os.Open("../../res/set1/4.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer file.Close()

	// read line by line
	var score float32 = math.MaxFloat32
	var ans []byte
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		ct := scanner.Bytes()

		// decode hex
		len := hex.DecodedLen(len(ct))
		ctDec := make([]byte, len)
		_, err := hex.Decode(ctDec, ct)
		if err != nil {
			t.Error(err)
			return
		}

		// crack line
		pt, _, s, err := set1.SingleByteXORDecipher(ctDec)
		if err != nil {
			t.Error(err)
			return
		}

		// update score
		if s <= score {
			ans = pt
			score = s
			// fmt.Println("Better:", string(ans), "\nKey:", key, "\nScore:", score)
		}
	}

	if err := scanner.Err(); err != nil {
		t.Error(err)
		return
	}

	expected := "Now that the party is jumping\n"
	if string(ans) != expected {
		t.Errorf("Wrong output.\nHave: %s\nNeed: %s\n", string(ans), expected)
		return
	}
}
