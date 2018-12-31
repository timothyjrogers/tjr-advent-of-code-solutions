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

func main() {
	//initialize file access for input
	inputPath := "star-align-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var points []point
	for scanner.Scan() {
		pointInput := strings.Fields(scanner.Text())
		newPoint := point{pos_x: pointInput[1].trim(","), pos_y: pointInput[2], vel_x: pointInput[4].trim(","), vel_y: pointInput[5]}
		points = append(points, newPoint)
	}
}