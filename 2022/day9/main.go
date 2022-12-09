package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
	"math"
)

const input_file string = "input.txt"

func partOne(lines []string) {
	head := [2]int{0,0}
	tail := [2]int{0,0}
	seen := map[string]bool{"0,0": true}
	for _, line := range lines {
		move := strings.Split(line, " ")
		dir := move[0]
		steps, _ := strconv.Atoi(move[1])
		for i := 1; i <= steps; i++ {
			switch dir {
			case "D":
				head[1]--
			case "U":
				head[1]++
			case "L":
				head[0]--
			case "R":
				head[0]++
			}
			dist := math.Abs(float64(head[0])-float64(tail[0])) + math.Abs(float64(head[1])-float64(tail[1]))
			if dist > 1 {
				if head[1] == tail[1] && head[0] > tail[0] {
					tail[0]++
				} else if head[1] == tail[1] && head[0] < tail[0] {
					tail[0]--
				} else if head[0] == tail[0] && head[1] > tail[1] {
					tail[1]++ 
				} else if head[0] == tail[0] && head[1] < tail[1] {
					tail[1]--
				} else if head[0] > tail[0] && head[1] > tail[1] && dist > 2 {
					tail[0]++
					tail[1]++
				} else if head[0] > tail[0] && head[1] < tail[1] && dist > 2 {
					tail[0]++
					tail[1]--
				} else if head[0] < tail[0] && head[1] > tail[1] && dist > 2 {
					tail[0]--
					tail[1]++
				} else if head[0] < tail[0] && head[1] < tail[1] && dist > 2 {
					tail[0]--
					tail[1]--
				}
				seen[strconv.Itoa(tail[0]) + "," + strconv.Itoa(tail[1])] = true
			}
		}
	}
	fmt.Printf("Part 1: %v\n", len(seen))
}

func partTwo(lines []string) {
	var knots [10][2]int
	for i := 0; i < 10; i++ {
		knots[i] = [2]int{0,0}
	}
	seen := map[string]bool{"0,0": true}
	for _, line := range lines {
		move := strings.Split(line, " ")
		dir := move[0]
		steps, _ := strconv.Atoi(move[1])
		for i := 1; i <= steps; i++ {
			switch dir {
			case "D":
				knots[0][1]--
			case "U":
				knots[0][1]++
			case "L":
				knots[0][0]--
			case "R":
				knots[0][0]++
			}
			for j := 1; j <= 9; j++ {
				dist := math.Abs(float64(knots[j-1][0])-float64(knots[j][0])) + math.Abs(float64(knots[j-1][1])-float64(knots[j][1]))
				if dist > 1 {
					if knots[j-1][1] == knots[j][1] && knots[j-1][0] > knots[j][0] {
						knots[j][0]++
					} else if knots[j-1][1] == knots[j][1] && knots[j-1][0] < knots[j][0] {
						knots[j][0]--
					} else if knots[j-1][0] == knots[j][0] && knots[j-1][1] > knots[j][1] {
						knots[j][1]++ 
					} else if knots[j-1][0] == knots[j][0] && knots[j-1][1] < knots[j][1] {
						knots[j][1]--
					} else if knots[j-1][0] > knots[j][0] && knots[j-1][1] > knots[j][1] && dist > 2 {
						knots[j][0]++
						knots[j][1]++
					} else if knots[j-1][0] > knots[j][0] && knots[j-1][1] < knots[j][1] && dist > 2 {
						knots[j][0]++
						knots[j][1]--
					} else if knots[j-1][0] < knots[j][0] && knots[j-1][1] > knots[j][1] && dist > 2 {
						knots[j][0]--
						knots[j][1]++
					} else if knots[j-1][0] < knots[j][0] && knots[j-1][1] < knots[j][1] && dist > 2 {
						knots[j][0]--
						knots[j][1]--
					}
				}
			}
			seen[strconv.Itoa(knots[9][0]) + "," + strconv.Itoa(knots[9][1])] = true
		}
	}
	fmt.Printf("Part 2: %v\n", len(seen))
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