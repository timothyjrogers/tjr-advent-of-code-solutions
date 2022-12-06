package main

import (
	"fmt"
	"os"
	"bufio"
)

const input_file string = "input.txt"

func countUniqueChars(s string) int {
	counter := make(map[string]bool)
	for _, c := range s {
		counter[string(c)] = true
	}
	return len(counter)
}

func partOne(line string) {
	for i := 3; i < len(line); i++ {
		if countUniqueChars(line[i-3:i+1]) == 4 {
			fmt.Printf("Part 1: %v\n", i+1)
			break
		}
	}
}

func partTwo(line string) {
	for i := 13; i < len(line); i++ {
		if countUniqueChars(line[i-13:i+1]) == 14 {
			fmt.Printf("Part 2: %v\n", i+1)
			break
		}
	}
}

func main() {
    file, _ := os.Open(input_file)
    defer file.Close()

    scanner := bufio.NewScanner(file)
	var lines []string
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

	partOne(lines[0])
	partTwo(lines[0])
}