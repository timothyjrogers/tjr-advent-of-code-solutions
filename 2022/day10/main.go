package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	x := 1
	sum := 0
	lineNum := 0
	instr := make([]string, 0)
	cycleSteps := 0
	for cycle := 1; cycle <= 220; cycle++ {
		if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
			sum += x * cycle
		}
		if len(instr) == 0 && lineNum < len(lines) {
			instr = strings.Split(lines[lineNum], " ")
			lineNum++
		} else if lineNum >= len(lines) {
			continue
		}
		if instr[0] == "noop" {
			instr = make([]string, 0)
		} else if cycleSteps == 0 {
			cycleSteps++
		} else if cycleSteps == 1 {
			val, _ := strconv.Atoi(instr[1])
			x += val
			cycleSteps = 0
			instr = make([]string, 0)
		}
	}
	fmt.Printf("Part 1: %v\n", sum)
}

func printScreen(screen [][]string) {
	view := ""
	for  i := 0; i < len(screen); i++ {
		view = view + strings.Join(screen[i], "") + "\n"
	}
	fmt.Printf("%v", view)
}

func partTwo(lines []string) {
	x := 1
	lineNum := 0
	instr := make([]string, 0)
	cycleSteps := 0
	screen := make([][]string, 0)
	for i := 0; i < 6; i++ {
		row := make([]string, 0)
		for j :=0; j < 40; j++ {
			row = append(row, ".")
		}
		screen = append(screen, row)
	}
	cycle := 0
	for {
		screenY := cycle / 40
		screenX := cycle % 40
		regX := x % 40
		if regX >= screenX -1 && regX <= screenX + 1  {
			screen[screenY][screenX] = "#"
		}
		if len(instr) == 0 && lineNum < len(lines) {
			instr = strings.Split(lines[lineNum], " ")
			lineNum++
		} else if len(instr) == 0 && lineNum >= len(lines) {
			break
		}
		if instr[0] == "noop" {
			instr = make([]string, 0)
		} else if cycleSteps == 0 {
			cycleSteps++
		} else if cycleSteps == 1 {
			val, _ := strconv.Atoi(instr[1])
			x += val
			cycleSteps = 0
			instr = make([]string, 0)
		}
		cycle++
	}
	fmt.Println("Part 2: ")
	printScreen(screen)
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