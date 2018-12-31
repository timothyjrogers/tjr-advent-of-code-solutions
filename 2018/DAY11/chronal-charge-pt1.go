package main

import ("fmt")

func calcCellPower(x int, y int, serial int) int {
	power := ((((x + 10) * y) + serial) * (x + 10))
	if power < 100 { return 0 }
	power = power % 1000
	return (power / 100 -5)
}

func main() {
	//initialize file access for input
	input := 7139
	maxPower := -100000000000
	maxPowerAns := 0
	max_x := 0
	max_y := 0
	max_m := 0

	for m := 0; m <= 299; m++ {
		for i := 1; i <= 300 - m; i++ {
			for j := 1; j <= 300 - m; j++ {
				power := 0
				for k := 0; k < m; k++ {
					for l := 0; l < m; l++ {
						power += calcCellPower(i+k, j+l, input)
					}
				}
				if power > maxPower {
					maxPower = power
					maxPowerAns = calcCellPower(i, j, input)
					max_x = i
					max_y = j
					max_m = m
				}
			}
		}
	}

	fmt.Println(maxPower, maxPowerAns, max_x, max_y, max_m)
}