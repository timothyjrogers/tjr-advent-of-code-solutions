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
	var nonInfinitePoints []point
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

func isEdgePoint(p point, max_x int, max_y int, min_x int, min_y int) bool {
	return p.x == max_x || p.x == min_x || p.y == max_y || p.y == min_y
}

func manhattanDist(p1 point, p2 point) int {
	return int(math.Abs(float64(p1.x) - float64(p2.x))) + int(math.Abs(float64(p1.y) - float64(p2.y)))
}

func isOrigin(points []point, x int, y int) bool {
	origin := false
	for _, p := range points {
		if p.x == x && p.y == y { origin = true }
	}
	return origin
}

func getAdjacentPoints(p point) []point {
	var points [8]point
	points[0] = point{x: p.x + 1, y: p.y, stringRep: intsToPointString(p.x + 1, p.y)}
	points[1] = point{x: p.x - 1, y: p.y, stringRep: intsToPointString(p.x - 1, p.y)}
	points[2] = point{x: p.x, y: p.y + 1, stringRep: intsToPointString(p.x, p.y + 1)}
	points[3] = point{x: p.x, y: p.y - 1, stringRep: intsToPointString(p.x, p.y - 1)}
	points[4] = point{x: p.x + 1, y: p.y + 1, stringRep: intsToPointString(p.x + 1, p.y + 1)}
	points[5] = point{x: p.x + 1, y: p.y - 1, stringRep: intsToPointString(p.x + 1, p.y - 1)}
	points[6] = point{x: p.x - 1, y: p.y + 1, stringRep: intsToPointString(p.x - 1, p.y + 1)}
	points[7] = point{x: p.x - 1, y: p.y - 1, stringRep: intsToPointString(p.x - 1, p.y - 1)}
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

	//nonInfinitePoints, x_max, y_max, x_min, y_min := getNonInfinitePoints(pointsInput)
	x_max, y_max, x_min, y_min := getBounds(pointsInput)
	var InfinitePoints []point
	closestPointsToX := make(map[string]int)

	for _, point := pointsInput {
		var seachStack []point
		var seenPoints map[string]bool
		searchStack = append(searchStack, getAdjacentPoints(point)...)
		//infiniteOrigin := false
		edgePoint := isEdgePoint(searchPoint, max_x, max_y, min_x, min_y)
		for len(searchStack) > 0 {
			searchPoint := searchStack[len(searchStack) - 1]
			searchStack = searchStack[:len(searchStack) - 1]
			if _, ok := seenPoints[searchPoint.stringRep]; ok { continue } else { seenPoints[searchPoint.stringRep] = true }
			maxDist := 0
			var maxDistPoint point
			for _, origin := pointsInput {
				
			}
		}
	}

	maxArea := 0
	for _, p := range nonInfinitePoints {
		infinite := false
		for _, i := range extraInfinitePoints { if i.stringRep == p.stringRep { infinite = true }}
		if infinite { continue }
		if closestPointsToX[p.stringRep] > maxArea { maxArea = closestPointsToX[p.stringRep] }
	}
	fmt.Println(maxArea)
}