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
const targetRow int = 2000000

func parseLine(line string) (int, int, int, int) {
	sb := strings.Split(line, ": ")
	sensor := strings.Split(sb[0], ", ")
	sensorX := strings.Split(sensor[0][10:], "=")[1]
	sensorY := strings.Split(sensor[1], "=")[1]
	beacon := strings.Split(sb[1], ", ")
	beaconX := strings.Split(beacon[0][21:], "=")[1]
	beaconY := strings.Split(beacon[1], "=")[1]
	x1, _ := strconv.Atoi(sensorX)
	y1, _ := strconv.Atoi(sensorY)
	x2, _ := strconv.Atoi(beaconX)
	y2, _ := strconv.Atoi(beaconY)
	return x1, y1, x2, y2
}

func mergeIntervals(intervals [][2]int, r [2]int) [][2]int {	
	for i := 0; i < len(intervals); i++ {
		if r[0] >= intervals[i][0] && r[1] <= intervals[i][1] {
			return intervals
		} else if intervals[i][0] >= r[0] && intervals[i][1] <= r[1] {
			intervals[i] = r
			return intervals
		} else if r[0] >= intervals[i][0] && r[0] <= intervals[i][1] && r[1] > intervals[i][1] {
			intervals[i][1] = r[1]
			return intervals
		} else if r[1] >= intervals[i][0] && r[1] <= intervals[i][1] && r[0] < intervals[i][0] {
			intervals[i][0] = r[0]
			return intervals
		}
	}
	return append(intervals, r)
}

func partOne(lines []string) {
	intervals := make(map[int][][2]int, 0)
	for _, line := range lines {
		sensorX, sensorY, beaconX, beaconY := parseLine(line)
		manhattan := int(math.Abs(float64(sensorX) - float64(beaconX)) + math.Abs(float64(sensorY) - float64(beaconY)))
		for y := sensorY - manhattan; y <= sensorY + manhattan; y++ {
			xOffset := manhattan - int(math.Abs(float64(y) - float64(sensorY)))
			r := [2]int{sensorX - xOffset, sensorX + xOffset}
			if beaconY == y && r[0] == r[1] {
				continue
			} else if beaconY == y && r[0] == beaconX {
				r[0]++
			} else if beaconY == y && r[1] == beaconX {
				r[1]--
			}
			if val, ok := intervals[y]; ok {
				intervals[y] = mergeIntervals(val, r)
			} else {
				intervals[y] = [][2]int{r}
			}
		}
	}
	counter := make(map[int]bool) 
	for _, interval := range intervals[targetRow] {
		for i := interval[0]; i <= interval[1]; i++ {
			counter[i] = true
		}
	}
	fmt.Printf("Part 1: %v\n", len(counter))
}

func coordInArea(diamond [8]int, coord [2]int) bool {
	var xRange [2]int
	diamondCenterY := diamond[1] + (diamond[5] - diamond[1])/2
	if coord[1] >= diamond[1] && coord[1] <= diamond[5] {
		yDiff := int(math.Abs(float64(coord[1]) - float64(diamondCenterY)))
		xRange[0] = diamond[6] + yDiff
		xRange[1] = diamond[2] - yDiff
		if coord[0] <= xRange[1] && coord[0] >= xRange[0] {
			return true
		} else {
			return false
		}
	} else {
		return false
	}
	return false
}

func getNeighbors(diamond [8]int) [][2]int {
	neighbors := make([][2]int, 0)
	if diamond[1] > 0 {
		neighbors = append(neighbors, [2]int{diamond[0], diamond[1]-1})
	}
	if diamond[5] < 20 {
		neighbors = append(neighbors, [2]int{diamond[4], diamond[5]+1})
	}
	diamondCenterY := diamond[1] + (diamond[5] - diamond[1])/2
	for i := diamond[1]; i <= diamond[5]; i++ {
		if i > 4000000 || i < 0 {
			continue
		}
		xRange := [2]int{diamond[6], diamond[2]}
		yDiff := int(math.Abs(float64(i) - float64(diamondCenterY)))
		if i == diamondCenterY {
			if xRange[0] > 0 {
				neighbors = append(neighbors, [2]int{xRange[0] - 1, diamondCenterY})
			}
			if xRange[1] < 4000000 {
				neighbors = append(neighbors, [2]int{xRange[1] + 1, diamondCenterY})
			}
		} else {
			xRange[0] += yDiff
			xRange[1] -= yDiff
			if xRange[0] > 0 {
				neighbors = append(neighbors, [2]int{xRange[0]-1, i})
				neighbors = append(neighbors, [2]int{xRange[0]-1, i})
			}
			if xRange[1] < 4000000 {
				neighbors = append(neighbors, [2]int{xRange[1]+1, i})
				neighbors = append(neighbors, [2]int{xRange[1]+1, i})
			}
		}
	}
	return neighbors
}

func partTwo(lines []string) {
	coords := make([][8]int, 0)
	for _, line := range lines {
		sensorX, sensorY, beaconX, beaconY := parseLine(line)
		manhattan := int(math.Abs(float64(sensorX) - float64(beaconX)) + math.Abs(float64(sensorY) - float64(beaconY)))
		//[top, right, bottom, left]
		diamond := [8]int{sensorX, sensorY - manhattan, sensorX + manhattan, sensorY, sensorX, sensorY + manhattan, sensorX - manhattan, sensorY}
		coords = append(coords, diamond)
	}
	neighbors := make([][2]int, 0)
	for _, diamond := range coords {
		neighbors = append(neighbors, getNeighbors(diamond)...)
	}
	x := int64(0)
	y := int64(0)
	for _, neighbor := range neighbors {
		count := 0
		for _, diamond := range coords {
			if coordInArea(diamond, neighbor) {
				count++
			}
		}
		if count == 0 {
			x = int64(neighbor[0])
			y = int64(neighbor[1])
			break
		}
	}
	fmt.Printf("Part 2: %v", x * 4000000 + y)
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