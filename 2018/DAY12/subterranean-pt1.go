package main

import ("fmt"; "os"; "bufio"; "strings")

func getMin(data map[int]bool) int {
	min := 100000000000
	for i, _ := range data { if i < min { min = i } }
	return min
}

func getMax(data map[int]bool) int {
	max := -100000000000
	for i, _ := range data { if i > max { max = i } }
	return max
}

func generation(rules map[string]bool, curGen map[int]bool) map[int]bool {
	nextGenPlants := make(map[int]bool)
	// +/- 3 accounts for leading/tailing pots that could change
	min, max := getMin(curGen)-3, getMax(curGen)+3
	for i := min; i <= max; i++ {
		pots := ""
		for j := -2; j <= 2; j++ {
			if _, ok := curGen[i+j]; ok {
				pots += "#"
			} else {
				pots += "."
			}
		}
		if _, ok := rules[pots]; ok {
			nextGenPlants[i] = true
		}
	}
	return nextGenPlants
}

func main() {
	//initialize file access for input
	inputPath := "subterranean-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var init string
	rules := make(map[string]bool)

	scanner.Scan()
	init = strings.Fields(scanner.Text())[2]
	scanner.Scan() //skip blank line
	for scanner.Scan() {
		rule := strings.Fields(scanner.Text())
		if rule[2] == "#" { rules[rule[0]] = true }
	}
	
	plants := make(map[int]bool)
	for pos, char := range init { if char == '#' { plants[pos] = true } }

	for i := 0; i < 20; i++ { plants = generation(rules, plants) }

	ans := 0
	for i, _ := range plants { ans += i }
	fmt.Println(ans)
}