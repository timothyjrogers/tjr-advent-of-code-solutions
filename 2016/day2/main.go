package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	var ans []string
	keys := [9]string{"1", "2", "3", "4", "5", "6", "7", "8", "9"}
	key_idx := 4
	for i := 0; i < len(lines); i++ {
		line := lines[i]
		for j := 0; j < len(line); j++ {
			dir := string(line[j])
			if dir == "U" && key_idx > 2 {		
				key_idx = key_idx - 3
			} else if dir == "D" && key_idx < 6 {
				key_idx = key_idx + 3
			} else if dir == "L" && key_idx % 3 != 0 {
				key_idx = key_idx - 1
			} else if dir == "R" && key_idx != 2 && key_idx != 5 && key_idx != 8 {
				key_idx = key_idx + 1
			}
		}
		ans = append(ans, keys[key_idx])
	}
	fmt.Printf("Part 1: %v", strings.Join(ans, ""))	
}

func partTwo(lines []string) {
	key_map := map[string]map[string]string{
		"1": map[string]string{"D": "3", "U": "1", "L": "1", "R": "1"}, 
		"2": map[string]string{"D": "6", "U": "2", "L": "2", "R": "3"}, 
		"3": map[string]string{"D": "7", "U": "1", "L": "2", "R": "4"},
		"4": map[string]string{"D": "8", "U": "4", "L": "3", "R": "4"},
		"5": map[string]string{"D": "5", "U": "5", "L": "5", "R": "6"},
		"6": map[string]string{"D": "A", "U": "2", "L": "5", "R": "7"},
		"7": map[string]string{"D": "B", "U": "3", "L": "6", "R": "8"},
		"8": map[string]string{"D": "C", "U": "4", "L": "7", "R": "9"},
		"9": map[string]string{"D": "9", "U": "9", "L": "8", "R": "9"},
		"A": map[string]string{"D": "A", "U": "6", "L": "A", "R": "B"},
		"B": map[string]string{"D": "D", "U": "7", "L": "A", "R": "C"},
		"C": map[string]string{"D": "C", "U": "8", "L": "B", "R": "C"},
		"D": map[string]string{"D": "D", "U": "B", "L": "D", "R": "D"},        
	}
	var ans []string
	key := "5"
	for i := 0; i < len(lines); i++ {
		line := lines[i]
		for j := 0; j < len(line); j++ {
			dir := string(line[j])
			key = key_map[key][dir]
		}
		ans = append(ans, key)
	}
	fmt.Printf("Part 1: %v", strings.Join(ans, ""))	
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