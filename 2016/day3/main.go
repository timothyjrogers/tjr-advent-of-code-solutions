package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
	"regexp"
)

const input_file string = "input.txt"

func check_triangle(sides []string) bool {
	side0, _ := strconv.Atoi(sides[0])
	side1, _ := strconv.Atoi(sides[1])
	side2, _ := strconv.Atoi(sides[2])
	return side0 + side1 > side2 && side1 + side2 > side0 && side2 + side0 > side1
}

func partOne(lines []string) {
	valid := 0
	for i := 0; i < len(lines); i++ {
		line := strings.Trim(lines[i], " ")
		pattern := regexp.MustCompile("[[:space:]]+")
		sides := pattern.Split(line, 3)
		if check_triangle(sides) {
			valid = valid + 1
		}
	}
	fmt.Printf("Part 1: %v\n", valid)
}

func partTwo(lines []string) {
	rows := make([][]string, 3)
	for i := 0; i < len(lines); i++ {
		line := strings.Trim(lines[i], " ")
		pattern := regexp.MustCompile("[[:space:]]+")
		sides := pattern.Split(line, 3)
		rows[0] = append(rows[0], sides[0])
		rows[1] = append(rows[1], sides[1])
		rows[2] = append(rows[2], sides[2])
	}
	valid := 0
	for i := 0; i < len(rows[0]); i += 3 {
		for j := 0; j < 3; j++ {
			if check_triangle(rows[j][i:i+3]) {
				valid = valid + 1
			}
		}
	}
	fmt.Printf("Part 2: %v", valid)
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