package set1_test

import (
	"cryptopals/internal/common"
	"testing"
)

func TestChal6(t *testing.T) {
	// test levenstein edit distance
	{
		s1 := []byte("this is a test")
		s2 := []byte("wokka wokka!!!")
		dist := common.LevensteinEditDistance(s1, s2)
		expectedDist := 37
		if dist != expectedDist {
			t.Errorf("Wrong output.\nHave: %d\nNeed: %d\n", dist, expectedDist)
		}
	}

}
