package set1_test

import (
	"bufio"
	"cryptopals/internal/constants"
	"cryptopals/pkg/set1"
	"encoding/hex"
	"math"
	"os"
	"testing"
)

func TestChal4(t *testing.T) {
	t.Skip("skipping test 4: too long")
	// open file
	file, err := os.Open("../../res/set1/4.txt")
	if err != nil {
		t.Error(err)
	}
	defer file.Close()

	// read line by line
	var score float32 = math.MaxFloat32
	var ans []byte
	var key byte
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		ct := scanner.Bytes()

		// decode hex
		len := hex.DecodedLen(len(ct))
		ctDec := make([]byte, len)
		_, err := hex.Decode(ctDec, ct)
		if err != nil {
			t.Error(err)
		}

		// crack line
		pt, k, s, err := set1.SingleByteXORDecipher(ctDec)
		if err != nil {
			t.Error(err)
		}

		// update score
		if s <= score {
			ans = pt
			key = k
			score = s
			// fmt.Println("Better:", string(ans), "\nKey:", key, "\nScore:", score)
		}
	}

	if err := scanner.Err(); err != nil {
		t.Error(err)
	}

	// key 21 --> "nOWTHATTHEPARTYISJUMPING*" (score: 0.7060677)
	if string(ans) != "nOWTHATTHEPARTYISJUMPING" && key != 21 {
		t.Error(constants.ErrWrongResult)
	}
}
