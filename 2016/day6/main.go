package main

import (
	"fmt"
	"os"
	"bufio"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	columns := [8]string{"","","","","",""}
	for _, line := range lines {
		for i, c := range line {
			columns[i] += string(c)
		}
	}
	answer := ""
	for i := 0; i < len(columns); i++ {
		counter := make(map[string]int)
		for _, c := range columns[i] {
			counter[string(c)]++
		}
		maxStr := ""
		maxVal := 0
		for k, v := range counter {
			if v > maxVal {
				maxVal = v
				maxStr = k
			}
		}
		answer += maxStr
	}
	fmt.Printf("Part 1: %v\n", answer)
}

func partTwo(lines []string) {
	columns := [8]string{"","","","","",""}
	for _, line := range lines {
		for i, c := range line {
			columns[i] += string(c)
		}
	}
	answer := ""
	for i := 0; i < len(columns); i++ {
		counter := make(map[string]int)
		for _, c := range columns[i] {
			counter[string(c)]++
		}
		minStr := ""
		minVal := len(lines)
		for k, v := range counter {
			if v < minVal {
				minVal = v
				minStr = k
			}
		}
		answer += minStr
	}
	fmt.Printf("Part 2: %v\n", answer)
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