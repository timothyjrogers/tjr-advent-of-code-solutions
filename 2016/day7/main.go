package main

import (
	"fmt"
	"os"
	"bufio"
)

const input_file string = "input.txt"

func checkAbba(seq string) bool {
	for i := 0; i < len(seq)-3; i += 1 {
		if seq[i] == seq[i+3] && seq[i+1] == seq[i+2] && seq[i] != seq[i+1] {
			return true
		}
	}
	return false
}

func partOne(lines []string) {
	res := 0
	for _, line := range lines {
		val := ""
		norm_seq := make([]string, 0)
		hypernet_seq := make([]string, 0)
		for _, char := range line {
			c := string(char)
			if c == "[" {
				norm_seq = append(norm_seq, val)
				val = ""
			} else if c == "]" {
				hypernet_seq = append(hypernet_seq, val)
				val = ""
			} else {
				val += c
			}
		}
		if val != "" {
			norm_seq = append(norm_seq, val)
		}
		norm_valid := false
		for _, seq := range norm_seq {
			if checkAbba(seq) {
				norm_valid = true
				break
			}
		}
		hypernet_valid := true
		for _, seq := range hypernet_seq {
			if checkAbba(seq) {
				hypernet_valid = false
				break
			}
		}
		if norm_valid && hypernet_valid {
			res += 1
		}
	}
	fmt.Printf("Part 1: %v\n", res)
}

func getAbas(seq string) []string {
	abas := make([]string, 0)
	for i := 0; i < len(seq)-2; i++ {
		if seq[i] == seq[i+2] && seq[i] != seq[i+1] {
			abas = append(abas, seq[i:i+3])
		}
	}
	return abas
}

func containsBab(seq string, aba string) bool {
	bab := string(aba[1]) + string(aba[0]) + string(aba[1])
	for i := 0; i < len(seq)-2; i++ {
		if seq[i:i+3] == bab {
			return true
		}
	}
	return false
}

func partTwo(lines []string) {
	res := 0
	for _, line := range lines {
		val := ""
		norm_seq := make([]string, 0)
		hypernet_seq := make([]string, 0)
		for _, char := range line {
			c := string(char)
			if c == "[" {
				norm_seq = append(norm_seq, val)
				val = ""
			} else if c == "]" {
				hypernet_seq = append(hypernet_seq, val)
				val = ""
			} else {
				val += c
			}
		}
		if val != "" {
			norm_seq = append(norm_seq, val)
		}

		abas := make([]string, 0)
		for _, seq := range norm_seq {
			abas = append(abas, getAbas(seq)...)
		}
		for _, seq := range hypernet_seq {
			found := false
			for _, aba := range abas {
				if containsBab(seq, aba) {
					found = true
					break
				}
			}
			if found {
				res += 1
				break
			}
		}
	}
	fmt.Printf("Part 2: %v\n", res)
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