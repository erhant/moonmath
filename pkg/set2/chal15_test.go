package set2_test

import (
	"cryptopals/internal/constants"
	"cryptopals/pkg/set2"
	"testing"
)

func TestChal15(t *testing.T) {
	t.Skip("todo")
	// valid padding, expect no errors
	{
		_, _, err := set2.UnpadPKCS7([]byte("ICE ICE BABY\x04\x04\x04\x04"))
		if err != nil {
			t.Error(err)
			return
		}
	}
	// invalid padding, expect error
	{
		_, _, err := set2.UnpadPKCS7([]byte("ICE ICE BABY\x05\x05\x05\x05"))
		if err == nil {
			t.Error(constants.ErrExpectedError)
			return
		}
	}
	// invalid padding, expect error
	{
		_, _, err := set2.UnpadPKCS7([]byte("ICE ICE BABY\x01\x02\x03\x04"))
		if err == nil {
			t.Error(constants.ErrExpectedError)
			return
		}
	}
}
