package main

import ("fmt"; "os"; "bufio")

func mapLetters(s string) map[rune]int {
	charMap := make(map[rune]int)
	for _, c := range s {
		if _, ok := charMap[c]; ok {
			charMap[c] = charMap[c] + 1
		} else {
			charMap[c] = 1
		}
	}
	return charMap
}

func twoOrThree(m map[rune]int) (bool, bool) {
	two := false
	three := false
	for _, val := range m {
		if val == 2 {
			two = true
		}
		if val == 3 {
			three = true
		}
		if two && three {
			break
		}
	}
	return two, three
}

func main() {
	inputPath := "../inventory-management-system-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	twos := 0
	threes := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		m := mapLetters(scanner.Text())
		mTwos, mThrees := twoOrThree(m)
		if mTwos {
			twos = twos + 1
		}
		if mThrees {
			threes = threes + 1
		}
	}
	fmt.Println(twos * threes)
}