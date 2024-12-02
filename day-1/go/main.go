package main

import (
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	v1()
	v2()
}

func v1() {
	rawInput, err := os.ReadFile("../input.txt")
	if err != nil {
		return
	}

	var lines = strings.Split(string(rawInput), "\n")

	var distance = 0.0

	var left_array = []int{}
	var right_array = []int{}

	for i := 0; i < len(lines); i++ {
		var number_slices = strings.Split(lines[i], "   ")

		if len(number_slices) < 2 {
			continue
		}

		var left int
		var right int

		left, _ = strconv.Atoi(number_slices[0])
		right, _ = strconv.Atoi(number_slices[1])

		left_array = append(left_array, left)
		right_array = append(right_array, right)

		sort.Ints(left_array)
		sort.Ints(right_array)

	}

	for i := 0; i < len(left_array); i++ {

		var diff = left_array[i] - right_array[i]

		distance += math.Abs(float64(diff))
	}

	fmt.Println("V1:", int32(distance))

}

func v2() {
	rawInput, err := os.ReadFile("../input.txt")
	if err != nil {
		return
	}

	var lines = strings.Split(string(rawInput), "\n")

	var similarity = 0

	var left_map = make(map[int]int)
	var right_map = make(map[int]int)

	for i := 0; i < len(lines); i++ {
		var number_slices = strings.Split(lines[i], "   ")

		if len(number_slices) < 2 {
			continue
		}

		var left int
		var right int

		left, _ = strconv.Atoi(number_slices[0])
		right, _ = strconv.Atoi(number_slices[1])

        left_map[left] = left_map[left] + 1
        right_map[right] = right_map[right] + 1
	}

    fmt.Println(left_map[1])
    for key := range left_map {
        similarity += (key * left_map[key] * right_map[key])
    }

    fmt.Println("V2:", similarity)
}
