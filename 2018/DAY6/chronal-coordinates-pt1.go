package main

import ("fmt"; "os"; "bufio"; "strconv"; "strings"; "math")

type point struct {
	x, y int
	stringRep string
}

func pointStringToInts(point string) (int, int) {
	tokens := strings.Split(point, ", ")
	x, _ := strconv.Atoi(tokens[0])
	y, _ := strconv.Atoi(tokens[1])
	return x, y
}

func intsToPointString(x int, y int) string {
	x_string := strconv.Itoa(x)
	y_string := strconv.Itoa(y)
	return x_string + ", " + y_string
}

func getBounds(points []point) (int, int, int, int) {
	min_x, min_y := 100000000000000, 100000000000000
	max_x, max_y := -100000000000000, -100000000000000
	for _, v := range points {
		if v.x < min_x { min_x = v.x }
		if v.x > max_x { max_x = v.x } 
		if v.y < min_y { min_y = v.y }
		if v.y > max_y { max_y = v.y }
	}
	return max_x, max_y, min_x, min_y
}

func getCenter(x_max int, y_max int, x_min int, y_min int) (int, int) {
	x_mid := x_min + ((x_max - x_min) / 2)
	y_mid := y_min + ((y_max - y_min) / 2)
	return x_mid, y_mid
}

func isEdgePoint(p point, max_x int, max_y int, min_x int, min_y int) bool {
	return p.x == max_x || p.x == min_x || p.y == max_y || p.y == min_y
}

func manhattanDist(p1 point, p2 point) int {
	return int(math.Abs(float64(p1.x) - float64(p2.x))) + int(math.Abs(float64(p1.y) - float64(p2.y)))
}

func removeSeenPoints(points []point, exclude map[string]bool) []point {
	var newPoints []point
	for _, p := range points {
		if _, ok := exclude[p.stringRep]; !ok { newPoints = append(newPoints, p)}
	}
	return newPoints
}

func getAdjacentPoints(p point, exclude map[string]bool) []point {
	var points []point
	points = append(points, point{x: p.x + 1, y: p.y, stringRep: intsToPointString(p.x + 1, p.y)})
	points = append(points, point{x: p.x - 1, y: p.y, stringRep: intsToPointString(p.x - 1, p.y)})
	points = append(points, point{x: p.x, y: p.y + 1, stringRep: intsToPointString(p.x, p.y + 1)})
	points = append(points, point{x: p.x, y: p.y - 1, stringRep: intsToPointString(p.x, p.y - 1)})
	points = append(points, point{x: p.x + 1, y: p.y + 1, stringRep: intsToPointString(p.x + 1, p.y + 1)})
	points = append(points, point{x: p.x + 1, y: p.y - 1, stringRep: intsToPointString(p.x + 1, p.y - 1)})
	points = append(points, point{x: p.x - 1, y: p.y + 1, stringRep: intsToPointString(p.x - 1, p.y + 1)})
	points = append(points, point{x: p.x - 1, y: p.y - 1, stringRep: intsToPointString(p.x - 1, p.y - 1)})
	points = removeSeenPoints(points, exclude)
	return points
}

func main() {
	inputPath := "chronal-coordinates-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	var pointsInput []point
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		pointString := scanner.Text()
		point_x, point_y := pointStringToInts(pointString)
		p := point{x: point_x, y: point_y, stringRep: pointString}
		pointsInput = append(pointsInput, p	)
	}

	x_max, y_max, x_min, y_min := getBounds(pointsInput)
	midpoint_x, midpoint_y := getCenter(x_max, y_max, x_min, y_min)
	midpoint := point{x: midpoint_x, y: midpoint_y, stringRep: intsToPointString(midpoint_x, midpoint_y)}

	infinitePoints := make(map[string]bool)
	closestPointsToX := make(map[string]int)
	seenPoints := make(map[string]bool)

	var searchStack []point
	searchStack = append(searchStack, midpoint)
	for len(searchStack) > 0 {
		searchPoint := searchStack[len(searchStack) - 1]
		searchStack = searchStack[:len(searchStack) - 1]
		if _, ok := seenPoints[searchPoint.stringRep]; ok { continue }
		maxDist := 100000000000000
		var maxDistPoint point
		seenDistances := make(map[int]int)
		for _, origin := range pointsInput {
			mdist := manhattanDist(searchPoint, origin)
			if _, ok := seenDistances[mdist]; ok { seenDistances[mdist] += 1 } else { seenDistances[mdist] = 1 }
			if mdist < maxDist {
				maxDist = mdist
				maxDistPoint = origin
			}
		}
		if isEdgePoint(searchPoint, x_max, y_max, x_min, y_min) {
			infinitePoints[maxDistPoint.stringRep] = true
			continue 
		} else if seenDistances[maxDist] == 1 {
			if _, ok := closestPointsToX[maxDistPoint.stringRep]; ok { closestPointsToX[maxDistPoint.stringRep] += 1} else { closestPointsToX[maxDistPoint.stringRep] = 1}
		}
		seenPoints[searchPoint.stringRep] = true
		searchStack = append(searchStack, getAdjacentPoints(searchPoint, seenPoints)...)
	}

	maxArea := 0
	for _, p := range pointsInput {
		pointString := p.stringRep
		infinite := false
		if _, ok := infinitePoints[pointString]; ok { infinite = true }
		if infinite { continue }
		if closestPointsToX[pointString] > maxArea { maxArea = closestPointsToX[pointString] }
	}
	fmt.Println(maxArea)
}