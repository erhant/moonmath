package set1_test

import (
	"bufio"
	"cryptopals/internal/common"
	"os"
	"testing"
)

func TestChal8(t *testing.T) {
	// open file
	file, err := os.Open("../../res/set1/8.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer file.Close()

	// read line by line
	score := 0
	const size = 16
	var ans []byte
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		// no need to decode hex for this challenge
		ct := scanner.Bytes()

		// count repeating blocks
		s := common.RepeatingBlocks(ct, size)

		// update score
		if s > score {
			ans = make([]byte, len(ct))
			copy(ans, ct)
			score = s
			// fmt.Println("Better:", string(ans), "\nScore:", score)
		}
	}

	if err := scanner.Err(); err != nil {
		t.Error(err)
		return
	}

	expected := "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a"
	if string(ans) != expected {
		t.Errorf("Wrong output.\nHave: %s\nNeed: %s\n", string(ans), expected)
		return
	}
}
