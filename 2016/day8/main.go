package main

import (
	"fmt"
	//"os"
	//"bufio"
)

const input_file string = "input.txt"

func rectAxB(cur [][]bool, a int, b int) [][]bool {
	grid := cur
	for i := 0; i < b; i++ {
		for j := 0; j < a; j++ {
			grid[j][i] = true
		}
	}
	return grid
}

func rotateRow(cur [][]bool, a int, b int) [][]bool {
	grid := cur
	row := make([]bool, 50)
	for i := 0; i < 50; i++ {
		row[(i+b)%50] = grid[a][i]
	}
	grid[a] = row
	return grid
}

func rotateCol(cur [][]bool, a int, b int) [][]bool {
	grid := cur
	col := make([]bool, 6)
	for i := 0; i < 6; i++ {
		col[(i+b)%6] = grid[i][a]
	}
	for i, c := range col {
		grid[i][a] = c
	}
	return grid
}


func partOne(lines []string) {
	res := 0
	grid := make([][]bool, 6)
	for i := 0; i < 6; i++ {
		for j := 0; j < 50; j++ {
			grid[i] = append(grid[i], false)
		}
	}
	for _, line := range lines {

	}
	fmt.Printf("Part 1: %v\n", res)
}


func printGrid(grid [][]bool) {
	for i := 0; i < 6; i++ {
		for j := 0; j < 50; j++ {
			c := "."
			if grid[i][j] {
				c = "#"
			}
			fmt.Printf("%v", c)
		}
		fmt.Printf("\n")
	}
}

func main() {
	/*
    file, _ := os.Open(input_file)
    defer file.Close()

    scanner := bufio.NewScanner(file)
	var lines []string
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }
	*/

	//partOne(lines)
	//partTwo(lines)

}