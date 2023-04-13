package set2_test

import (
	"bytes"
	"cryptopals/internal/constants"
	"cryptopals/pkg/set2"
	"testing"
)

func TestChal9(t *testing.T) {
	buf := []byte("YELLOW SUBMARINE")
	paddedBuf := set2.PadPKCS7(buf, 20)                     // pad to 20 bytes
	unpaddedBuf, padding, err := set2.UnpadPKCS7(paddedBuf) // unpad
	if err != nil {
		t.Error(err)
		return
	}
	if padding != 4 || !bytes.Equal(buf, unpaddedBuf) {
		t.Error(constants.ErrWrongResult)
		return
	}
}
