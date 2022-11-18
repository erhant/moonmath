package common

import "math"

func Min(nums ...int) int {
	var min int = math.MaxInt
	for _, n := range nums {
		if n < min {
			min = n
		}
	}
	return min
}
