package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
)

const input_file string = "input.txt"

func partOne(lines []string) {

	loss := 0
	draw := 3
	win := 6
	scores := map[string]int{"A": 1, "X": 1, "B": 2, "Y": 2, "C": 3, "Z": 3}
	matchups := map[string][3]string{
		"A": [3]string{"Y", "X", "Z"},
		"B": [3]string{"Z", "Y", "X"},
		"C": [3]string{"X", "Z", "Y"},
	}

	sum := 0
	for _, line := range lines {
		parts := strings.Split(line, " ")
		if parts[1] == matchups[parts[0]][0] {
			sum += win + scores[parts[1]]
		} else if parts[1] == matchups[parts[0]][1] {
			sum += draw + scores[parts[1]]
		} else if parts[1] == matchups[parts[0]][2] {
			sum += loss + scores[parts[1]]
		}
	}
	fmt.Printf("Part 1: %v\n", sum)
}

func partTwo(lines []string) {

	loss := 0
	draw := 3
	win := 6
	scores := map[string]int{"A": 1, "X": 1, "B": 2, "Y": 2, "C": 3, "Z": 3}
	matchups := map[string][3]string{
		"A": [3]string{"Y", "X", "Z"},
		"B": [3]string{"Z", "Y", "X"},
		"C": [3]string{"X", "Z", "Y"},
	}

	sum := 0
	for _, line := range lines {
		parts := strings.Split(line, " ")
		if parts[1] == "X" {
			//lose
			sum += loss + scores[matchups[parts[0]][2]]
		} else if parts[1] == "Y" {
			//draw
			sum += draw + scores[matchups[parts[0]][1]]

		} else if parts[1] == "Z" {
			//win
			sum += win + scores[matchups[parts[0]][0]]
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