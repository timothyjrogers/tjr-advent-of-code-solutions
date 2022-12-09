package main

import (
	"fmt"
	"os"
	"bufio"
	//"strings"
	"strconv"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	width := len(lines[0])
	height := len(lines)
	grid := make([]int, width*height)
	for i, line := range lines {
		for j, c := range line {
			val, _ := strconv.Atoi(string(c))
			grid[i*width + j] = val
		}
	}
	sum := 0
	for i := 0; i < len(grid); i++ {
		visible := [4]bool{true, true, true, true}
		row := i / width;
		col := i % width;
		if row == 0 || row == height - 1 || col == 0 || col == width -1 {
			sum += 1
			continue
		}
		for j := row-1; j >= 0; j-- {
			if grid[i] <= grid[j*width + col] {
				visible[0] = false
				break
			}
		}
		for j := row+1; j < height; j++ {
			if grid[i] <= grid[j*width + col] {
				visible[1] = false
				break
			}
		}
		for j := col-1; j >= 0; j-- {
			if grid[i] <= grid[row*width + j] {
				visible[2] = false
				break
			}
		}
		for j := col+1; j < width; j++ {
			if grid[i] <= grid[row*width + j] {
				visible[3] = false
				break
			}
		}
		if visible[0] || visible[1] || visible[2] || visible[3] {
			sum += 1
		}
	}
	fmt.Printf("Part 1: %v\n", sum)
}

func partTwo(lines []string) {
	width := len(lines[0])
	height := len(lines)
	grid := make([]int, width*height)
	for i, line := range lines {
		for j, c := range line {
			val, _ := strconv.Atoi(string(c))
			grid[i*width + j] = val
		}
	}
	res := 0
	for i := 0; i < len(grid); i++ {
		row := i / width;
		col := i % width;
		score := 1
		if row == 0 || row == height - 1 || col == 0 || col == width -1 {
			continue
		}
		count := 0
		for j := row-1; j >= 0; j-- {
			count++
			if grid[i] <= grid[j*width + col] {
				break
			}
		}
		score = score * count
		count = 0
		for j := row+1; j < height; j++ {
			count++
			if grid[i] <= grid[j*width + col] {
				break
			}
		}
		score = score * count
		count = 0
		for j := col-1; j >= 0; j-- {
			count++
			if grid[i] <= grid[row*width + j] {
				break
			}
		}
		score = score * count
		count = 0
		for j := col+1; j < width; j++ {
			count++
			if grid[i] <= grid[row*width + j] {
				break
			}
		}
		score = score * count
		if score > res {
			res = score
		}
	}
	fmt.Printf("Part 2: %v", res)
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