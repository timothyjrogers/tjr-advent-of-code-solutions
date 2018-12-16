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

func getNonInfinitePoints(points []point) ([]point, int, int, int, int) {
	var nonInfinitePoints []point
	min_x, min_y := 100000000000000, 100000000000000
	max_x, max_y := -100000000000000, -100000000000000
	for _, v := range points {
		if v.x < min_x { min_x = v.x }
		if v.x > max_x { max_x = v.x } 
		if v.y < min_y { min_y = v.y }
		if v.y > max_y { max_y = v.y }
	}
	for _, v := range points {
		if v.x > min_x && v.x < max_x && v.y > min_y && v.y < max_y {
			nonInfinitePoints = append(nonInfinitePoints, v)
		}
	}
	return nonInfinitePoints, max_x, max_y, min_x, min_y
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

	nonInfinitePoints, x_max, y_max, x_min, y_min := getNonInfinitePoints(pointsInput)
	var extraInfinitePoints []point
	closestPointsToX := make(map[string]int)

	for i := x_min; i <= x_max; i++ {
		for j := y_min; j <= y_max; j++ {
			var closestOrigin point
			var closestOriginDist = 100000000000000
			mdistSeen := make(map[int]int)
			for _, p := range pointsInput {
				mdist := manhattanDist(p, point{x: i, y: j, stringRep: intsToPointString(i, j)})
				if _, ok := mdistSeen[mdist]; ok { mdistSeen[mdist] = mdistSeen[mdist] + 1} else { mdistSeen[mdist] = 1 }
				if mdist < closestOriginDist {
					closestOriginDist = mdist
					closestOrigin = p
				}
			}
			if mdistSeen[closestOriginDist] == 1 {
				if _, ok := closestPointsToX[closestOrigin.stringRep]; ok { closestPointsToX[closestOrigin.stringRep] += 1 } else { closestPointsToX[closestOrigin.stringRep] = 1 }
			}
			if i == x_max || j == y_max { extraInfinitePoints = append(extraInfinitePoints, closestOrigin) }
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