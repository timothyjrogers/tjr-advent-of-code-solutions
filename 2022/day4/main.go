package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
)

const input_file string = "input.txt"

func splitRange(r string) [4]int {
	ranges := strings.Split(r, ",")
	strs := append(make([]string, 0), strings.Split(ranges[0], "-")...)
	strs = append(strs, strings.Split(ranges[1], "-")...)
	var ret [4]int
	for i := 0; i < 4; i++ {
		v, _ := strconv.Atoi(strs[i])
		ret[i] = v
	}
	return ret
}

func partOne(lines []string) {
	sum := 0
	for _, line := range lines {
		ranges := splitRange(line)
		if ((ranges[2] >= ranges[0] && ranges[2] <= ranges[1]) && (ranges[3] >= ranges[0] && ranges[3] <= ranges[1])) || ((ranges[0] >= ranges[2] && ranges[0] <= ranges[3]) && (ranges[1] >= ranges[2] && ranges[1] <= ranges[3])) {
			sum += 1
		}
	}
	fmt.Printf("Part 1: %v\n", sum)
}

func partTwo(lines []string) {
	sum := 0
	for _, line := range lines {
		ranges := splitRange(line)
		if (ranges[2] >= ranges[0] && ranges[2] <= ranges[1]) || (ranges[3] >= ranges[0] && ranges[3] <= ranges[1]) || ((ranges[2] >= ranges[0] && ranges[2] <= ranges[1]) && (ranges[3] >= ranges[0] && ranges[3] <= ranges[1])) || ((ranges[0] >= ranges[2] && ranges[0] <= ranges[3]) && (ranges[1] >= ranges[2] && ranges[1] <= ranges[3])) {
			sum += 1
		}
	}
	fmt.Printf("Part 2: %v\n", sum)
}

func main() {
    file, _ := os.Open(input_file)
    defer file.Close()

    scanner := bufio.NewScanner(file)
	var lines []string
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

	partOne(lines)
	partTwo(lines)
}