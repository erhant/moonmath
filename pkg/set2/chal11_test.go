package set2_test

import (
	"cryptopals/pkg/set2"
	"os"
	"testing"
)

func TestChal11(t *testing.T) {
	// read file
	pt, err := os.ReadFile("../../res/set2/11.txt")
	if err != nil {
		t.Error(err)
		return
	}

	const numTries = 20
	for i := 0; i < numTries; i++ {
		ct, useECB, err := set2.EncryptionOracle(pt)
		if err != nil {
			t.Error(err)
			return
		}
		detectedECB := set2.DetectionOracle(ct)
		// if useECB != detectedECB {
		t.Log("USED:", useECB, "\tDETECTED:", detectedECB)
		// }
		// t.Log("CT:", ct, "\tECB:", useECB)
	}
}
