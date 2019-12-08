package main

import ("fmt"; "os"; "bufio"; "strings"; "strconv")

type point struct {
	pos_x, pos_y int
	vel_x, vel_y int
}
func (p *point) updatePosX() {
	p.pos_x += p.vel_x
}
func (p *point) updatePosY() {
	p.pos_y += p.vel_y
}
func (p *point) reverseUpdatePosX() {
	p.pos_x -= p.vel_x
}
func (p *point) reverseUpdatePosY() {
	p.pos_y -= p.vel_y
}


func getBounds(points []*point) (int, int, int, int) {
	max_x, max_y := -10000000000000, -10000000000000
	min_x, min_y := 10000000000000, 10000000000000
	for _, p := range points {
		if p.pos_x > max_x { max_x = p.pos_x }
		if p.pos_x < min_x { min_x = p.pos_x }
		if p.pos_y > max_y { max_y = p.pos_y }
		if p.pos_y < min_y { min_y = p.pos_y }
	}
	return max_x, max_y, min_x, min_y
}

func main() {
	//initialize file access for input
	inputPath := "star-align-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var points []*point
	for scanner.Scan() {
		pointInput := scanner.Text()
		firstPosIdx := strings.Index(pointInput, "<")
		secondPosIdx := strings.Index(pointInput, ">")
		pos_string := pointInput[firstPosIdx+1:secondPosIdx]
		pos := strings.Fields(pos_string)
		firstVelIdx := strings.LastIndex(pointInput, "<")
		secondVelIdx := strings.LastIndex(pointInput, ">")
		vel_string := pointInput[firstVelIdx+1:secondVelIdx]
		vel := strings.Fields(vel_string)
		px, _ := strconv.Atoi(strings.Trim(pos[0], ","))
		py, _ := strconv.Atoi(pos[1])
		vx, _ := strconv.Atoi(strings.Trim(vel[0], ","))
		vy, _ := strconv.Atoi(vel[1])
		newPoint := &point{pos_x: px, pos_y: py, vel_x: vx, vel_y: vy}
		points = append(points, newPoint)
	}

	max_x, max_y, min_x, min_y := getBounds(points)
	area := (max_x - min_x) * (max_y - min_y)
	time := 0
	for true {
		time += 1
		for _, p := range points {
			p.updatePosX()
			p.updatePosY()
		}
		max_x, max_y, min_x, min_y = getBounds(points)
		newArea := (max_x - min_x) * (max_y - min_y)
		if newArea >= area { break }
		area = newArea
	}

	fmt.Println(time - 1)
}