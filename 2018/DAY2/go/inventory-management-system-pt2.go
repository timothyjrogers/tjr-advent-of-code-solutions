package main

import ("fmt"; "os"; "bufio")

//Assumes Strings are same length
func strDiffOne(s1 string, s2 string) bool {
	diffs := 0
	for i, val := range s1 {
		if val != rune(s2[i]) {
			diffs = diffs + 1
		}
	}
	if diffs == 1 {
		return true
	}
	return false
}

//Assumes Strings have exactly one different char in same spot
func indexOfSingleDiff(s1 string, s2 string) int {
	for i, val := range s1 {
		if val != rune(s2[i]) {
			return i
		}
	}
	return -1
}

func main() {
	inputPath := "../inventory-management-system-input.txt"
	file, _ := os.Open(inputPath)
	defer file.Close()

	var ids []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		ids = append(ids, scanner.Text())
	}
	for i, v1 := range ids {
		for _, v2 := range ids[i+1:] {
			if strDiffOne(v1, v2) {
				fmt.Println(v1)
				fmt.Println(v2)
				diffIdx := indexOfSingleDiff(v1, v2)
				fmt.Println(v1[:diffIdx], v1[diffIdx+1:])
			}
		}
	}
}