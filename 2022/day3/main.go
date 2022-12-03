package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	sum := 0
	vals := map[string]int{ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": 6, "g": 7, "h": 8, "i": 9, "j": 10, "k": 11, "l": 12, "m": 13, "n": 14, "o": 15, "p": 16, "q": 17, "r": 18, "s": 19, "t": 20, "u": 21, "v": 22, "w": 23, "x": 24, "y": 25, "z": 26, "A": 27, "B": 28, "C": 29, "D": 30, "E": 31, "F": 32, "G": 33, "H": 34, "I": 35, "J": 36, "K": 37, "L": 38, "M": 39, "N": 40, "O": 41, "P": 42, "Q": 43, "R": 44, "S": 45, "T": 46, "U": 47, "V": 48, "W": 49, "X": 50, "Y": 51, "Z": 52}
	for _, line := range lines {
		duplicate := ""
		for i := 0; i < len(line)/2; i++ {
			for j := len(line)/2; j < len(line); j++ {
				if line[i] == line[j] {
					duplicate = string(line[i])
					break
				}
			}
			if duplicate != "" {
				break
			}
		}
		sum += vals[duplicate]
	}
	fmt.Printf("Part 1: %v\n", sum)
}

func dedupeLine(line string) string {
    keys := make(map[string]bool)
    deduped := make([]string, 0)
    for _, char := range line {
        if _, val := keys[string(char)]; !val {
            keys[string(char)] = true
            deduped = append(deduped, string(char))
        }
    }
    return strings.Join(deduped, "")
}

func partTwo(lines []string) {
	sum := 0
	vals := map[string]int{ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": 6, "g": 7, "h": 8, "i": 9, "j": 10, "k": 11, "l": 12, "m": 13, "n": 14, "o": 15, "p": 16, "q": 17, "r": 18, "s": 19, "t": 20, "u": 21, "v": 22, "w": 23, "x": 24, "y": 25, "z": 26, "A": 27, "B": 28, "C": 29, "D": 30, "E": 31, "F": 32, "G": 33, "H": 34, "I": 35, "J": 36, "K": 37, "L": 38, "M": 39, "N": 40, "O": 41, "P": 42, "Q": 43, "R": 44, "S": 45, "T": 46, "U": 47, "V": 48, "W": 49, "X": 50, "Y": 51, "Z": 52}
	for i := 0; i < len(lines)-2; i+=3 {
		line1 := dedupeLine(lines[i])
		line2 := dedupeLine(lines[i+1])
		line3 := dedupeLine(lines[i+2])
		counter := make(map[string]int)
		for j := 0; j < len(line1); j++ {
			counter[string(line1[j])] += 1
		}
		for j := 0; j < len(line2); j++ {
			counter[string(line2[j])] += 1
		}
		for j := 0; j < len(line3); j++ {
			counter[string(line3[j])] += 1
		}
		for k, v := range counter {
			if v == 3 {
				sum += vals[k]
			}
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