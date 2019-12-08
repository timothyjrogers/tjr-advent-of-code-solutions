package main

import ("fmt"; "os"; "bufio"; "unicode"; "strings")

func compareRune(c1 rune, c2 rune) bool {
	if unicode.ToLower(c1) != unicode.ToLower(c2) {
		return false
	}
	if unicode.IsLower(c1) && unicode.IsUpper(c2) {
		return true
	}
	if unicode.IsUpper(c1) && unicode.IsLower(c2) {
		return true
	}
	return false
}

func reactPolymer(polymer string) string {
	complete := false
	for !complete {
		change := false
		for i := 0; i < len(polymer) - 1; i++ {
			if compareRune(rune(polymer[i]), rune(polymer[i+1])) {
				polymerRunes := []rune(polymer)
				polymerRunes = append(polymerRunes[:i], polymerRunes[i+2:]...)
				polymer = string(polymerRunes)
				change = true
				break
			}
			
		}
		complete = !change
	}
	return polymer
}

func main() {
	inputPath := "alchemical-reduction-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	polymer := scanner.Text()
	
	alpha := [26]string{ "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z" }

	shortestPolymer := 100000000000000
	for _, a := range alpha {
		adjustedPolymer := strings.Replace(polymer, a, "", -1)
		adjustedPolymer = strings.Replace(adjustedPolymer, string(unicode.ToUpper(rune(a[0]))), "", -1)
		adjustedReaction := reactPolymer(adjustedPolymer)
		lenAdjustedPolymer := len(adjustedReaction)
		if lenAdjustedPolymer < shortestPolymer {
			shortestPolymer = lenAdjustedPolymer
		}
	}
	fmt.Println(shortestPolymer)
}