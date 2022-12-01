package main

import (
	"fmt"
	"os"
	"bufio"
	"strconv"
	"sort"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	max := 0
	cur := 0
	for _, line := range lines {
		if len(line) == 0 {
			if cur > max {
				max = cur
			}
			cur = 0
			continue
		}
		calories, _ := strconv.Atoi(line)
		cur += calories
	}
	if cur > max {
		max = cur
	}
	fmt.Printf("Part 1: %v\n", max)
}

func partTwo(lines []string) {
	elves := make([]int, 0)
	cur := 0
	for _, line := range lines {
		if len(line) == 0 {
			elves = append(elves, cur)
			cur = 0
			continue
		}
		calories, _ := strconv.Atoi(line)
		cur += calories
	}
	elves = append(elves, cur)
	sort.Sort(sort.Reverse(sort.IntSlice(elves)))
	sum := elves[0] + elves[1] + elves[2]
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