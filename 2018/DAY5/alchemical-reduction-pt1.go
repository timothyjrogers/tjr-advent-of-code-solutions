package main

import ("fmt"; "os"; "bufio"; "unicode")

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
	
	ans := reactPolymer(polymer)	
	fmt.Println(ans, len(ans))

}