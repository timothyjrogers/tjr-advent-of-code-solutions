package main

import (
	"fmt"
	"os"
	"bufio"
	"regexp"
	"strconv"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	header := make([]string, 0)
	start := 0
	for idx, line := range lines {
		if line == "" {
			start = idx + 1
			break
		}
		header = append(header, line)
	}
	stacks := make([][]string, 0)
	for i, v := range header[len(header)-1] {
		if string(v) != " " {
			stacks = append(stacks, make([]string, 0))
			for j := len(header)-2; j >= 0; j-- {
				if string(header[j][i]) != " " {
					stacks[len(stacks)-1] = append(stacks[len(stacks)-1], string(header[j][i]))
				}
			}
		}
	}

	digitsRegex := regexp.MustCompile("[0-9]+")
	for _, line := range lines[start:] {
		vals := digitsRegex.FindAllString(line, -1)
		count, _ := strconv.Atoi(vals[0])
		from, _ := strconv.Atoi(vals[1])
		to, _ := strconv.Atoi(vals[2])
		for i := 0; i < count; i++ {
			if len(stacks[from-1]) > 0 {
				stacks[to-1] = append(stacks[to-1], stacks[from-1][len(stacks[from-1])-1])
				stacks[from-1] = stacks[from-1][:len(stacks[from-1])-1]
			}
		}
	}
	res := ""
	for _, v := range stacks {
		res += v[len(v)-1]
	}
	fmt.Printf("Part 1: %v\n", res)
}

func partTwo(lines []string) {
	header := make([]string, 0)
	start := 0
	for idx, line := range lines {
		if line == "" {
			start = idx + 1
			break
		}
		header = append(header, line)
	}
	stacks := make([][]string, 0)
	for i, v := range header[len(header)-1] {
		if string(v) != " " {
			stacks = append(stacks, make([]string, 0))
			for j := len(header)-2; j >= 0; j-- {
				if string(header[j][i]) != " " {
					stacks[len(stacks)-1] = append(stacks[len(stacks)-1], string(header[j][i]))
				}
			}
		}
	}

	digitsRegex := regexp.MustCompile("[0-9]+")
	for _, line := range lines[start:] {
		vals := digitsRegex.FindAllString(line, -1)
		count, _ := strconv.Atoi(vals[0])
		from, _ := strconv.Atoi(vals[1])
		to, _ := strconv.Atoi(vals[2])
		if len(stacks[from-1]) > count {
			stacks[to-1] = append(stacks[to-1], stacks[from-1][len(stacks[from-1])-count:]...)
			stacks[from-1] = stacks[from-1][:len(stacks[from-1])-count]
		} else {
			stacks[to-1] = append(stacks[to-1], stacks[from-1][:]...)
			stacks[from-1] = make([]string, 0)
		}
	}
	res := ""
	for _, v := range stacks {
		res += v[len(v)-1]
	}
	fmt.Printf("Part 1: %v\n", res)
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