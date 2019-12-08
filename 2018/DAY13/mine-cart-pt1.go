package main

import ("fmt"; "os"; "bufio"; "strings")

type cart struct {
	x, y int
	lastIntersection string
}

type maptile struct {
	tile rune
	x, y int
}

func main() {
	//initialize file access for input
	inputPath := "subterranean-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	map := make([][]maptile)
	carts := []cart

	for scanner.Scan() {
		rule := strings.Fields(scanner.Text())
		if rule[2] == "#" { rules[rule[0]] = true }
	}
}