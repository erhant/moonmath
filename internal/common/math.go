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

func Max(nums ...int) int {
	var max int = math.MinInt
	for _, n := range nums {
		if n > max {
			max = n
		}
	}
	return max
}
