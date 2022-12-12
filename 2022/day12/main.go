package main

import (
	"fmt"
	"os"
	"bufio"
	"math"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	yDim := len(lines)
	xDim := len(lines[0])
	grid := make([]string, 0)
	adj := make(map[int][]int, 0)
	start := 0
	heightVals := map[string]int{ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": 6, "g": 7, "h": 8, "i": 9, "j": 10, "k": 11, "l": 12, "m": 13, "n": 14, "o": 15, "p": 16, "q": 17, "r": 18, "s": 19, "t": 20, "u": 21, "v": 22, "w": 23, "x": 24, "y": 25, "z": 26, "E": 26, "S": 1}
	for i, line := range lines {
		for j, c := range line {
			grid = append(grid, string(c))
			if string(c) == "S" {
				start = i * xDim + j
			}
		}
	}
	for idx, _ := range grid {
		x := idx % xDim
		y := idx / xDim
		adj[idx] = make([]int, 0)
		if x > 0 && heightVals[grid[idx-1]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx - 1)
		}
		if x < xDim-1 && heightVals[grid[idx+1]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx + 1)
		}	
		if y > 0 && heightVals[grid[idx-xDim]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx - xDim)
		}	
		if y < yDim-1 && heightVals[grid[idx+xDim]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx + xDim)
		}
	}
	queue := []int{start} //x = index % xDim, y = index / xDim
	seen := map[int]bool{start: true}
	prev := make([]int, 0)
	end := 0
	for i := 0; i < len(grid); i++ {
		prev = append(prev, -1)
	}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		if grid[cur] == "E" {
			end = cur
			break
		}
		for _, neighbor := range adj[cur] {
			if _, ok := seen[neighbor]; !ok {
				seen[neighbor] = true
				queue = append(queue, neighbor)
				prev[neighbor] = cur
			}
		}
	}
	path := []int{end}
	for {
		if prev[end] != -1 {
			path = append(path, prev[end])
			end = prev[end]
		} else {
			break
		}
	}
	fmt.Printf("Part 1: %v\n", len(path)-1)
}

func partTwo(lines []string) {
	yDim := len(lines)
	xDim := len(lines[0])
	grid := make([]string, 0)
	adj := make(map[int][]int, 0)
	starts := make([]int, 0)
	min := math.MaxInt32
	heightVals := map[string]int{ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": 6, "g": 7, "h": 8, "i": 9, "j": 10, "k": 11, "l": 12, "m": 13, "n": 14, "o": 15, "p": 16, "q": 17, "r": 18, "s": 19, "t": 20, "u": 21, "v": 22, "w": 23, "x": 24, "y": 25, "z": 26, "E": 26, "S": 1}
	for i, line := range lines {
		for j, c := range line {
			grid = append(grid, string(c))
			if string(c) == "S" || string(c) == "a" {
				starts = append(starts, i * xDim + j)
			}
		}
	}
	for idx, _ := range grid {
		x := idx % xDim
		y := idx / xDim
		adj[idx] = make([]int, 0)
		if x > 0 && heightVals[grid[idx-1]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx - 1)
		}
		if x < xDim-1 && heightVals[grid[idx+1]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx + 1)
		}	
		if y > 0 && heightVals[grid[idx-xDim]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx - xDim)
		}	
		if y < yDim-1 && heightVals[grid[idx+xDim]] - heightVals[grid[idx]] <= 1 {
			adj[idx] = append(adj[idx], idx + xDim)
		}
	}
	fmt.Println(adj[starts[len(starts)-1]])
	for iter := 0; iter < len(starts); iter++ {
		queue := []int{starts[iter]}
		seen := map[int]bool{starts[iter]: true}
		prev := make([]int, 0)
		end := 0
		for i := 0; i < len(grid); i++ {
			prev = append(prev, -1)
		}
		for len(queue) > 0 {
			cur := queue[0]
			queue = queue[1:]
			if grid[cur] == "E" {
				end = cur
				break
			}
			for _, neighbor := range adj[cur] {
				if _, ok := seen[neighbor]; !ok {
					seen[neighbor] = true
					queue = append(queue, neighbor)
					prev[neighbor] = cur
				}
			}
		}
		path := []int{end}
		for {
			if prev[end] != -1 {
				path = append(path, prev[end])
				end = prev[end]
			} else {
				break
			}
		}
		//fmt.Printf("Path len %v: %v\n", iter, len(path)-1)
		if len(path) > 1 && len(path) - 1 < min {
			min = len(path) - 1
		}
	}
	fmt.Printf("Part 2: %v", min)
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