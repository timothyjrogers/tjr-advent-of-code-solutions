package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"math"
	"strconv"
)

const input_file string = "input.txt"

func printCave(cave [][]string) {
	out := ""
	for _, row := range cave {
		for _, s := range row {
			out += s
		}
		out += "\n"
	}
	fmt.Println(out)
}

func partOne(lines []string) {
	xMin := math.MaxInt32
	xMax := 0
	yMin := math.MaxInt32
	yMax := 0
	for _, line := range lines {
		points := strings.Split(line, " -> ")
		for _, point := range points {
			xy := strings.Split(point, ",")
			x, _ := strconv.Atoi(xy[0])
			y, _ := strconv.Atoi(xy[1])
			if x < xMin {
				xMin = x
			}
			if x > xMax {
				xMax = x
			}
			if y < yMin {
				yMin = y
			}
			if y > yMax {
				yMax = y
			}
		}
	}
	cave := make([][]string, 0)
	for i := 0; i <= yMax; i++ {
		row := make([]string, 0)
		for j := 0; j <= xMax; j++ {
			row = append(row, " ")
		}
		cave = append(cave, row)
	}
	for _, line := range lines {
		points := strings.Split(line, " -> ")
		for i := 0; i < len(points) - 1; i++ {
			p1 := points[i]
			p2 := points[i+1]
			xy1 := strings.Split(p1, ",")
			x1, _ := strconv.Atoi(xy1[0])
			y1, _ := strconv.Atoi(xy1[1])
			x1 = x1
			y1 = y1
			xy2 := strings.Split(p2, ",")
			x2, _ := strconv.Atoi(xy2[0])
			y2, _ := strconv.Atoi(xy2[1])
			x2 = x2
			y2 = y2
			if x1 == x2 {
				//vertical
				yHigh := int(math.Max(float64(y1), float64(y2)))
				yLow := int(math.Min(float64(y1), float64(y2)))
				for j := yLow; j <= yHigh; j++ {
					cave[j][x1] = "#"
				}
			} else {
				//horizontal
				xHigh := int(math.Max(float64(x1), float64(x2)))
				xLow := int(math.Min(float64(x1), float64(x2)))
				for j := xLow; j <= xHigh; j++ {
					cave[y1][j] = "#"
				}
			}
		}
	}
	sum := 0
	done := false
	for {
		sandX := 500
		sandY := 0
		for {
			if sandX < 0 || sandX >= xMax || sandY >= yMax {
				done = true
				break
			}
			if cave[sandY+1][sandX] != " " && cave[sandY+1][sandX-1] != " " && cave[sandY+1][sandX+1] != " " {
				break
			} else if cave[sandY+1][sandX] == " " {
				sandY++
			} else if cave[sandY+1][sandX-1] == " " {
				sandY++
				sandX--
			} else if cave[sandY+1][sandX+1] == " " {
				sandY++
				sandX++
			}
		}
		if done {
			break
		} else {
			cave[sandY][sandX] = "+"
			sum++
		}
	}
	fmt.Printf("Part 1: %v\n", sum)
}

func partTwo(lines []string) {
	xMin := math.MaxInt32
	xMax := 0
	yMin := math.MaxInt32
	yMax := 0
	for _, line := range lines {
		points := strings.Split(line, " -> ")
		for _, point := range points {
			xy := strings.Split(point, ",")
			x, _ := strconv.Atoi(xy[0])
			y, _ := strconv.Atoi(xy[1])
			if x < xMin {
				xMin = x
			}
			if x > xMax {
				xMax = x
			}
			if y < yMin {
				yMin = y
			}
			if y > yMax {
				yMax = y
			}
		}
	}
	cave := make([][]string, 0)
	for i := 0; i <= yMax+2; i++ {
		row := make([]string, 0)
		for j := 0; j <= 1000; j++ {
			if i < yMax+2 {
				row = append(row, " ")
			} else {
				row = append(row, "#")
			}
		}
		cave = append(cave, row)
	}
	for _, line := range lines {
		points := strings.Split(line, " -> ")
		for i := 0; i < len(points) - 1; i++ {
			p1 := points[i]
			p2 := points[i+1]
			xy1 := strings.Split(p1, ",")
			x1, _ := strconv.Atoi(xy1[0])
			y1, _ := strconv.Atoi(xy1[1])
			x1 = x1
			y1 = y1
			xy2 := strings.Split(p2, ",")
			x2, _ := strconv.Atoi(xy2[0])
			y2, _ := strconv.Atoi(xy2[1])
			x2 = x2
			y2 = y2
			if x1 == x2 {
				//vertical
				yHigh := int(math.Max(float64(y1), float64(y2)))
				yLow := int(math.Min(float64(y1), float64(y2)))
				for j := yLow; j <= yHigh; j++ {
					cave[j][x1] = "#"
				}
			} else {
				//horizontal
				xHigh := int(math.Max(float64(x1), float64(x2)))
				xLow := int(math.Min(float64(x1), float64(x2)))
				for j := xLow; j <= xHigh; j++ {
					cave[y1][j] = "#"
				}
			}
		}
	}
	sum := 0
	done := false
	for cave[0][500] != "+" {
		sandX := 500
		sandY := 0
		for {
			if cave[sandY+1][sandX] != " " && cave[sandY+1][sandX-1] != " " && cave[sandY+1][sandX+1] != " " {
				break
			} else if cave[sandY+1][sandX] == " " {
				sandY++
			} else if cave[sandY+1][sandX-1] == " " {
				sandY++
				sandX--
			} else if cave[sandY+1][sandX+1] == " " {
				sandY++
				sandX++
			}
		}
		if !done {
			cave[sandY][sandX] = "+"
			sum++
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