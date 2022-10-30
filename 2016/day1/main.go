package main

import (
	"fmt"
	"os"
	"bufio"
	"math"
	"strings"
	"strconv"
)

const input_file string = "input.txt"

type point struct {
	x int
	y int
}

func newPoint(x, y int) *point {
	p := point{x, y}
	return &p
}

func contains(points []point, p point) bool {
	for i := 0; i < len(points); i++ {
		if points[i] == p {
			return true
		}
	}
	return false
}

func manhattan(x1, x2, y1, y2 int) float64 {
	return math.Abs(float64(x1) - float64(x2)) + math.Abs(float64(y1) - float64(y2))
}

func new_face(face int, change string) int {
	new_face := face
	if change == "L" {
		new_face = new_face -1
	} else if change == "R" {
		new_face = new_face + 1
	}

	if new_face == 4 {
		new_face = 0
	} else if new_face == -1 {
		new_face = 3
	}

	return new_face
}

func main() {
	file, _ := os.Open(input_file)
	reader := bufio.NewReader(file)
	input, _ := reader.ReadString('\n')
	instructions := strings.Split(input, ", ")
	
	face := 0 //0 == north, 1 == east, 2 == south, 3 == west
	x, y := 0, 0
	var points []point
	points = append(points, *newPoint(0,0))
	repeat_found := false
	for i := 0; i < len(instructions); i++ {
		face = new_face(face, string(instructions[i][0]))
		dist, _ := strconv.Atoi(instructions[i][1:])
		if face == 0 {
			for j := 0; j < dist; j++ {
				y = y + 1
				if !repeat_found {
					p := newPoint(x, y)
					if contains(points, *p) {
						fmt.Printf("Part 2: %v\n", manhattan(0, x, 0, y))
						repeat_found = true
					}
					points = append (points, *newPoint(x, y))
				}
			}
		} else if face == 1 {
			for j := 0; j < dist; j++ {
				x = x + 1
				if !repeat_found {
					p := newPoint(x, y)
					if contains(points, *p) {
						fmt.Printf("Part 2: %v\n", manhattan(0, x, 0, y))
						repeat_found = true
					}
					points = append (points, *newPoint(x, y))
				}
			}
		} else if face == 2 {
			for j := 0; j < dist; j++ {
				y = y - 1
				if !repeat_found {
					p := newPoint(x, y)
					if contains(points, *p) {
						fmt.Printf("Part 2: %v\n", manhattan(0, x, 0, y))
						repeat_found = true
					}
					points = append (points, *newPoint(x, y))
				}
			}
		} else if face == 3 {
			for j := 0; j < dist; j++ {
				x = x - 1
				if !repeat_found {
					p := newPoint(x, y)
					if contains(points, *p) {
						fmt.Printf("Part 2: %v\n", manhattan(0, x, 0, y))
						repeat_found = true
					}
					points = append (points, *newPoint(x, y))
				}
			}
		}
	}
	fmt.Printf("Part 1: %v", manhattan(0, x, 0, y))
}